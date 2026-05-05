#!/usr/bin/env python3
"""
Run seed_tests.jsonl in isolation for coreutils_actor programs.

Temporarily hides existing testsXX.jsonl files, makes seed_tests.jsonl
the only test file, runs testcmp.sh (compare or coverage), then reverts.

Usage:
    python3 run_seed_tests.py compare                          # all programs, c
    python3 run_seed_tests.py compare --target c               # explicit c
    python3 run_seed_tests.py compare --target rust            # rust binary
    python3 run_seed_tests.py compare --target rust head cat   # rust, specific
    python3 run_seed_tests.py coverage                         # always c (rust unsupported)
    python3 run_seed_tests.py coverage head                    # coverage for head only
    python3 run_seed_tests.py compare -v --target rust head    # verbose: print failing tests

Flags:
  -h, --help     Show this help.
  -v, --verbose  Print names of failing/timeout tests.

Notes:
  - --target rust is only valid with `compare` mode.
  - For --target rust, the binary at <prog>/rust_WIP/target/release/<prog>
    is used; auto-built if missing.
"""

import re
import shutil
import subprocess
import sys
from pathlib import Path

BASE_DIR = Path(__file__).parent / "c2saferrust" / "coreutils_actor" / "src"
PROGRAMS = ["cat", "head", "pwd", "split", "tail", "truncate", "uniq"]
RUST_FOLDER = "rust_WIP"


def get_c_dir(program: str) -> Path:
    return BASE_DIR / program / "c"


def get_rust_dir(program: str) -> Path:
    return BASE_DIR / program / RUST_FOLDER


def build_c_ref(c_dir: Path) -> tuple[bool, str]:
    """Ensure {prog}.ref exists in c_dir."""
    program = c_dir.parent.name
    ref = c_dir / f"{program}.ref"
    if ref.exists():
        return True, "already built"
    r = subprocess.run(
        ["make", "all"], cwd=c_dir, capture_output=True, text=True, timeout=120
    )
    if r.returncode != 0:
        return False, f"make failed: {r.stderr[-200:]}"
    if not ref.exists():
        return False, f"missing after build: {ref}"
    return True, "built"


def build_rust_bin(program: str) -> tuple[bool, str, Path]:
    """cargo build --release if missing. Returns (ok, msg, abs_bin_path)."""
    rust_dir = get_rust_dir(program)
    binary = rust_dir / "target" / "release" / program
    if not rust_dir.exists():
        return False, f"rust dir not found: {rust_dir}", binary
    if binary.exists():
        return True, "already built", binary.resolve()
    r = subprocess.run(
        ["cargo", "build", "--release"],
        cwd=rust_dir, capture_output=True, text=True, timeout=600,
    )
    if r.returncode != 0:
        return False, f"cargo build failed: {r.stderr[-300:]}", binary
    if not binary.exists():
        return False, f"binary missing after build: {binary}", binary
    return True, "built", binary.resolve()


def run_seed_tests(program: str, mode: str, target: str) -> dict:
    """
    Run seed_tests.jsonl in isolation for a single program.

    Steps:
      1. Rename all testsXX.jsonl -> testsXX.jsonl.bak
      2. Copy seed_tests.jsonl -> tests00.jsonl
      3. Run ./testcmp.sh <mode> [<binary>]
      4. Revert: remove tests00.jsonl, rename .bak files back

    Returns dict with results.
    """
    c_dir = get_c_dir(program)

    if not c_dir.exists():
        return {"program": program, "error": f"Directory not found: {c_dir}"}

    seed_file = c_dir / "seed_tests.jsonl"
    if not seed_file.exists():
        return {"program": program, "error": "No seed_tests.jsonl found"}

    # Determine binary to compare against
    if mode == "compare" and target == "rust":
        ok, msg, rust_bin = build_rust_bin(program)
        if not ok:
            return {"program": program, "error": f"rust build: {msg}"}
        binary_arg = str(rust_bin)
    else:
        # mode == coverage, or target == "c": use {prog}.ref in c_dir
        ok, msg = build_c_ref(c_dir)
        if not ok:
            return {"program": program, "error": f"c build: {msg}"}
        binary_arg = f"./{program}.ref"

    # Find existing testsXX.jsonl files
    existing = sorted(c_dir.glob("tests[0-9][0-9].jsonl"))

    # Step 1: Rename existing files
    renamed = []
    output = ""
    try:
        for f in existing:
            bak = f.with_suffix(".jsonl.bak")
            f.rename(bak)
            renamed.append((bak, f))

        # Step 2: Copy seed_tests.jsonl to tests00.jsonl
        temp_test = c_dir / "tests00.jsonl"
        shutil.copy2(seed_file, temp_test)

        # Step 3: Run testcmp.sh
        if mode == "compare":
            cmd = ["bash", "./testcmp.sh", "compare", binary_arg]
        else:
            cmd = ["bash", "./testcmp.sh", "coverage"]

        result = subprocess.run(
            cmd, cwd=c_dir, capture_output=True, text=True, timeout=300
        )
        output = result.stdout + result.stderr

    finally:
        # Step 4: Revert — always runs even on error
        temp_test = c_dir / "tests00.jsonl"
        if temp_test.exists():
            temp_test.unlink()

        for bak, orig in renamed:
            if bak.exists():
                bak.rename(orig)

    # Parse results
    res = {
        "program": program, "mode": mode, "target": target,
        "output_tail": output[-2000:],
    }

    m = re.search(r"(\d+) passed, (\d+) failed out of (\d+) tests", output)
    if m:
        res["passed"] = int(m.group(1))
        res["failed"] = int(m.group(2))
        res["total"] = int(m.group(3))
    else:
        # Coverage mode: count "Running test:" lines
        runs = re.findall(r"Running test: .+?\.\.\.", output)
        timeouts = len(re.findall(r"TIMEOUT", output))
        if runs:
            res["total"] = len(runs)
            res["failed"] = timeouts
            res["passed"] = len(runs) - timeouts

    # Collect failing test names for verbose mode
    fail_names = re.findall(r"^FAIL: (\S+)", output, re.MULTILINE)
    timeout_names = re.findall(
        r"^TIMEOUT: (?:Test ')?(\S+?)(?:'|\s|$)", output, re.MULTILINE
    )
    res["fail_names"] = fail_names
    res["timeout_names"] = [t for t in timeout_names if t not in ("Reference", "Test")]

    if "total" in res and res["total"]:
        res["pass_rate"] = res["passed"] / res["total"]

    # Parse main .c coverage if coverage mode
    if mode == "coverage":
        pattern = rf"File '{program}\.c'\nLines executed:(\d+\.\d+)% of (\d+)"
        m = re.search(pattern, output)
        if m:
            res["main_c_percent"] = float(m.group(1))
            res["main_c_lines"] = int(m.group(2))

    return res


def parse_args(argv):
    """Parse: <mode> [--target c|rust] [-v|--verbose] [program ...]"""
    # Help flag handling (anywhere in argv)
    if len(argv) < 2 or any(a in ("-h", "--help") for a in argv[1:]):
        print(__doc__)
        sys.exit(0 if len(argv) >= 2 else 1)

    mode = argv[1]
    if mode not in ("compare", "coverage"):
        print(f"Error: mode must be 'compare' or 'coverage', got '{mode}'")
        print("Run with -h for help.")
        sys.exit(1)

    target = "c"
    verbose = False
    rest = argv[2:]
    i = 0
    progs = []
    while i < len(rest):
        if rest[i] == "--target":
            if i + 1 >= len(rest):
                print("Error: --target requires a value (c|rust)")
                sys.exit(1)
            target = rest[i + 1]
            if target not in ("c", "rust"):
                print(f"Error: --target must be 'c' or 'rust', got '{target}'")
                sys.exit(1)
            i += 2
        elif rest[i] in ("-v", "--verbose"):
            verbose = True
            i += 1
        else:
            progs.append(rest[i])
            i += 1

    if mode == "coverage" and target == "rust":
        print("Error: --target rust is only valid with `compare` mode")
        sys.exit(1)

    if not progs:
        progs = list(PROGRAMS)

    for p in progs:
        if p not in PROGRAMS:
            print(f"Error: unknown program '{p}'. Must be one of: {PROGRAMS}")
            sys.exit(1)
    return mode, target, verbose, progs


def main():
    mode, target, verbose, programs = parse_args(sys.argv)

    print(f"Mode:     {mode}")
    print(f"Target:   {target}")
    print(f"Programs: {', '.join(programs)}")
    print("=" * 70)

    results = []
    for prog in programs:
        print(f"\n[{prog}] Running seed_tests.jsonl ({mode}, target={target})...")
        try:
            r = run_seed_tests(prog, mode, target)
        except subprocess.TimeoutExpired:
            r = {"program": prog, "error": "Timed out (300s)"}
        except Exception as e:
            r = {"program": prog, "error": str(e)}
        results.append(r)

        if "error" in r:
            print(f"  ERROR: {r['error']}")
        else:
            passed = r.get("passed", "?")
            failed = r.get("failed", "?")
            total = r.get("total", "?")
            rate = r.get("pass_rate")
            rate_s = f" ({rate * 100:.2f}%)" if rate is not None else ""
            print(f"  Tests: {passed}/{total} passed{rate_s}, {failed} failed")
            if "main_c_percent" in r:
                print(
                    f"  {prog}.c coverage: {r['main_c_percent']}% "
                    f"of {r['main_c_lines']} lines"
                )
            if verbose:
                fails = r.get("fail_names", [])
                tos = r.get("timeout_names", [])
                if fails:
                    print(f"  Failed tests ({len(fails)}):")
                    for n in fails:
                        print(f"    FAIL    {n}")
                if tos:
                    print(f"  Timeout tests ({len(tos)}):")
                    for n in tos:
                        print(f"    TIMEOUT {n}")
                if not fails and not tos and (r.get("failed") or 0) > 0:
                    print(f"  ({r['failed']} failures, but no test names parsed)")

    # Summary table
    print("\n" + "=" * 70)
    header = f"{'Program':<12} {'Tests':>6} {'Pass':>6} {'Fail':>6} {'Rate':>9}"
    if mode == "coverage":
        header += f" {'Main .c%':>10}"
    print(header)
    print("-" * 70)

    total_pass = 0
    total_all = 0
    for r in results:
        if "error" in r:
            print(f"{r['program']:<12} ERROR: {r['error'][:50]}")
        else:
            rate = r.get("pass_rate")
            rate_s = f"{rate * 100:>8.2f}%" if rate is not None else "       -"
            line = (
                f"{r['program']:<12} "
                f"{r.get('total', '?'):>6} "
                f"{r.get('passed', '?'):>6} "
                f"{r.get('failed', '?'):>6} "
                f"{rate_s}"
            )
            if mode == "coverage" and "main_c_percent" in r:
                line += f" {r['main_c_percent']:>9.2f}%"
            print(line)
            if isinstance(r.get("passed"), int) and isinstance(r.get("total"), int):
                total_pass += r["passed"]
                total_all += r["total"]

    print("-" * 70)
    if total_all and len(programs) > 1:
        rate = total_pass / total_all
        print(
            f"{'OVERALL':<12} {total_all:>6} {total_pass:>6} "
            f"{total_all - total_pass:>6} {rate * 100:>8.2f}%"
        )
    print()


if __name__ == "__main__":
    main()
