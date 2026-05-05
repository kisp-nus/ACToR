#!/usr/bin/env python3
"""
Filter bad test cases from ancient_BSD_tests.

For each project folder:
  Step 1: Compile C code with two flag sets (-O0 and -O2)
  Step 2: Run every test against both binaries, detect bad tests
  Step 3: Remove bad tests from JSONL files

A test is "bad" if ANY of:
  - Either binary crashes (exit code 132-139, i.e. killed by signal)
  - Output contains crash keywords: "core dumped", "Segmentation fault",
    "Aborted", "Bus error", "SIGABRT", "SIGSEGV", etc.
  - Output contains sanitizer keywords (AddressSanitizer, UBSan, etc.)
  - The two binaries produce different output or different exit codes
    (sign of undefined behavior / optimization-sensitive code)
  - Test times out (>1s)

Usage:
    python3 filter_bad_tests.py [--dry-run] [--timeout SECS] [--projects P1,P2,...] [--verbose]
"""

import argparse
import glob
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

TESTS_DIR = Path(__file__).parent / "ancient_BSD_tests"
COREUTILS_DIR = Path(__file__).parent / "c2saferrust" / "coreutils_actor" / "src"
DANGEROUS_JSON = Path(__file__).parent / "scripts" / "dangerous.json"

# Crash/error keywords to look for in output
CRASH_PATTERNS = re.compile(
    r"core dumped|Segmentation fault|SIGSEGV|SIGBUS|Bus error|SIGABRT|"
    r"Aborted|Illegal instruction|SIGILL|SIGFPE|"
    r"Floating point exception|stack smashing detected|"
    r"buffer overflow detected|double free or corruption|"
    r"free\(\): invalid pointer|malloc\(\): corrupted|"
    r"munmap_chunk\(\): invalid pointer",
    re.IGNORECASE,
)

SANITIZER_PATTERNS = re.compile(
    r"AddressSanitizer|LeakSanitizer|MemorySanitizer|"
    r"ThreadSanitizer|UndefinedBehaviorSanitizer|"
    r"runtime error:|ERROR: .*Sanitizer",
    re.IGNORECASE,
)

# Signal exit codes: 128 + signal_number
SIGNAL_EXIT_CODES = set(range(128, 160))  # signals 0-31


def parse_makefile(makefile_path: Path, proj_dir: Path = None) -> dict:
    """Parse Makefile to extract TARGET, SOURCES, LIBS, CC, and CFLAGS.
    
    Supports both explicit SOURCES and wildcard $(wildcard *.c) patterns.
    """
    info = {
        "cc": "gcc",
        "target": None,
        "sources": [],
        "libs": "",
        "cflags_base": [],
        "defines": [],
        "include_dirs": [],
    }

    with open(makefile_path) as f:
        content = f.read()

    for line in content.splitlines():
        line = line.strip()
        if line.startswith("CC"):
            m = re.match(r"CC\s*=\s*(.*)", line)
            if m:
                info["cc"] = m.group(1).strip()
        elif line.startswith("TARGET =") or line.startswith("TARGET="):
            m = re.match(r"TARGET\s*=\s*(.*)", line)
            if m:
                info["target"] = m.group(1).strip()
        elif line.startswith("SOURCES") or line.startswith("SRCS"):
            m = re.match(r"(?:SOURCES|SRCS)\s*:?=\s*(.*)", line)
            if m:
                val = m.group(1).strip()
                # Check for wildcard pattern
                if "$(wildcard" in val:
                    # Extract the pattern, e.g., $(wildcard *.c)
                    wm = re.search(r"\$\(wildcard\s+([^)]+)\)", val)
                    if wm and proj_dir:
                        pattern = wm.group(1).strip()
                        # Resolve the wildcard
                        import glob as glob_module
                        matched = glob_module.glob(str(proj_dir / pattern))
                        info["sources"] = [os.path.basename(f) for f in matched]
                else:
                    info["sources"] = val.split()
        elif line.startswith("LIBS"):
            m = re.match(r"LIBS\s*=\s*(.*)", line)
            if m:
                info["libs"] = m.group(1).strip()
        elif line.startswith("CFLAGS =") or line.startswith("CFLAGS="):
            m = re.match(r"CFLAGS\s*=\s*(.*)", line)
            if m:
                # Parse out flags — keep defines, std, include dirs, etc.
                flags = m.group(1).strip().split()
                for flag in flags:
                    if flag.startswith("-D") or flag.startswith("-std"):
                        info["defines"].append(flag)
                    elif flag.startswith("-I"):
                        info["include_dirs"].append(flag)
                    elif flag in ("-Wall", "-Wextra"):
                        info["cflags_base"].append(flag)

    return info


def compile_binary(
    proj_dir: Path, mk_info: dict, opt_flag: str, suffix: str
) -> Path | None:
    """Compile the project with given optimization flag. Returns binary path or None."""
    output = proj_dir / f"{mk_info['target']}.{suffix}"

    cflags = [mk_info["cc"]]
    cflags.extend(mk_info["cflags_base"])
    cflags.append(opt_flag)
    cflags.extend(mk_info["defines"])
    cflags.extend(mk_info.get("include_dirs", []))
    cflags.extend(["-o", str(output)])
    cflags.extend([str(proj_dir / s) for s in mk_info["sources"]])
    if mk_info["libs"]:
        cflags.extend(mk_info["libs"].split())

    try:
        result = subprocess.run(
            cflags,
            capture_output=True,
            text=True,
            timeout=60,  # Increased timeout for larger projects
            cwd=str(proj_dir),
        )
        if result.returncode != 0:
            print(f"  [COMPILE ERROR] {opt_flag}: {result.stderr.strip()[:500]}")
            return None
        os.chmod(output, 0o755)
        return output
    except Exception as e:
        print(f"  [COMPILE EXCEPTION] {opt_flag}: {e}")
        return None


def load_tests(proj_dir: Path) -> list[tuple[Path, list[dict]]]:
    """Load all testsXX.jsonl files from a project directory.
    Returns list of (filepath, [test_dicts])."""
    results = []
    jsonl_files = sorted(glob.glob(str(proj_dir / "tests[0-9][0-9].jsonl")))
    for jf in jsonl_files:
        jf_path = Path(jf)
        tests = []
        with open(jf_path) as f:
            for line_num, line in enumerate(f, 1):
                line = line.strip()
                if not line:
                    continue
                try:
                    tests.append(json.loads(line))
                except json.JSONDecodeError as e:
                    print(f"  [JSON ERROR] {jf_path.name} line {line_num}: {e}")
        results.append((jf_path, tests))
    return results


def apply_norm_rules(output: str, test: dict, progname: str) -> str:
    """Apply normalization rules from a test case."""
    norm_rules = test.get("norm_rules", [])
    if not norm_rules:
        return output

    normalized = output
    for rule in norm_rules:
        pattern = rule.get("pattern", "")
        replacement = rule.get("replacement", "")
        # Replace {progname} placeholders
        pattern = pattern.replace("{progname}", progname)
        replacement = replacement.replace("{progname}", progname)
        try:
            normalized = re.sub(pattern, replacement, normalized)
        except re.error:
            pass  # skip bad regex

    return normalized


def run_single_test(
    binary: Path,
    test: dict,
    proj_dir: Path,
    timeout_secs: float,
    progname: str,
) -> tuple[int, str, str]:
    """Run a single test case. Returns (exit_code, stdout+stderr, reason_if_bad).

    reason_if_bad is empty string if test is fine, otherwise a description.
    """
    cmd_prep = test.get("cmd_prep", "") or ""
    cmd_target = test.get("cmd_target", "")
    cmd_post = test.get("cmd_post", "") or ""
    alias_name = test.get("alias_name", "") or ""

    if not alias_name:
        alias_name = "main"

    # Build the actual command
    # Handle cmd_target: could be a string or a list (check_file mode)
    if isinstance(cmd_target, list):
        target_cmd_str = cmd_target[0]
    else:
        target_cmd_str = cmd_target

    # Create a temporary working directory to avoid conflicts
    work_dir = tempfile.mkdtemp(prefix=f"test_{progname}_", dir=proj_dir)
    try:
        # Create the alias symlink
        alias_path = os.path.join(work_dir, alias_name)
        abs_binary = str(binary.resolve())
        os.symlink(abs_binary, alias_path)

        actual_binary = f"./{alias_name}"

        # Replace BINARY placeholder
        target_cmd_str = target_cmd_str.replace("BINARY", actual_binary)

        # Build the full shell command: prep && target; post
        parts = []
        if cmd_prep:
            parts.append(cmd_prep)
        parts.append(target_cmd_str)
        full_cmd = " && ".join(parts)

        # Append post command (always run for cleanup, use ;)
        if cmd_post:
            full_cmd = f"({full_cmd}); {cmd_post}"

        try:
            result = subprocess.run(
                ["bash", "-c", full_cmd],
                capture_output=True,
                timeout=timeout_secs,
                cwd=work_dir,
            )
            # Decode with surrogateescape to handle binary output
            combined = (
                result.stdout.decode("utf-8", errors="surrogateescape")
                + result.stderr.decode("utf-8", errors="surrogateescape")
            )
            exit_code = result.returncode

            # Check for crash signals
            if exit_code in SIGNAL_EXIT_CODES:
                return exit_code, combined, f"signal_exit({exit_code - 128})"

            # Negative exit codes also indicate signals
            if exit_code < 0:
                return exit_code, combined, f"signal({-exit_code})"

            # Check for crash keywords in output
            if CRASH_PATTERNS.search(combined):
                return exit_code, combined, "crash_keyword"

            # Check for sanitizer messages
            if SANITIZER_PATTERNS.search(combined):
                return exit_code, combined, "sanitizer"

            # Apply normalization before returning
            normalized = apply_norm_rules(combined, test, progname)
            return exit_code, normalized, ""

        except subprocess.TimeoutExpired:
            return 124, "", "timeout"
        except Exception as e:
            return -1, str(e), f"exception({e})"
    finally:
        # Clean up the temp work directory
        try:
            shutil.rmtree(work_dir)
        except Exception:
            pass


def check_test(
    binary_o0: Path,
    binary_o2: Path,
    test: dict,
    proj_dir: Path,
    timeout_secs: float,
    progname: str,
) -> tuple[str, str]:
    """Check if a test is bad.
    Returns (test_name, reason) where reason is "" if test is good.
    """
    test_name = test["name"]

    # Run with -O0 binary
    exit_o0, output_o0, reason_o0 = run_single_test(
        binary_o0, test, proj_dir, timeout_secs, progname
    )
    if reason_o0:
        return test_name, f"O0:{reason_o0}"

    # Run with -O2 binary
    exit_o2, output_o2, reason_o2 = run_single_test(
        binary_o2, test, proj_dir, timeout_secs, progname
    )
    if reason_o2:
        return test_name, f"O2:{reason_o2}"

    # Compare outputs between the two compilation modes
    if exit_o0 != exit_o2:
        return test_name, f"exit_mismatch(O0={exit_o0},O2={exit_o2})"

    if output_o0 != output_o2:
        print(f"  [OUTPUT MISMATCH] {test_name}: {output_o0} != {output_o2}")
        return test_name, f"output_mismatch"

    return test_name, ""


def process_project(
    proj_dir: Path,
    timeout_secs: float,
    dry_run: bool,
    verbose: bool,
    coreutils_mode: bool = False,
) -> dict:
    """Process a single project. Returns stats dict."""
    # For coreutils, the project dir is src/PROG/c/, so name is parent
    proj_name = proj_dir.parent.name if coreutils_mode else proj_dir.name
    makefile = proj_dir / "Makefile"

    stats = {
        "project": proj_name,
        "total": 0,
        "bad": 0,
        "good": 0,
        "bad_tests": [],
        "compile_error": False,
    }

    if not makefile.exists():
        print(f"  [SKIP] {proj_name}: no Makefile")
        return stats

    # Parse Makefile
    mk_info = parse_makefile(makefile, proj_dir)
    if not mk_info["target"] or not mk_info["sources"]:
        print(f"  [SKIP] {proj_name}: could not parse TARGET/SOURCES from Makefile")
        return stats

    progname = mk_info["target"]

    # Step 1: Compile with -O0 and -O2
    bin_o0 = compile_binary(proj_dir, mk_info, "-O0", "filter_o0")
    bin_o2 = compile_binary(proj_dir, mk_info, "-O2", "filter_o2")

    if not bin_o0 or not bin_o2:
        stats["compile_error"] = True
        print(f"  [SKIP] {proj_name}: compilation failed")
        # cleanup
        for suffix in ("filter_o0", "filter_o2"):
            p = proj_dir / f"{progname}.{suffix}"
            if p.exists():
                p.unlink()
        return stats

    # Load tests
    all_test_files = load_tests(proj_dir)
    if not all_test_files:
        print(f"  [SKIP] {proj_name}: no test files")
        for suffix in ("filter_o0", "filter_o2"):
            (proj_dir / f"{progname}.{suffix}").unlink(missing_ok=True)
        return stats

    # Step 2: Run all tests and identify bad ones
    bad_test_names = set()

    for jf_path, tests in all_test_files:
        for test in tests:
            stats["total"] += 1
            tname, reason = check_test(
                bin_o0, bin_o2, test, proj_dir, timeout_secs, progname
            )
            if reason:
                bad_test_names.add(tname)
                stats["bad"] += 1
                stats["bad_tests"].append((tname, reason))
                if verbose:
                    print(f"    BAD: {tname} — {reason}")
            else:
                stats["good"] += 1

    # Step 3: Remove bad tests from JSONL files
    if bad_test_names:
        if dry_run:
            print(
                f"  [DRY-RUN] Would remove {len(bad_test_names)} bad test(s): "
                f"{', '.join(sorted(bad_test_names))}"
            )
        else:
            for jf_path, tests in all_test_files:
                filtered = [t for t in tests if t["name"] not in bad_test_names]
                removed_count = len(tests) - len(filtered)
                if removed_count > 0:
                    # Re-index
                    for i, t in enumerate(filtered, 1):
                        t["idx"] = i
                    # Write back
                    with open(jf_path, "w") as f:
                        for t in filtered:
                            f.write(json.dumps(t, ensure_ascii=False) + "\n")
                    print(
                        f"    Removed {removed_count} test(s) from {jf_path.name}, "
                        f"{len(filtered)} remaining"
                    )

    # Cleanup compiled binaries
    for suffix in ("filter_o0", "filter_o2"):
        (proj_dir / f"{progname}.{suffix}").unlink(missing_ok=True)

    return stats


def main():
    parser = argparse.ArgumentParser(
        description="Filter bad test cases from ancient_BSD_tests"
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Show what would be removed without modifying files",
    )
    parser.add_argument(
        "--timeout",
        type=float,
        default=1.0,
        help="Timeout per test in seconds (default: 1.0)",
    )
    parser.add_argument(
        "--projects",
        type=str,
        default="",
        help="Comma-separated list of project names to process (default: all)",
    )
    parser.add_argument(
        "--verbose", "-v", action="store_true", help="Print each bad test as found"
    )
    parser.add_argument(
        "--tests-dir",
        type=str,
        default=str(TESTS_DIR),
        help=f"Path to tests directory (default: {TESTS_DIR})",
    )
    parser.add_argument(
        "--coreutils",
        action="store_true",
        help="Use coreutils_actor directory structure (tests in src/PROG/c/)",
    )
    args = parser.parse_args()

    # Determine base directory based on mode
    if args.coreutils:
        tests_dir = COREUTILS_DIR
        print(f"Using coreutils mode: {tests_dir}")
    else:
        tests_dir = Path(args.tests_dir)
    
    if not tests_dir.is_dir():
        print(f"Error: {tests_dir} is not a directory")
        sys.exit(1)

    # Load dangerous projects list (only for non-coreutils mode)
    dangerous_projects = set()
    if not args.coreutils:
        dangerous_json = Path(args.tests_dir).parent / "scripts" / "dangerous.json"
        if dangerous_json.exists():
            with open(dangerous_json) as f:
                data = json.load(f)
                dangerous_projects = set(data.get("ignore_list", []))
            print(f"Skipping dangerous projects: {', '.join(sorted(dangerous_projects))}")

    # Determine which projects to process
    if args.projects:
        project_names = [p.strip() for p in args.projects.split(",")]
        proj_dirs = []
        for name in project_names:
            if name in dangerous_projects:
                print(f"Warning: project '{name}' is in dangerous list, skipping")
                continue
            if args.coreutils:
                # For coreutils, structure is src/PROG/c/
                d = tests_dir / name / "c"
            else:
                d = tests_dir / name
            if d.is_dir():
                proj_dirs.append(d)
            else:
                print(f"Warning: project '{name}' not found at {d}")
    else:
        if args.coreutils:
            # For coreutils, find all src/PROG/c/ directories
            proj_dirs = sorted(
                [
                    d / "c"
                    for d in tests_dir.iterdir()
                    if d.is_dir() and (d / "c").is_dir()
                ],
                key=lambda p: p.parent.name,
            )
        else:
            proj_dirs = sorted(
                [
                    d
                    for d in tests_dir.iterdir()
                    if d.is_dir() and d.name not in dangerous_projects
                ],
                key=lambda p: p.name,
            )

    print(f"Processing {len(proj_dirs)} project(s) from {tests_dir}")
    print(f"Timeout: {args.timeout}s per test")
    if args.dry_run:
        print("Mode: DRY-RUN (no files will be modified)")
    print("=" * 70)

    total_stats = {
        "projects": 0,
        "total_tests": 0,
        "bad_tests": 0,
        "good_tests": 0,
        "compile_errors": 0,
        "all_bad": [],
    }

    for proj_dir in proj_dirs:
        # For coreutils mode, proj_dir is src/PROG/c/, so use parent.name
        proj_name = proj_dir.parent.name if args.coreutils else proj_dir.name
        print(f"\n[{proj_name}]")

        stats = process_project(proj_dir, args.timeout, args.dry_run, args.verbose, args.coreutils)

        total_stats["projects"] += 1
        total_stats["total_tests"] += stats["total"]
        total_stats["bad_tests"] += stats["bad"]
        total_stats["good_tests"] += stats["good"]
        if stats["compile_error"]:
            total_stats["compile_errors"] += 1
        total_stats["all_bad"].extend(
            [(stats["project"], name, reason) for name, reason in stats["bad_tests"]]
        )

        if stats["total"] > 0:
            print(
                f"  Result: {stats['good']}/{stats['total']} good, "
                f"{stats['bad']} bad"
            )

    # Summary
    print("\n" + "=" * 70)
    print("SUMMARY")
    print(f"  Projects processed: {total_stats['projects']}")
    print(f"  Compile errors:     {total_stats['compile_errors']}")
    print(f"  Total tests:        {total_stats['total_tests']}")
    print(f"  Good tests:         {total_stats['good_tests']}")
    print(f"  Bad tests:          {total_stats['bad_tests']}")

    if total_stats["all_bad"]:
        print(f"\nAll bad tests ({len(total_stats['all_bad'])}):")
        for proj, name, reason in total_stats["all_bad"]:
            print(f"  {proj}/{name}: {reason}")


if __name__ == "__main__":
    main()
