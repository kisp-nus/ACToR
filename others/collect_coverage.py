#!/usr/bin/env python3
"""
Collect line coverage statistics for the 7 coreutils_actor C programs.

For each program under coreutils_actor/src/<prog>/c/:
  1. Builds the reference binary (make all)
  2. Runs ./testcmp.sh coverage to execute all tests and generate gcov data
  3. Parses the per-file gcov output to compute covered / total executable lines
  4. Prints a summary table and writes results to collect_coverage.json

Usage:
    python3 collect_coverage.py            # run all 7 programs
    python3 collect_coverage.py cat head   # run only specified programs
"""

import json
import os
import re
import subprocess
import sys
from pathlib import Path

BASE_DIR = Path(__file__).parent / "c2saferrust"
SRC_DIR = BASE_DIR / "coreutils_actor" / "src"

PROGRAMS = ["cat", "head", "pwd", "split", "tail", "truncate", "uniq"]


def build_program(prog_dir: Path) -> bool:
    """Run make all in the program directory. Returns True on success."""
    result = subprocess.run(
        ["make", "all"],
        cwd=prog_dir,
        capture_output=True,
        text=True,
        timeout=120,
    )
    if result.returncode != 0:
        print(f"  [build] FAILED: {result.stderr.strip()[-200:]}")
        return False
    return True


def run_coverage(prog_dir: Path) -> str:
    """Run ./testcmp.sh coverage and return the combined stdout+stderr.

    If testcmp.sh fails to produce .gcov files (due to a gcda naming mismatch
    bug in some programs), falls back to running gcov directly on all .ref.gcda
    files.
    """
    result = subprocess.run(
        ["bash", "./testcmp.sh", "coverage"],
        cwd=prog_dir,
        capture_output=True,
        text=True,
        timeout=600,
    )
    output = result.stdout + result.stderr

    # Check if .gcov files were actually generated
    gcov_files = list(prog_dir.glob("*.gcov"))
    if not gcov_files:
        # Fallback: run gcov directly on all .ref.gcda files
        gcda_files = list(prog_dir.glob("*.ref.gcda"))
        if gcda_files:
            gcda_names = [f.name for f in gcda_files]
            subprocess.run(
                ["gcov", "-b", "-c"] + gcda_names,
                cwd=prog_dir,
                capture_output=True,
                text=True,
                timeout=120,
            )

    return output


def parse_gcov_files(prog_dir: Path):
    """
    Parse all *.gcov files in prog_dir to compute per-file and total coverage.

    A line is 'executable' if it matches the gcov format:
        <count>: <lineno>: <source>     (covered, count is a number)
        #####:   <lineno>: <source>     (uncovered)

    System headers (error.h, stdio.h) are excluded since they are not
    part of the project source.

    Returns (file_stats, total_covered, total_executable).
    file_stats is a list of dicts sorted by uncovered count descending.
    """
    skip_files = {"error.h.gcov", "stdio.h.gcov"}
    file_stats = []
    total_covered = 0
    total_executable = 0

    for gcov_file in sorted(prog_dir.glob("*.gcov")):
        if gcov_file.name in skip_files:
            continue

        covered = 0
        uncovered = 0
        with open(gcov_file, "r", errors="replace") as fh:
            for line in fh:
                m = re.match(r"\s+(#####|\d+\*?):\s+\d+:", line)
                if m:
                    if m.group(1) == "#####":
                        uncovered += 1
                    else:
                        covered += 1

        executable = covered + uncovered
        if executable > 0:
            source_name = gcov_file.name.removesuffix(".gcov")
            file_stats.append(
                {
                    "file": source_name,
                    "covered": covered,
                    "uncovered": uncovered,
                    "executable": executable,
                    "percent": round(100 * covered / executable, 2),
                }
            )
            total_covered += covered
            total_executable += uncovered + covered

    file_stats.sort(key=lambda x: -x["uncovered"])
    return file_stats, total_covered, total_executable


def parse_test_counts(output: str):
    """Extract total test count and pass/fail from testcmp.sh coverage output.

    Coverage mode outputs lines like:
        Running test: <name>... exit_code: N
        Running test: <name>... TIMEOUT (500ms exceeded)

    We count total runs, and treat timeouts as failures.
    """
    runs = re.findall(r"Running test: .+?\.\.\.", output)
    tests_total = len(runs)
    timeouts = len(re.findall(r"TIMEOUT", output))
    tests_fail = timeouts
    tests_pass = tests_total - tests_fail
    return tests_total, tests_pass, tests_fail


def parse_main_source_coverage(prog_dir: Path, program: str):
    """Get coverage stats for the main source file (e.g. cat.c)."""
    gcov_file = prog_dir / f"{program}.c.gcov"
    if not gcov_file.exists():
        return None

    covered = 0
    uncovered = 0
    with open(gcov_file, "r", errors="replace") as fh:
        for line in fh:
            m = re.match(r"\s+(#####|\d+\*?):\s+\d+:", line)
            if m:
                if m.group(1) == "#####":
                    uncovered += 1
                else:
                    covered += 1

    executable = covered + uncovered
    if executable == 0:
        return None
    return {
        "covered": covered,
        "executable": executable,
        "percent": round(100 * covered / executable, 2),
    }


def collect_program_coverage(program: str):
    """Collect coverage data for a single program. Returns a result dict."""
    prog_dir = SRC_DIR / program / "c"
    if not prog_dir.exists():
        return {"program": program, "error": f"Directory not found: {prog_dir}"}

    print(f"  [{program}] Building...")
    if not build_program(prog_dir):
        return {"program": program, "error": "Build failed"}

    print(f"  [{program}] Running tests and generating coverage...")
    output = run_coverage(prog_dir)

    tests_total, tests_pass, tests_fail = parse_test_counts(output)
    print(f"  [{program}] Tests: {tests_pass}/{tests_total} passed", end="")
    if tests_fail > 0:
        print(f", {tests_fail} FAILED", end="")
    print()

    file_stats, total_covered, total_executable = parse_gcov_files(prog_dir)
    if total_executable == 0:
        return {"program": program, "error": "No coverage data generated"}

    total_pct = round(100 * total_covered / total_executable, 2)
    print(f"  [{program}] Coverage: {total_covered}/{total_executable} = {total_pct}%")

    main_cov = parse_main_source_coverage(prog_dir, program)

    return {
        "program": program,
        "tests_total": tests_total,
        "tests_pass": tests_pass,
        "tests_fail": tests_fail,
        "total_covered": total_covered,
        "total_executable": total_executable,
        "total_percent": total_pct,
        "main_source": {
            "file": f"{program}.c",
            **(main_cov if main_cov else {}),
        },
        "files": file_stats,
    }


def print_report(results: list):
    """Print a formatted coverage report table."""
    print()
    print("=" * 100)
    print("COREUTILS_ACTOR: Line Coverage Report")
    print(f"Source: {SRC_DIR}")
    print("=" * 100)
    print()
    print(
        f"{'Program':<12} | {'Tests':>5} {'Pass':>5} {'Fail':>5} | "
        f"{'Covered':>8} {'Total':>8} {'Cov%':>7} | "
        f"{'Main .c':>10} {'Main%':>7}"
    )
    print("-" * 100)

    sum_tests = sum_pass = sum_fail = 0
    sum_covered = sum_total = 0
    sum_main_cov = sum_main_total = 0

    for r in sorted(results, key=lambda x: x["program"]):
        if "error" in r:
            print(f"{r['program']:<12} | {'ERROR':>17} | {r['error']}")
            continue

        mc = r.get("main_source", {})
        mc_str = f"{mc.get('covered', '?')}/{mc.get('executable', '?')}"
        mc_pct = f"{mc['percent']}%" if "percent" in mc else "N/A"

        print(
            f"{r['program']:<12} | "
            f"{r['tests_total']:>5} {r['tests_pass']:>5} {r['tests_fail']:>5} | "
            f"{r['total_covered']:>8} {r['total_executable']:>8} {r['total_percent']:>6}% | "
            f"{mc_str:>10} {mc_pct:>7}"
        )

        sum_tests += r["tests_total"]
        sum_pass += r["tests_pass"]
        sum_fail += r["tests_fail"]
        sum_covered += r["total_covered"]
        sum_total += r["total_executable"]
        sum_main_cov += mc.get("covered", 0)
        sum_main_total += mc.get("executable", 0)

    print("-" * 100)

    if sum_total > 0:
        agg_pct = round(100 * sum_covered / sum_total, 2)
    else:
        agg_pct = 0
    if sum_main_total > 0:
        main_pct = f"{round(100 * sum_main_cov / sum_main_total, 2)}%"
        main_str = f"{sum_main_cov}/{sum_main_total}"
    else:
        main_pct = "N/A"
        main_str = "N/A"

    print(
        f"{'TOTAL':<12} | "
        f"{sum_tests:>5} {sum_pass:>5} {sum_fail:>5} | "
        f"{sum_covered:>8} {sum_total:>8} {agg_pct:>6}% | "
        f"{main_str:>10} {main_pct:>7}"
    )
    print()


def main():
    programs = sys.argv[1:] if len(sys.argv) > 1 else PROGRAMS

    # Validate program names
    for p in programs:
        if p not in PROGRAMS:
            print(f"Error: unknown program '{p}'. Must be one of: {PROGRAMS}")
            sys.exit(1)

    print()
    print("#" * 100)
    print("#" + " COREUTILS_ACTOR COVERAGE COLLECTION ".center(98) + "#")
    print("#" * 100)
    print()

    results = []
    for program in programs:
        print(f"[{program}]")
        try:
            r = collect_program_coverage(program)
        except subprocess.TimeoutExpired:
            r = {"program": program, "error": "Timed out"}
            print(f"  [{program}] TIMEOUT")
        except Exception as e:
            r = {"program": program, "error": str(e)}
            print(f"  [{program}] ERROR: {e}")
        results.append(r)
        print()

    print_report(results)

    # Write JSON output
    out_path = Path(__file__).parent / "collect_coverage.json"
    with open(out_path, "w") as f:
        json.dump(results, f, indent=2)
    print(f"Detailed results written to {out_path}")


if __name__ == "__main__":
    main()
