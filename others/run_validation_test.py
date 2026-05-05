#!/usr/bin/env python3
"""
Run validation tests against the C reference or Rust binary using the
existing testsXX.jsonl files (auto-discovered by testcmp.sh).

For target=c:    runs ./testcmp.sh compare ./<prog>.ref
For target=rust: builds rust_WIP if needed, then
                 runs ./testcmp.sh compare <abs-path-to-rust-binary>

Usage:
    python3 run_validation_test.py                          # all programs, c
    python3 run_validation_test.py --target c               # explicit c
    python3 run_validation_test.py --target rust            # rust binary
    python3 run_validation_test.py --target rust head cat   # rust, specific
    python3 run_validation_test.py -v --target rust head    # verbose

Flags:
  -h, --help     Show this help.
  -v, --verbose  Print names of failing/timeout tests.
"""

import re
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


def build_c_ref(c_dir: Path, program: str) -> tuple[bool, str]:
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


def run_compare(program: str, target: str) -> dict:
    c_dir = get_c_dir(program)
    if not c_dir.exists():
        return {"program": program, "error": f"C dir not found: {c_dir}"}

    if target == "rust":
        ok, msg, rust_bin = build_rust_bin(program)
        if not ok:
            return {"program": program, "error": f"rust build: {msg}"}
        binary_arg = str(rust_bin)
    else:
        ok, msg = build_c_ref(c_dir, program)
        if not ok:
            return {"program": program, "error": f"c build: {msg}"}
        binary_arg = f"./{program}.ref"

    result = subprocess.run(
        ["bash", "./testcmp.sh", "compare", binary_arg],
        cwd=c_dir, capture_output=True, text=True, timeout=300,
    )
    output = result.stdout + result.stderr

    res = {"program": program, "target": target, "output_tail": output[-500:]}
    m = re.search(r"(\d+) passed, (\d+) failed out of (\d+) tests", output)
    if m:
        res["passed"] = int(m.group(1))
        res["failed"] = int(m.group(2))
        res["total"] = int(m.group(3))
        res["pass_rate"] = res["passed"] / res["total"] if res["total"] else 0.0
    else:
        res["error"] = "could not parse pass/fail summary"

    # Collect failing test names for verbose mode
    fail_names = re.findall(r"^FAIL: (\S+)", output, re.MULTILINE)
    timeout_names = re.findall(
        r"^TIMEOUT: (?:Test ')?(\S+?)(?:'|\s|$)", output, re.MULTILINE
    )
    res["fail_names"] = fail_names
    res["timeout_names"] = [t for t in timeout_names if t not in ("Reference", "Test")]
    return res


def parse_args(argv):
    """Parse: [--target c|rust] [-v|--verbose] [program ...]"""
    target = "c"
    verbose = False
    rest = argv[1:]
    progs = []
    i = 0
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
        elif rest[i] in ("-h", "--help"):
            print(__doc__)
            sys.exit(0)
        elif rest[i] in ("-v", "--verbose"):
            verbose = True
            i += 1
        else:
            progs.append(rest[i])
            i += 1

    if not progs:
        progs = list(PROGRAMS)
    for p in progs:
        if p not in PROGRAMS:
            print(f"Error: unknown program '{p}'. Must be one of: {PROGRAMS}")
            sys.exit(1)
    return target, verbose, progs


def main():
    target, verbose, programs = parse_args(sys.argv)

    print(f"Target:   {target}")
    print(f"Programs: {', '.join(programs)}")
    print("=" * 70)

    results = []
    for prog in programs:
        print(f"\n[{prog}] target={target} ...")
        try:
            r = run_compare(prog, target)
        except subprocess.TimeoutExpired:
            r = {"program": prog, "error": "timeout (300s)"}
        except Exception as e:
            r = {"program": prog, "error": str(e)}
        results.append(r)

        if "error" in r:
            print(f"  ERROR: {r['error']}")
        else:
            print(
                f"  Tests: {r['passed']}/{r['total']} passed "
                f"({r['pass_rate'] * 100:.2f}%), {r['failed']} failed"
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

    # Summary
    print("\n" + "=" * 70)
    print(f"{'Program':<12} {'Pass':>6} {'Total':>6} {'Pass Rate':>11}")
    print("-" * 70)
    total_pass = 0
    total_all = 0
    for r in results:
        if "error" in r:
            print(f"{r['program']:<12} ERROR: {r['error'][:50]}")
        else:
            print(
                f"{r['program']:<12} "
                f"{r['passed']:>6} "
                f"{r['total']:>6} "
                f"{r['pass_rate'] * 100:>10.2f}%"
            )
            total_pass += r["passed"]
            total_all += r["total"]
    print("-" * 70)
    if total_all and len(programs) > 1:
        print(
            f"{'OVERALL':<12} {total_pass:>6} {total_all:>6} "
            f"{total_pass / total_all * 100:>10.2f}%"
        )
    print()


if __name__ == "__main__":
    main()
