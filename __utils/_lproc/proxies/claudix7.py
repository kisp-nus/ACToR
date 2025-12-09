#!/usr/bin/env python3

"""
Proxy: claudix7

Runs `claude` with the same defaults as claudix5 but inside the bubblewrap-based
sandbox (`sand.py --in-docker`). The proxy keeps the restart coordination and
stderr previews from claudix5 while ensuring the model process is isolated by
`sand`.

Behavior
- Launches `sand.py --in-docker -- claude ...` using the local sand utility.
- Mirrors child stdout to stdout unchanged and stderr to stderr.
- Tracks pending work: increments expected count for each valid JSON user message
  sent to claude, and increments seen count for each `{ "type": "result" }`
  observed on stdout.
- On input containing `{ "_CLAUDIX_": "RESTART", ... }`, waits until all
  outstanding results have been observed, restarts claude via sand, removes the
  control field, and forwards the modified JSON to the new process.
- Mirrors the first line of assistant text responses to stderr (with ` ...` if
  multi-line) for quick monitoring.

Notes
- Exits with the child return code; warns if writing to stdin fails.
- Prefers the `sand` CLI from PATH and falls back to `../_sand/sand.py`.
"""
import os
import json
import queue
import shlex
import sys
import time
import threading
import subprocess
from pathlib import Path
from typing import Any, List, Optional
from shutil import which


CLAUDE_DEFAULT = shlex.split(
    "/utils/claude_cred_shim/claude-wrap -p --dangerously-skip-permissions --model sonnet "
    "--output-format stream-json --input-format stream-json --verbose"
)


def build_sandboxed_command(extra_args: List[str]) -> List[str]:
    sand_cli = which("sand")
    if sand_cli:
        launch = [sand_cli, "--in-docker", "--"]
    else:
        sand_path = Path(__file__).resolve().parents[2] / "_sand" / "sand.py"
        if not sand_path.exists():
            raise FileNotFoundError(f"sand executable not found; expected CLI in PATH or script at {sand_path}")
        launch = [sys.executable, str(sand_path), "--in-docker", "--"]

    return [
        *launch,
        *CLAUDE_DEFAULT,
        *extra_args,
    ]


class ClaudeRunner:
    def __init__(self, args: List[str]):
        self.args = list(args)
        self.proc: Optional[subprocess.Popen] = None
        self.t_out: Optional[threading.Thread] = None
        self.t_err: Optional[threading.Thread] = None
        self._lock = threading.RLock()
        self._results_seen = 0
        self._results_expected = 0

    @property
    def results_seen(self) -> int:
        with self._lock:
            return self._results_seen

    @property
    def results_expected(self) -> int:
        with self._lock:
            return self._results_expected

    def inc_expected(self, n: int = 1) -> None:
        with self._lock:
            self._results_expected += n

    def poll(self) -> Optional[int]:
        with self._lock:
            if not self.proc:
                return None
            return self.proc.poll()

    def _pump_stdout(self, p: subprocess.Popen) -> None:
        assert p.stdout is not None
        for line in p.stdout:
            try:
                sys.stdout.write(line)
                sys.stdout.flush()
            except Exception:
                pass

            try:
                obj = json.loads(line.strip())
            except Exception:
                obj = None

            if isinstance(obj, dict):
                if obj.get("type") == "result":
                    with self._lock:
                        self._results_seen += 1
                if obj.get("type") == "assistant":
                    msg = obj.get("message", {})
                    contents = msg.get("content", []) or []
                    text_parts = []
                    if isinstance(contents, list):
                        for item in contents:
                            if isinstance(item, dict) and item.get("type") == "text":
                                t = item.get("text", "")
                                if t:
                                    text_parts.append(str(t))
                    if text_parts:
                        body = "\n".join(text_parts).strip()
                        if body:
                            first = body.splitlines()[0]
                            multi = "\n" in body
                            try:
                                sys.stderr.write(first + (" ..." if multi else "") + "\n")
                                sys.stderr.flush()
                            except Exception:
                                pass

    def _pump_stderr(self, p: subprocess.Popen) -> None:
        assert p.stderr is not None
        for line in p.stderr:
            try:
                sys.stderr.write(line)
                sys.stderr.flush()
            except Exception:
                pass

    def start(self) -> None:
        with self._lock:
            self._start_locked()

    def _start_locked(self) -> None:
        self.proc = subprocess.Popen(
            self.args,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            bufsize=1,
            text=True,
        )
        self.t_out = threading.Thread(target=self._pump_stdout, args=(self.proc,), daemon=True)
        self.t_err = threading.Thread(target=self._pump_stderr, args=(self.proc,), daemon=True)
        self.t_out.start()
        self.t_err.start()
        try:
            cmd_str = " ".join(shlex.quote(a) for a in self.args)
            claude_cred_path = os.environ.get("CLAUDE_CRED_PATH", "None")
            print(f"[claudix7] started PID {self.proc.pid}: {cmd_str} CLAUDE_CRED_PATH={claude_cred_path}", file=sys.stderr)
        except Exception:
            pass

    def stop(self, timeout: float = 3.0) -> Optional[int]:
        with self._lock:
            p = self.proc
            if not p:
                return None
            try:
                if p.stdin:
                    p.stdin.close()
            except Exception:
                pass
            try:
                p.terminate()
            except Exception:
                pass
            try:
                p.wait(timeout=timeout)
            except Exception:
                try:
                    p.kill()
                except Exception:
                    pass
                try:
                    p.wait(timeout=1.0)
                except Exception:
                    pass
            rc = p.returncode
            self.proc = None
            return rc

    def restart(self) -> None:
        with self._lock:
            self.stop()
            self._start_locked()
            try:
                cmd_str = " ".join(shlex.quote(a) for a in self.args)
                print(f"[claudix7] restarted; new PID {self.proc.pid}: {cmd_str}", file=sys.stderr)
            except Exception:
                pass

    def write(self, data: str) -> bool:
        with self._lock:
            p = self.proc
            if not p or not p.stdin:
                return False
            try:
                p.stdin.write(data)
                p.stdin.flush()
                return True
            except Exception:
                return False


def is_user_message_obj(obj: Any) -> bool:
    if not isinstance(obj, dict):
        return False
    if obj.get("type") != "user":
        return False
    msg = obj.get("message")
    if not isinstance(msg, dict):
        return False
    return msg.get("role") == "user"


def main(argv: List[str]) -> int:
    try:
        sys.stdout.reconfigure(line_buffering=True)
    except Exception:
        pass
    try:
        sys.stderr.reconfigure(line_buffering=True)
    except Exception:
        pass

    try:
        cmd_args = build_sandboxed_command(argv)
    except FileNotFoundError as exc:
        print(f"Error: {exc}", file=sys.stderr)
        return 1

    try:
        runner = ClaudeRunner(cmd_args)
        runner.start()
    except FileNotFoundError:
        print("Error: 'claude-wrap' command not found inside sandbox.", file=sys.stderr)
        return 127
    except Exception as e:
        print(f"Error launching claude-wrap via sand: {e}", file=sys.stderr)
        return 1

    child_rc: Optional[int] = None

    # Use a background reader thread so we can poll the child process between lines.
    # This avoids the prior hang where bad JSON killed claude but the proxy stayed
    # blocked waiting for another stdin line before noticing the exit.
    line_queue: "queue.Queue[Optional[str]]" = queue.Queue()

    def stdin_reader() -> None:
        # Read lines in a thread so the main loop can monitor the child process without blocking.
        # Each line is forwarded via the queue; when stdin closes we send a sentinel.
        for raw in sys.stdin:
            line_queue.put(raw)
        # Signal end-of-input with a sentinel
        line_queue.put(None)

    t_reader = threading.Thread(target=stdin_reader, daemon=True)
    t_reader.start()

    while True:
        # Poll for child exit before waiting for more input so we can exit promptly on errors.
        polled = runner.poll()
        if polled is not None:
            child_rc = polled
            break

        try:
            raw = line_queue.get(timeout=0.2)
        except queue.Empty:
            # Loop again; poll() will run next iteration.
            continue

        if raw is None:
            break

        line = raw.rstrip("\n")
        if not line:
            runner.write("\n")
            continue

        need_restart = False
        to_send = line
        parsed_obj: Optional[Any] = None
        try:
            obj = json.loads(line)
            parsed_obj = obj
            if isinstance(obj, dict) and obj.get("_CLAUDIX_") == "RESTART":
                need_restart = True
                obj.pop("_CLAUDIX_", None)
                to_send = json.dumps(obj, ensure_ascii=False)
        except Exception:
            pass

        if not need_restart:
            if parsed_obj is None:
                try:
                    parsed_obj = json.loads(to_send)
                except Exception:
                    parsed_obj = None
            if is_user_message_obj(parsed_obj):
                runner.inc_expected()
            ok = runner.write(to_send + "\n")
            if not ok:
                # Child closed its stdin (commonly because it aborted on invalid JSON). Bail out instead of blocking forever.
                child_rc = runner.poll() or 1
                print("[claudix7] Child process closed stdin; exiting.", file=sys.stderr)
                break
            continue

        expected_at_trigger = runner.results_expected
        waited = 0
        while runner.results_seen < expected_at_trigger:
            waited += 2
            print(f"[claudix7] RESTART requested; waiting for result... {waited}s", file=sys.stderr)
            try:
                time.sleep(2)
            except KeyboardInterrupt:
                pass

        print("[claudix7] Result observed; restarting claude-wrap...", file=sys.stderr)
        runner.restart()
        try:
            obj2 = json.loads(to_send)
        except Exception:
            obj2 = None
        if is_user_message_obj(obj2):
            runner.inc_expected()
        ok = runner.write(to_send + "\n")
        if not ok:
            child_rc = runner.poll() or 1
            print("[claudix7] Warning: failed to write to restarted process stdin.", file=sys.stderr)
            break

    # Ensure the child is stopped and propagate its exit code if available.
    rc = runner.stop()
    if child_rc is None:
        child_rc = rc if rc is not None else 0
    return child_rc


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

