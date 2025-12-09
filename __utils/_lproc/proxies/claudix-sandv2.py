#!/usr/bin/env python3

"""
Proxy: claudix-sandv2

Runs `claude` with the same defaults as claudix4 but inside the bubblewrap-based
sandbox (`sand --in-docker`). Supports restart control via text-based commands
embedded in user messages. Accepts sandbox config as first argument (defaults to 'sand').

Usage:
  [proxies/claudix-sandv2.py]              # Uses default 'sand' config
  [proxies/claudix-sandv2.py::sand1]       # Uses 'sand1' config
  [proxies/claudix-sandv2.py::myconfig]    # Uses 'myconfig' config

Behavior
- Launches `sand --config <config> --in-docker -- claude ...` using the local sand utility.
- Mirrors child stdout to stdout unchanged and stderr to stderr.
- Tracks pending work: increments expected count for each valid JSON user message
  sent to claude, and increments seen count for each `{ "type": "result" }`
  observed on stdout.
- Tracks session_id: automatically captures and updates the latest session_id from
  any stdout message containing a "session_id" field. Used for --resume functionality.
- Mirrors the first line of assistant text responses to stderr (with ` ...` if
  multi-line) for quick monitoring.

Restart Commands (embedded in user message text):
1. [CLAUDIX:RESTART] - Wait for all pending results, restart claude, send modified
   message (with command removed) to new instance.

2. [CLAUDIX:FORCE_RESTART] - Kill claude immediately without waiting. Inject
   synthetic failure results for any missing results. Restart claude and send
   modified message (with command removed) to new instance.

3. [CLAUDIX:FORCE_RESTART_NO_SEND] - Kill claude immediately. Inject failure
   results for missing results, plus an additional reminder message explaining
   the previous message was NOT sent. Restart claude and wait for new input
   (current message is discarded).

4. [CLAUDIX:FORCE_RESTART_RESUME] - Same as FORCE_RESTART but restarts with
   --resume flag using the latest tracked session_id. Preserves conversation
   context across restart.

5. [CLAUDIX:FORCE_RESTART_RESUME_NO_SEND] - Same as FORCE_RESTART_NO_SEND but
   restarts with --resume flag. The message is not sent but session is resumed.

Injected failure message formats:
  For FORCE_RESTART and FORCE_RESTART_NO_SEND (no session resume):
    {"type":"result", "subtype":"CLAUDIX_FAIL", "is_error":true,
     "result":"Claude Code Failure. Will Restart. The Agent will lose its memory.
     Please provide necessary context and instructions in the next message."}

  For FORCE_RESTART_RESUME and FORCE_RESTART_RESUME_NO_SEND (with session resume):
    {"type":"result", "subtype":"CLAUDIX_FAIL", "is_error":true,
     "result":"Claude Code Force Restart. The corresponding input message might not
     be fully processed. The session will be resumed with previous context preserved."}

Notes
- Exits with the child return code; warns if writing to stdin fails.
- Requires the `sand` CLI from PATH.
"""

import json
import queue
import shlex
import sys
import time
import threading
import subprocess
import re
import copy
from pathlib import Path
from typing import Any, List, Optional, Tuple
from shutil import which
import os


CLAUDE_DEFAULT = shlex.split(
    "claude -p --dangerously-skip-permissions --model claude-sonnet-4-5-20250929 "
    "--output-format stream-json --input-format stream-json --verbose "
    "--replay-user-messages"
)



def build_sandboxed_command(sandbox_config: str, extra_args: List[str]) -> List[str]:
    """Build sandboxed claude command with dynamic config."""
    sand_cli = which("sand")
    if not sand_cli:
        raise FileNotFoundError("sand executable not found; expected `sand` in PATH")

    return [
        "sand",
        "--config", sandbox_config,
        "--in-docker",
        "--",
        *CLAUDE_DEFAULT,
        *extra_args,
    ]


def extract_claudix_command(obj: Any) -> Tuple[Optional[str], Optional[Any]]:
    """
    Extract [CLAUDIX:*] command from user message text content.

    Returns:
        (command_type, modified_obj) tuple where:
        - command_type: None, "RESTART", "FORCE_RESTART", or "FORCE_RESTART_NO_SEND"
        - modified_obj:
            - For RESTART/FORCE_RESTART: obj with [CLAUDIX:*] pattern removed
            - For FORCE_RESTART_NO_SEND: None (message not sent)
            - For no command: original obj unchanged
    """
    if not isinstance(obj, dict):
        return None, obj

    if obj.get("type") != "user":
        return None, obj

    message = obj.get("message", {})
    if not isinstance(message, dict) or message.get("role") != "user":
        return None, obj

    contents = message.get("content", [])
    if not isinstance(contents, list):
        return None, obj

    # Search for [CLAUDIX:*] pattern in text items
    pattern = r'\[CLAUDIX:(RESTART|FORCE_RESTART|FORCE_RESTART_NO_SEND|FORCE_RESTART_RESUME|FORCE_RESTART_RESUME_NO_SEND)\]'

    for item in contents:
        if isinstance(item, dict) and item.get("type") == "text":
            text = item.get("text", "")
            match = re.search(pattern, text)
            if match:
                command = match.group(1)

                if command == "FORCE_RESTART_NO_SEND" or command == "FORCE_RESTART_RESUME_NO_SEND":
                    # Don't send this message at all
                    return command, None

                else:  # RESTART, FORCE_RESTART, or FORCE_RESTART_RESUME
                    # Remove the pattern and return modified message
                    new_text = re.sub(pattern, '', text).strip()

                    # Deep copy and modify
                    modified_obj = copy.deepcopy(obj)

                    # Find and update the text item
                    for mod_item in modified_obj["message"]["content"]:
                        if isinstance(mod_item, dict) and mod_item.get("type") == "text":
                            if re.search(pattern, mod_item.get("text", "")):
                                mod_item["text"] = new_text
                                break

                    return command, modified_obj

    return None, obj


class ClaudeRunner:
    def __init__(self, args: List[str]):
        self.args = list(args)
        self.proc: Optional[subprocess.Popen] = None
        self.t_out: Optional[threading.Thread] = None
        self.t_err: Optional[threading.Thread] = None
        self._lock = threading.RLock()
        self._results_seen = 0
        self._results_expected = 0
        self._latest_session_id: Optional[str] = None

    @property
    def results_seen(self) -> int:
        with self._lock:
            return self._results_seen

    @property
    def results_expected(self) -> int:
        with self._lock:
            return self._results_expected

    @property
    def latest_session_id(self) -> Optional[str]:
        with self._lock:
            return self._latest_session_id

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
            except Exception as e:
                obj = None
                # Warn about unparseable messages (but still forwarded to stdout above)
                stripped = line.strip()
                if stripped:  # Only warn for non-empty lines
                    try:
                        sys.stderr.write(f"[claudix-sandv2] Warning: Unparseable JSON from Claude Code: {stripped[:100]}\n")
                        sys.stderr.flush()
                    except Exception:
                        pass

            # Warn if valid JSON but not a dict (cannot be processed)
            if obj is not None and not isinstance(obj, dict):
                try:
                    sys.stderr.write(f"[claudix-sandv2] Warning: Non-dict JSON from Claude Code (type={type(obj).__name__}): {str(obj)[:100]}\n")
                    sys.stderr.flush()
                except Exception:
                    pass

            if isinstance(obj, dict):
                # Track session_id if present
                if "session_id" in obj:
                    with self._lock:
                        self._latest_session_id = obj["session_id"]

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
            print(f"[claudix-sandv2] started PID {self.proc.pid}: {cmd_str}", file=sys.stderr)
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
                print(f"[claudix-sandv2] restarted; new PID {self.proc.pid}: {cmd_str}", file=sys.stderr)
            except Exception:
                pass

    def restart_with_resume(self) -> None:
        """Restart with --resume flag using latest session_id."""
        with self._lock:
            self.stop()

            # Build new args with --resume if we have a session_id
            if self._latest_session_id:
                # Remove any existing --resume flags first
                new_args = []
                skip_next = False
                for arg in self.args:
                    if skip_next:
                        skip_next = False
                        continue
                    if arg == "--resume":
                        skip_next = True
                        continue
                    new_args.append(arg)

                # Add --resume with latest session_id
                new_args.extend(["--resume", self._latest_session_id])

                # Temporarily update args for this restart
                old_args = self.args
                self.args = new_args

                try:
                    self._start_locked()
                    cmd_str = " ".join(shlex.quote(a) for a in self.args)
                    print(f"[claudix-sandv2] restarted with --resume {self._latest_session_id}; new PID {self.proc.pid}: {cmd_str}", file=sys.stderr)
                except Exception as e:
                    # Restore old args on failure
                    self.args = old_args
                    print(f"[claudix-sandv2] Warning: failed to restart with resume: {e}", file=sys.stderr)
                    raise
            else:
                # No session_id yet, do normal restart
                print("[claudix-sandv2] Warning: No session_id tracked yet, restarting without resume", file=sys.stderr)
                self._start_locked()
                try:
                    cmd_str = " ".join(shlex.quote(a) for a in self.args)
                    print(f"[claudix-sandv2] restarted; new PID {self.proc.pid}: {cmd_str}", file=sys.stderr)
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

    # Parse arguments: first arg is sandbox config, rest are claude args
    sandbox_config = "sand"  # default
    claude_args = argv

    # If first arg doesn't start with -, treat it as sandbox config
    if argv and not argv[0].startswith('-'):
        sandbox_config = argv[0]
        claude_args = argv[1:]

    try:
        cmd_args = build_sandboxed_command(sandbox_config, claude_args)
    except FileNotFoundError as exc:
        print(f"Error: {exc}", file=sys.stderr)
        return 1

    try:
        runner = ClaudeRunner(cmd_args)
        runner.start()
    except FileNotFoundError as e:
        print(f"Error: FileNotFoundError inside sandbox: {e}", file=sys.stderr)
        return 127
    except Exception as e:
        print(f"Error launching claude via sand: {e}", file=sys.stderr)
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

    try:
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

            # Parse the line and check for CLAUDIX commands
            parsed_obj: Optional[Any] = None
            try:
                obj = json.loads(line)
                parsed_obj = obj
            except Exception as e:
                parsed_obj = None
                # Warn about malformed input (will still be forwarded to Claude Code)
                if line.strip():  # Only warn for non-empty lines
                    print(f"[claudix-sandv2] Warning: Malformed JSON received on stdin (forwarding to Claude): {line[:100]}", file=sys.stderr)

            # Extract CLAUDIX command if present
            command, modified_obj = extract_claudix_command(parsed_obj)

            # --- Handle Normal Messages (no command) ---
            if command is None:
                if is_user_message_obj(parsed_obj):
                    runner.inc_expected()
                ok = runner.write(line + "\n")
                if not ok:
                    child_rc = runner.poll() or 1
                    print("[claudix-sandv2] Child process closed stdin; exiting.", file=sys.stderr)
                    break
                continue

            # --- Handle [CLAUDIX:RESTART] ---
            if command == "RESTART":
                expected_at_trigger = runner.results_expected
                waited = 0
                while runner.results_seen < expected_at_trigger:
                    waited += 2
                    print(f"[claudix-sandv2] RESTART requested; waiting for result... {waited}s", file=sys.stderr)
                    try:
                        time.sleep(2)
                    except KeyboardInterrupt:
                        pass

                print("[claudix-sandv2] All results received; restarting claude...", file=sys.stderr)
                runner.restart()

                # Send modified message (with [CLAUDIX:RESTART] removed)
                modified_line = json.dumps(modified_obj, ensure_ascii=False)
                if is_user_message_obj(modified_obj):
                    runner.inc_expected()
                    print("[claudix-sandv2] Sending modified message to new instance (counted as user message)", file=sys.stderr)

                ok = runner.write(modified_line + "\n")
                if not ok:
                    child_rc = runner.poll() or 1
                    print("[claudix-sandv2] Warning: failed to write to restarted process stdin.", file=sys.stderr)
                    break
                continue

            # --- Handle [CLAUDIX:FORCE_RESTART] ---
            if command == "FORCE_RESTART":
                # Calculate missing results BEFORE stopping
                missing = runner.results_expected - runner.results_seen

                print(f"[claudix-sandv2] FORCE_RESTART requested; killing claude immediately (missing {missing} results)...", file=sys.stderr)
                runner.stop()

                # Inject failure messages for each missing result
                if missing > 0:
                    failure_msg = {
                        "type": "result",
                        "subtype": "CLAUDIX_FAIL",
                        "is_error": True,
                        "result": "Claude Code Failure. Will Restart. The Agent will lose its memory. Please provide necessary context and instructions in the next message."
                    }
                    for i in range(missing):
                        try:
                            sys.stdout.write(json.dumps(failure_msg, ensure_ascii=False) + "\n")
                            sys.stdout.flush()
                        except Exception as e:
                            print(f"[claudix-sandv2] Warning: failed to inject failure message: {e}", file=sys.stderr)

                    # Balance counters
                    with runner._lock:
                        runner._results_seen = runner._results_expected
                    print(f"[claudix-sandv2] Injected {missing} failure messages and balanced counters", file=sys.stderr)

                # Restart
                print("[claudix-sandv2] Restarting claude...", file=sys.stderr)
                runner.restart()

                # Send modified message (with [CLAUDIX:FORCE_RESTART] removed)
                modified_line = json.dumps(modified_obj, ensure_ascii=False)
                if is_user_message_obj(modified_obj):
                    runner.inc_expected()
                    print("[claudix-sandv2] Sending modified message to new instance (counted as user message)", file=sys.stderr)

                ok = runner.write(modified_line + "\n")
                if not ok:
                    child_rc = runner.poll() or 1
                    print("[claudix-sandv2] Warning: failed to write to restarted process stdin.", file=sys.stderr)
                    break
                continue

            # --- Handle [CLAUDIX:FORCE_RESTART_NO_SEND] ---
            if command == "FORCE_RESTART_NO_SEND":
                # Calculate missing results BEFORE stopping
                missing = runner.results_expected - runner.results_seen

                print(f"[claudix-sandv2] FORCE_RESTART_NO_SEND requested; killing claude immediately (missing {missing} results)...", file=sys.stderr)
                runner.stop()

                # Inject failure messages for each missing result
                if missing > 0:
                    failure_msg = {
                        "type": "result",
                        "subtype": "CLAUDIX_FAIL",
                        "is_error": True,
                        "result": "Claude Code Failure. Will Restart. The Agent will lose its memory. Please provide necessary context and instructions in the next message."
                    }
                    for i in range(missing):
                        try:
                            sys.stdout.write(json.dumps(failure_msg, ensure_ascii=False) + "\n")
                            sys.stdout.flush()
                        except Exception as e:
                            print(f"[claudix-sandv2] Warning: failed to inject failure message: {e}", file=sys.stderr)

                    # Balance counters
                    with runner._lock:
                        runner._results_seen = runner._results_expected
                    print(f"[claudix-sandv2] Injected {missing} failure messages and balanced counters", file=sys.stderr)

                # Inject additional human-readable reminder
                reminder_msg = {
                    "type": "result",
                    "subtype": "CLAUDIX_FAIL",
                    "is_error": True,
                    "result": "Claude Code was force-restarted. The previous message was NOT sent to the new instance. Please resend your request with necessary context and instructions."
                }
                try:
                    sys.stdout.write(json.dumps(reminder_msg, ensure_ascii=False) + "\n")
                    sys.stdout.flush()
                    print("[claudix-sandv2] Injected reminder message about NO_SEND", file=sys.stderr)
                except Exception as e:
                    print(f"[claudix-sandv2] Warning: failed to inject reminder message: {e}", file=sys.stderr)

                # Update counter for the extra message
                with runner._lock:
                    runner._results_seen += 1

                # Restart
                print("[claudix-sandv2] Restarting claude and waiting for new input...", file=sys.stderr)
                runner.restart()

                # DO NOT send any message - just continue to next stdin input
                continue

            # --- Handle [CLAUDIX:FORCE_RESTART_RESUME] ---
            if command == "FORCE_RESTART_RESUME":
                # Calculate missing results BEFORE stopping
                missing = runner.results_expected - runner.results_seen

                print(f"[claudix-sandv2] FORCE_RESTART_RESUME requested; killing and resuming (missing {missing} results)...", file=sys.stderr)
                runner.stop()

                # Inject failure messages for each missing result
                if missing > 0:
                    failure_msg = {
                        "type": "result",
                        "subtype": "CLAUDIX_FAIL",
                        "is_error": True,
                        "result": "Claude Code Force Restart. The corresponding input message might not be fully processed. The session will be resumed with previous context preserved."
                    }
                    for i in range(missing):
                        try:
                            sys.stdout.write(json.dumps(failure_msg, ensure_ascii=False) + "\n")
                            sys.stdout.flush()
                        except Exception as e:
                            print(f"[claudix-sandv2] Warning: failed to inject failure message: {e}", file=sys.stderr)

                    # Balance counters
                    with runner._lock:
                        runner._results_seen = runner._results_expected
                    print(f"[claudix-sandv2] Injected {missing} failure messages and balanced counters", file=sys.stderr)

                # Restart with resume
                print("[claudix-sandv2] Restarting claude with --resume...", file=sys.stderr)
                runner.restart_with_resume()

                # Send modified message (with [CLAUDIX:FORCE_RESTART_RESUME] removed)
                modified_line = json.dumps(modified_obj, ensure_ascii=False)
                if is_user_message_obj(modified_obj):
                    runner.inc_expected()
                    print("[claudix-sandv2] Sending modified message to new instance (counted as user message)", file=sys.stderr)

                ok = runner.write(modified_line + "\n")
                if not ok:
                    child_rc = runner.poll() or 1
                    print("[claudix-sandv2] Warning: failed to write to restarted process stdin.", file=sys.stderr)
                    break
                continue

            # --- Handle [CLAUDIX:FORCE_RESTART_RESUME_NO_SEND] ---
            if command == "FORCE_RESTART_RESUME_NO_SEND":
                # Calculate missing results BEFORE stopping
                missing = runner.results_expected - runner.results_seen

                print(f"[claudix-sandv2] FORCE_RESTART_RESUME_NO_SEND requested; killing and resuming (missing {missing} results)...", file=sys.stderr)
                runner.stop()

                # Inject failure messages for each missing result
                if missing > 0:
                    failure_msg = {
                        "type": "result",
                        "subtype": "CLAUDIX_FAIL",
                        "is_error": True,
                        "result": "Claude Code Force Restart. The corresponding input message might not be fully processed. The session will be resumed with previous context preserved."
                    }
                    for i in range(missing):
                        try:
                            sys.stdout.write(json.dumps(failure_msg, ensure_ascii=False) + "\n")
                            sys.stdout.flush()
                        except Exception as e:
                            print(f"[claudix-sandv2] Warning: failed to inject failure message: {e}", file=sys.stderr)

                    # Balance counters
                    with runner._lock:
                        runner._results_seen = runner._results_expected
                    print(f"[claudix-sandv2] Injected {missing} failure messages and balanced counters", file=sys.stderr)

                # Inject additional human-readable reminder
                reminder_msg = {
                    "type": "result",
                    "subtype": "CLAUDIX_FAIL",
                    "is_error": True,
                    "result": "Claude Code was force-restarted with resume. The previous message was NOT sent to the resumed instance. Please resend your request with necessary context and instructions."
                }
                try:
                    sys.stdout.write(json.dumps(reminder_msg, ensure_ascii=False) + "\n")
                    sys.stdout.flush()
                    print("[claudix-sandv2] Injected reminder message about NO_SEND with resume", file=sys.stderr)
                except Exception as e:
                    print(f"[claudix-sandv2] Warning: failed to inject reminder message: {e}", file=sys.stderr)

                # Update counter for the extra message
                with runner._lock:
                    runner._results_seen += 1

                # Restart with resume
                print("[claudix-sandv2] Restarting claude with --resume and waiting for new input...", file=sys.stderr)
                runner.restart_with_resume()

                # DO NOT send any message - just continue to next stdin input
                continue

    finally:
        # Ensure the child is stopped even if restart fails or exception is raised
        runner.stop()

    # Propagate child's exit code if available
    rc = runner.poll()
    if child_rc is None:
        child_rc = rc if rc is not None else 0
    return child_rc


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

