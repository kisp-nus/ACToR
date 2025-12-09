#!/usr/bin/env python3

"""
Proxy: claudix2

Replicates the behavior of the [claudix] template by launching the
`claude` CLI with sensible defaults, forwarding stdin/stdout/stderr,
and passing through any extra arguments.

Usage examples (within lproc):
  ./lproc.py -s myproc "[proxies/claudix2.py] --help"
  ./lproc.py -s myproc "[proxies/claudix2.py] -p --model opus"
"""

import os
import sys
import shlex


def main(argv):
    # Base args copied from lproc.yaml's `claudix` template
    base = "claude -p --dangerously-skip-permissions --model sonnet --output-format stream-json --input-format stream-json --verbose"
    args = shlex.split(base)

    # Pass through any additional args provided to the proxy
    args.extend(argv)

    # Replace current process with claude (preserves stdin/stdout/stderr, returns same code)
    try:
        os.execvp(args[0], args)
    except FileNotFoundError:
        print("Error: 'claude' command not found in PATH.", file=sys.stderr)
        return 127
    except Exception as e:
        print(f"Error launching claude via proxy: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

