#!/usr/bin/env python3

"""
Proxy: claudix3

Runs the `claude` CLI with the same defaults as claudix2, but additionally
parses the child's STDOUT stream (JSONL) to extract assistant message text
and mirrors that text to STDERR, without altering the original STDOUT stream.

Also forwards the child's STDERR directly to our STDERR.

Usage with lproc:
  ./lproc.py -s myproc "[proxies/claudix3.py] --help"

Any extra args after the bracket are appended to the claude invocation.
"""

import os
import sys
import shlex
import json
import threading
import subprocess
from typing import Any, Dict, List


BASE_CLAUDE = (
    "claude -p --dangerously-skip-permissions --model sonnet "
    "--output-format stream-json --input-format stream-json --verbose"
)


def extract_assistant_text(line: str) -> str:
    """Return concatenated assistant text content from a JSON line if present; else ''."""
    try:
        obj = json.loads(line)
    except Exception:
        return ""
    if obj.get("type") != "assistant":
        return ""
    msg = obj.get("message", {})
    contents: List[Dict[str, Any]] = msg.get("content", []) or []
    text_parts: List[str] = []
    for item in contents:
        if isinstance(item, dict) and item.get("type") == "text":
            text = item.get("text", "")
            if text:
                text_parts.append(str(text))
    return "\n".join(text_parts).strip()


def main(argv: List[str]) -> int:
    # Ensure our own stdio is as flushy as possible
    try:
        sys.stdout.reconfigure(line_buffering=True)
    except Exception:
        pass
    try:
        sys.stderr.reconfigure(line_buffering=True)
    except Exception:
        pass

    args = shlex.split(BASE_CLAUDE)
    args.extend(argv)

    # Spawn claude, inheriting stdin, capturing stdout/stderr
    try:
        proc = subprocess.Popen(
            args,
            stdin=None,  # inherit
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            bufsize=1,  # line buffered in text mode
            text=True,
        )
    except FileNotFoundError:
        print("Error: 'claude' command not found in PATH.", file=sys.stderr)
        return 127
    except Exception as e:
        print(f"Error launching claude: {e}", file=sys.stderr)
        return 1

    def pump_stdout():
        assert proc.stdout is not None
        for line in proc.stdout:
            # Mirror original stdout unchanged
            try:
                sys.stdout.write(line)
                sys.stdout.flush()
            except Exception:
                pass

            # Extract assistant text and send to stderr
            text = extract_assistant_text(line.strip())
            if text:
                try:
                    sys.stderr.write(text + "\n")
                    sys.stderr.flush()
                except Exception:
                    pass

    def pump_stderr():
        assert proc.stderr is not None
        for line in proc.stderr:
            try:
                sys.stderr.write(line)
                sys.stderr.flush()
            except Exception:
                pass

    t_out = threading.Thread(target=pump_stdout, daemon=True)
    t_err = threading.Thread(target=pump_stderr, daemon=True)
    t_out.start()
    t_err.start()

    # Wait for process to finish, then wait for pumps
    rc = proc.wait()
    t_out.join(timeout=1.0)
    t_err.join(timeout=1.0)
    return int(rc)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

