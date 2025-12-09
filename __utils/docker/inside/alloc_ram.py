#!/usr/bin/env python3
"""
Allocate a large amount of memory to test container memory limits.

- Default target: 17 GiB
- Allocates in chunks and touches each page to force actual commitment
- Prints progress at each full GiB and stops on MemoryError or signal/OOM kill

Usage:
  python3 alloc_ram.py [TARGET_GIB]

Options:
  -h, --help    Show this help message and exit.

Environment:
  CHUNK_MB      Allocation chunk size in MiB (default: 256)
  HOLD_SECONDS  Seconds to hold allocation after success (default: 5)

This script is intended for testing memory limits in containers or CI by
intentionally allocating and touching memory up to a specified size.
"""

import os
import sys
import time
import signal

MiB = 1024 * 1024
GiB = 1024 * MiB

def format_bytes(n):
    try:
        if n >= GiB:
            return f"{n / GiB:.2f} GiB"
        if n >= MiB:
            return f"{n / MiB:.2f} MiB"
        return f"{n} B"
    except Exception:
        return str(n)

def get_rss_bytes():
    """Best-effort current RSS in bytes (Linux), else None."""
    try:
        with open("/proc/self/status", "r") as f:
            for line in f:
                if line.startswith("VmRSS:"):
                    parts = line.split()
                    # Format: VmRSS:  123456 kB
                    if len(parts) >= 3 and parts[2] == "kB":
                        return int(parts[1]) * 1024
    except Exception:
        pass
    return None

def parse_target_gib(argv):
    if len(argv) > 1:
        if argv[1] in ("-h", "--help", "help"):
            print(__doc__)
            sys.exit(0)
        try:
            return float(argv[1])
        except Exception:
            print("Invalid TARGET_GIB. Use a number, e.g., 17 or 4.5.")
            print(__doc__)
            sys.exit(2)
    return 17.0

def main():
    target_gib = parse_target_gib(sys.argv)
    chunk_mb = int(os.environ.get("CHUNK_MB", "256"))
    page_step = 4096  # 4 KiB

    target_bytes = int(target_gib * GiB)
    chunk_bytes = chunk_mb * MiB
    allocated = []
    total = 0

    print(f"Target: {target_gib:.2f} GiB (~{target_bytes/1e9:.1f} GB)")
    print(f"Chunk size: {chunk_mb} MiB; page touch stride: {page_step} bytes", flush=True)

    try:
        idx = 0
        last_gib_reported = -1
        while total < target_bytes:
            idx += 1
            # Allocate the chunk
            b = bytearray(chunk_bytes)
            # Touch each page to force commit
            # Write non-zero so it's not optimized as zero-page COW.
            stride = page_step
            for off in range(0, len(b), stride):
                b[off] = 1

            allocated.append(b)
            total += len(b)

            # Report once per full GiB reached
            curr_gib_int = total // GiB
            if curr_gib_int > last_gib_reported:
                last_gib_reported = curr_gib_int
                rss = get_rss_bytes()
                rss_str = f", RSS: {format_bytes(rss)}" if rss is not None else ""
                print(
                    f"Allocated: {total / GiB:6.2f} GiB (chunks: {idx}{rss_str})",
                    flush=True,
                )

        print("SUCCESS: Reached target allocation.")
        # Hold for a brief moment so external monitors can observe usage
        hold_sec = int(os.environ.get("HOLD_SECONDS", "5"))
        if hold_sec > 0:
            print(f"Holding allocation for {hold_sec}s...", flush=True)
            time.sleep(hold_sec)
    except MemoryError:
        rss = get_rss_bytes()
        print("\nMemoryError: Allocation failed (likely memory limit reached).")
        print(
            f" - Allocated so far: {total / GiB:.2f} GiB ({format_bytes(total)})"
        )
        print(f" - Target: {target_gib:.2f} GiB ({format_bytes(target_bytes)})")
        print(f" - Chunk size: {chunk_mb} MiB ({format_bytes(chunk_bytes)})")
        if rss is not None:
            print(f" - Current RSS: {format_bytes(rss)}")
        print(
            " - Hint: Lower TARGET_GIB or CHUNK_MB, or raise the container limit."
        )
        return 1
    except KeyboardInterrupt:
        print("Interrupted by user.")
    finally:
        # Free memory by clearing references
        allocated.clear()
        print("Freed allocated memory.")

if __name__ == "__main__":
    sys.exit(main())
