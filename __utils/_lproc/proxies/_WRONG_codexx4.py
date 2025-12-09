#!/usr/bin/env python3

"""
Proxy: codexx4

Runs `codex -m gpt5 --dangerously-bypass-approvals-and-sandbox` (plus any extras),
forwards child stdout/stderr unchanged, and adds robust restart control identical
to claudix4, including stderr previews of assistant text.

Behavior
- Mirrors child's stdout to our stdout (unchanged) and stderr to our stderr.
- Tracks pending work: increments expected count for each valid JSON user message
  of the form {"type":"user","message":{"role":"user",...}} sent to codex,
  and increments seen count for each {"type":"result"} observed on stdout.
- On input containing {"_CLAUDIX_":"RESTART", ...}:
  - Snapshot expected-at-trigger and poll every 2s (logging to stderr) until
    results_seen >= expected_at_trigger.
  - Restart `codex` with the same args, logging PID and command to stderr.
  - Remove the "_CLAUDIX_" field and send the modified JSON to the new process
    (and count it if it is a user message).

Logging
- On first start and after each restart, logs to stderr: PID and full command.
- Wait progress logs to stderr every 2 seconds.

Usage (within lproc)
  ./lproc.py -s myproc "[proxies/codexx4.py] <extra args>"

Notes
- Streams are line-buffered; stdout is not altered.
- Exits with the child return code; warns if writing to stdin fails.
"""

import sys
import shlex
import json
import time
import threading
import subprocess
from typing import Any, List, Optional


BASE_CODEX = (
    "codex -m gpt5 --dangerously-bypass-approvals-and-sandbox "
)


class CodexRunner:
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

    def _pump_stdout(self, p: subprocess.Popen) -> None:
        assert p.stdout is not None
        for line in p.stdout:
            # Forward original stdout unchanged
            try:
                sys.stdout.write(line)
                sys.stdout.flush()
            except Exception:
                pass

            # Tally result events for restart coordination and preview assistant
            try:
                obj = json.loads(line.strip())
                if isinstance(obj, dict):
                    # Count results
                    if obj.get("type") == "result":
                        with self._lock:
                            self._results_seen += 1
                    # Pretty-print assistant text first line to stderr
                    if obj.get("type") == "assistant":
                        msg = obj.get("message", {})
                        contents = msg.get("content", []) or []
                        text_parts = []
                        if isinstance(contents, list):
                            for it in contents:
                                if isinstance(it, dict) and it.get("type") == "text":
                                    t = it.get("text", "")
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
        # Spawn child, inherit stdin for streaming
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
            print(f"[codexx4] started PID {self.proc.pid}: {cmd_str}", file=sys.stderr)
        except Exception:
            pass

    def stop(self, timeout: float = 3.0) -> None:
        with self._lock:
            p = self.proc
            if not p:
                return
            try:
                # Close stdin to signal EOF (best-effort)
                try:
                    if p.stdin:
                        p.stdin.close()
                except Exception:
                    pass
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
            self.proc = None

    def restart(self) -> None:
        with self._lock:
            self.stop()
            self._start_locked()
            try:
                cmd_str = " ".join(shlex.quote(a) for a in self.args)
                print(f"[codexx4] restarted; new PID {self.proc.pid}: {cmd_str}", file=sys.stderr)
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


def main(argv: List[str]) -> int:
    # Make our own streams line-buffered for responsiveness
    try:
        sys.stdout.reconfigure(line_buffering=True)
    except Exception:
        pass
    try:
        sys.stderr.reconfigure(line_buffering=True)
    except Exception:
        pass

    args = shlex.split(BASE_CODEX)
    args.extend(argv)

    # Start child
    try:
        runner = CodexRunner(args)
        runner.start()
    except FileNotFoundError:
        print("Error: 'codex' command not found in PATH.", file=sys.stderr)
        return 127
    except Exception as e:
        print(f"Error launching codex: {e}", file=sys.stderr)
        return 1

    # Helper: determine if a JSON object is a user message we expect a result for
    def is_user_message_obj(obj: Any) -> bool:
        if not isinstance(obj, dict):
            return False
        if obj.get("type") != "user":
            return False
        msg = obj.get("message")
        if not isinstance(msg, dict):
            return False
        return msg.get("role") == "user"

    # Read controller input and forward to child, handling restart marker
    for raw in sys.stdin:
        if raw is None:
            continue
        line = raw.rstrip("\n")
        if not line:
            # Forward blank lines as-is
            runner.write("\n")
            continue
        # Check for restart control
        need_restart = False
        to_send = line
        parsed_obj: Optional[Any] = None
        try:
            obj = json.loads(line)
            parsed_obj = obj
            if isinstance(obj, dict) and obj.get("_CLAUDIX_") == "RESTART":
                need_restart = True
                # Remove control key before re-sending to new process
                try:
                    del obj["_CLAUDIX_"]
                except Exception:
                    pass
                to_send = json.dumps(obj, ensure_ascii=False)
        except Exception:
            pass

        if not need_restart:
            # Count expected result for user messages
            try:
                if parsed_obj is None:
                    parsed_obj = json.loads(to_send)
            except Exception:
                parsed_obj = None
            if is_user_message_obj(parsed_obj):
                runner.inc_expected()
            runner.write(to_send + "\n")
            continue

        # Snapshot how many results are expected so far (before the restart-triggering message)
        expected_at_trigger = runner.results_expected
        waited = 0
        # Wait in 2s increments until all expected results have been seen
        while runner.results_seen < expected_at_trigger:
            waited += 2
            print(f"[codexx4] RESTART requested; waiting for result... {waited}s", file=sys.stderr)
            try:
                time.sleep(2)
            except KeyboardInterrupt:
                pass

        print("[codexx4] Result observed; restarting codex...", file=sys.stderr)
        runner.restart()
        # Send the modified message to the new process
        # Count expected result if this is a user message being sent to the new process
        try:
            obj2 = json.loads(to_send)
        except Exception:
            obj2 = None
        if is_user_message_obj(obj2):
            runner.inc_expected()
        ok = runner.write(to_send + "\n")
        if not ok:
            print("[codexx4] Warning: failed to write to restarted process stdin.", file=sys.stderr)

    # If stdin closes, keep process alive as long as child runs
    runner.stop()
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
