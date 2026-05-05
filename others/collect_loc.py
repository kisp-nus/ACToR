#!/usr/bin/env python3
"""
Collect lines of code (LOC) statistics for C programs and their Rust translations.
Uses 'cloc' for accurate LOC counting (excluding comments).
Supports two benchmark directories:
  - c2saferrust/coreutils/src: C source in prog/c/, Rust in prog/rust_WIP/
  - c2saferrust/laertes_benchmarks: Original Rust in prog/, Translation in prog_WIP/
"""

import json
import re
import subprocess
from pathlib import Path
from typing import Dict, Optional

BASE_DIR = Path(__file__).parent / "c2saferrust"
COREUTILS_DIR = BASE_DIR / "coreutils" / "src"
LAERTES_DIR = BASE_DIR / "laertes_benchmarks"


def run_cloc(dir_path: Path) -> Optional[Dict]:
    """
    Run cloc on a directory and return parsed JSON results.
    Returns dict with language stats: {lang: {files, blank, comment, code}}
    """
    if not dir_path.exists():
        return None
    
    try:
        result = subprocess.run(
            ['cloc', '--json', str(dir_path)],
            capture_output=True,
            text=True,
            timeout=60
        )
        if result.returncode != 0:
            return None
        
        data = json.loads(result.stdout)
        return data
    except (subprocess.TimeoutExpired, json.JSONDecodeError, FileNotFoundError) as e:
        print(f"Error running cloc on {dir_path}: {e}")
        return None


def extract_cloc_stats(cloc_data: Optional[Dict], languages: list) -> Dict:
    """Extract stats for specified languages from cloc output."""
    result = {}
    if not cloc_data:
        return result
    
    for lang in languages:
        if lang in cloc_data:
            result[lang] = {
                'files': cloc_data[lang].get('nFiles', 0),
                'blank': cloc_data[lang].get('blank', 0),
                'comment': cloc_data[lang].get('comment', 0),
                'code': cloc_data[lang].get('code', 0),
            }
    return result


def count_unsafe_in_dir(dir_path: Path) -> int:
    """Count occurrences of 'unsafe' keyword in all .rs files in a directory."""
    if not dir_path.exists():
        return 0
    
    total = 0
    for file_path in dir_path.glob("*.rs"):
        if file_path.is_file():
            try:
                with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                    content = f.read()
                    total += len(re.findall(r'\bunsafe\b', content))
            except Exception:
                pass
    return total


def collect_coreutils_stats() -> Dict:
    """
    Collect LOC stats for coreutils programs using cloc.
    Structure: prog/c/*.c,*.h, prog/rust_WIP/*.rs
    """
    if not COREUTILS_DIR.exists():
        return {}
    
    all_stats = {}
    programs = sorted([d for d in COREUTILS_DIR.iterdir() if d.is_dir()])
    
    for program_dir in programs:
        program_name = program_dir.name
        c_dir = program_dir / "c"
        rust_dir = program_dir / "rust_WIP"
        
        c_cloc = run_cloc(c_dir)
        rust_cloc = run_cloc(rust_dir)
        
        c_stats = extract_cloc_stats(c_cloc, ['C', 'C/C++ Header'])
        rust_stats = extract_cloc_stats(rust_cloc, ['Rust'])
        
        rust_unsafe = count_unsafe_in_dir(rust_dir)
        
        all_stats[program_name] = {
            'c': c_stats.get('C', {'files': 0, 'blank': 0, 'comment': 0, 'code': 0}),
            'header': c_stats.get('C/C++ Header', {'files': 0, 'blank': 0, 'comment': 0, 'code': 0}),
            'rust': rust_stats.get('Rust', {'files': 0, 'blank': 0, 'comment': 0, 'code': 0}),
            'rust_unsafe': rust_unsafe,
        }
    
    return all_stats


def collect_laertes_stats() -> Dict:
    """
    Collect LOC stats for laertes benchmarks using cloc.
    Structure: prog/*.rs (original c2rust output), prog_WIP/*.rs (translation)
    """
    if not LAERTES_DIR.exists():
        return {}
    
    all_stats = {}
    
    # Find base program names (directories without _WIP, _laertes, _crown suffix)
    all_dirs = [d.name for d in LAERTES_DIR.iterdir() if d.is_dir()]
    program_names = sorted(set(
        d.replace('_WIP', '').replace('_laertes', '').replace('_crown', '')
        for d in all_dirs
    ))
    
    for program_name in program_names:
        orig_dir = LAERTES_DIR / program_name
        wip_dir = LAERTES_DIR / f"{program_name}_WIP"
        
        if not orig_dir.exists():
            continue
        
        orig_cloc = run_cloc(orig_dir)
        wip_cloc = run_cloc(wip_dir) if wip_dir.exists() else None
        
        orig_stats = extract_cloc_stats(orig_cloc, ['Rust'])
        wip_stats = extract_cloc_stats(wip_cloc, ['Rust'])
        
        orig_unsafe = count_unsafe_in_dir(orig_dir)
        wip_unsafe = count_unsafe_in_dir(wip_dir) if wip_dir.exists() else 0
        
        all_stats[program_name] = {
            'original': orig_stats.get('Rust', {'files': 0, 'blank': 0, 'comment': 0, 'code': 0}),
            'original_unsafe': orig_unsafe,
            'wip': wip_stats.get('Rust', {'files': 0, 'blank': 0, 'comment': 0, 'code': 0}),
            'wip_unsafe': wip_unsafe,
        }
    
    return all_stats


def print_coreutils_report(stats: Dict):
    """Print report for coreutils benchmarks."""
    print("=" * 140)
    print("COREUTILS: LOC Statistics for C Programs and Rust Translations (using cloc)")
    print(f"Source: {COREUTILS_DIR}")
    print("=" * 140)
    print()
    
    print(f"{'Program':<12} | {'C Files':>7} | {'C Code':>8} | {'C Cmt':>7} | "
          f"{'H Files':>7} | {'H Code':>8} | {'H Cmt':>7} | "
          f"{'Rs Files':>8} | {'Rs Code':>8} | {'Rs Cmt':>7} | {'Unsafe':>7}")
    print("-" * 140)
    
    totals = {
        'c_files': 0, 'c_code': 0, 'c_comment': 0,
        'h_files': 0, 'h_code': 0, 'h_comment': 0,
        'rust_files': 0, 'rust_code': 0, 'rust_comment': 0, 'unsafe': 0
    }
    
    for program_name, data in sorted(stats.items()):
        c = data['c']
        h = data['header']
        r = data['rust']
        unsafe = data['rust_unsafe']
        
        print(f"{program_name:<12} | {c['files']:>7} | {c['code']:>8} | {c['comment']:>7} | "
              f"{h['files']:>7} | {h['code']:>8} | {h['comment']:>7} | "
              f"{r['files']:>8} | {r['code']:>8} | {r['comment']:>7} | {unsafe:>7}")
        
        totals['c_files'] += c['files']
        totals['c_code'] += c['code']
        totals['c_comment'] += c['comment']
        totals['h_files'] += h['files']
        totals['h_code'] += h['code']
        totals['h_comment'] += h['comment']
        totals['rust_files'] += r['files']
        totals['rust_code'] += r['code']
        totals['rust_comment'] += r['comment']
        totals['unsafe'] += unsafe
    
    print("-" * 140)
    print(f"{'TOTAL':<12} | {totals['c_files']:>7} | {totals['c_code']:>8} | {totals['c_comment']:>7} | "
          f"{totals['h_files']:>7} | {totals['h_code']:>8} | {totals['h_comment']:>7} | "
          f"{totals['rust_files']:>8} | {totals['rust_code']:>8} | {totals['rust_comment']:>7} | {totals['unsafe']:>7}")
    print()


def print_laertes_report(stats: Dict):
    """Print report for laertes benchmarks."""
    print("=" * 130)
    print("LAERTES BENCHMARKS: LOC Statistics for Original (c2rust) and Translated (WIP) Rust (using cloc)")
    print(f"Source: {LAERTES_DIR}")
    print("=" * 130)
    print()
    
    print(f"{'Program':<18} | {'Orig Files':>10} | {'Orig Code':>10} | {'Orig Cmt':>9} | {'Orig Unsafe':>11} | "
          f"{'WIP Files':>9} | {'WIP Code':>9} | {'WIP Cmt':>8} | {'WIP Unsafe':>10} | {'Unsafe Δ':>9}")
    print("-" * 130)
    
    totals = {
        'orig_files': 0, 'orig_code': 0, 'orig_comment': 0, 'orig_unsafe': 0,
        'wip_files': 0, 'wip_code': 0, 'wip_comment': 0, 'wip_unsafe': 0
    }
    
    for program_name, data in sorted(stats.items()):
        orig = data['original']
        wip = data['wip']
        orig_unsafe = data['original_unsafe']
        wip_unsafe = data['wip_unsafe']
        
        unsafe_delta = wip_unsafe - orig_unsafe
        delta_str = f"{unsafe_delta:+d}" if wip['files'] > 0 else "N/A"
        
        print(f"{program_name:<18} | {orig['files']:>10} | {orig['code']:>10} | {orig['comment']:>9} | {orig_unsafe:>11} | "
              f"{wip['files']:>9} | {wip['code']:>9} | {wip['comment']:>8} | {wip_unsafe:>10} | {delta_str:>9}")
        
        totals['orig_files'] += orig['files']
        totals['orig_code'] += orig['code']
        totals['orig_comment'] += orig['comment']
        totals['orig_unsafe'] += orig_unsafe
        totals['wip_files'] += wip['files']
        totals['wip_code'] += wip['code']
        totals['wip_comment'] += wip['comment']
        totals['wip_unsafe'] += wip_unsafe
    
    print("-" * 130)
    total_delta = totals['wip_unsafe'] - totals['orig_unsafe']
    print(f"{'TOTAL':<18} | {totals['orig_files']:>10} | {totals['orig_code']:>10} | {totals['orig_comment']:>9} | {totals['orig_unsafe']:>11} | "
          f"{totals['wip_files']:>9} | {totals['wip_code']:>9} | {totals['wip_comment']:>8} | {totals['wip_unsafe']:>10} | {total_delta:>+9}")
    print()


def main():
    print("\n" + "#" * 140)
    print("#" + " " * 55 + "LOC COLLECTION REPORT (cloc)" + " " * 55 + "#")
    print("#" * 140 + "\n")
    
    # Collect both benchmarks
    coreutils_stats = collect_coreutils_stats()
    laertes_stats = collect_laertes_stats()
    
    # Print summary reports
    if coreutils_stats:
        print_coreutils_report(coreutils_stats)
    else:
        print(f"Coreutils directory not found: {COREUTILS_DIR}\n")
    
    if laertes_stats:
        print_laertes_report(laertes_stats)
    else:
        print(f"Laertes benchmarks directory not found: {LAERTES_DIR}\n")


if __name__ == "__main__":
    main()
