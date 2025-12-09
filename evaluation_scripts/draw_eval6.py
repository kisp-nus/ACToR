#!/usr/bin/env python3
"""
Draw script for Experiment 6: Macro Experiment (Macro Benchmark, Relative)
Creates horizontal stacked bar charts showing cross-validation results
"""

import json
import matplotlib.pyplot as plt
import numpy as np
import os

plt.rcParams['font.family'] = 'serif'
plt.rcParams['font.serif'] = ['DejaVu Serif', 'Liberation Serif', 'Times', 'serif']

programs = [
    "arch", "basename", "cat", "chmod", "comm", "cp", "csplit", "cut",
    "date", "df", "dirname", "du", "echo", "env", "expand", "expr",
    "factor", "fmt", "fold", "head", "id", "join", "ln", "logname",
    "ls", "mkdir", "mknod", "mktemp", "nice", "nl", "paste", "pathchk",
    "pr", "printenv", "printf", "pwd", "readlink", "realpath", "sleep",
    "sort", "split", "stat", "sync", "tail", "tee", "test", "touch",
    "tr", "tsort", "tty", "uname", "unexpand", "uniq", "users", "wc",
    "who", "xargs"
]

iteration_num = 10

settings = [
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR",
    "CC-Sonnet-4.5+CC-Sonnet-4.5-Coverage",
]

setting_names = {
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR": "ACToR",
    "CC-Sonnet-4.5+CC-Sonnet-4.5-Coverage": "Coverage",
}

working_dir = ".working_BSD/"

CACHE_FILE = "./__cache/cross_comparison_cache_macro.json"


def load_cache(cache_file):
    """Load cached results from JSON file."""
    if os.path.exists(cache_file):
        try:
            with open(cache_file, 'r') as f:
                cache = json.load(f)
            print(f"[INFO] Loaded cache from {cache_file}")
            return cache
        except (json.JSONDecodeError, IOError) as e:
            print(f"[WARNING] Failed to load cache: {e}")
            return {}
    else:
        print(f"[INFO] No cache file found")
        return {}


def load_benchmark_stats():
    """Load benchmark stats containing LOC information."""
    try:
        with open('./evaluation_scripts/benchmark_stats.json', 'r') as f:
            return json.load(f)
    except FileNotFoundError:
        print("WARNING: benchmark_stats.json not found, LOC info will not be available")
        return {}


def extract_results(cache):
    """Extract cross-validation results from cache."""
    results = {}
    
    actor_setting = settings[0]
    cov_setting = settings[1]
    
    for program in programs:
        results[program] = {
            actor_setting: {cov_setting: {'passed': 0, 'failed': 0, 'total': 0}},
            cov_setting: {actor_setting: {'passed': 0, 'failed': 0, 'total': 0}}
        }
        
        try:
            # ACToR vs Coverage
            defense, attack = cache[program][str(iteration_num)][actor_setting][cov_setting]
            results[program][actor_setting][cov_setting]['passed'] = defense
            results[program][actor_setting][cov_setting]['failed'] = attack
            results[program][actor_setting][cov_setting]['total'] = defense + attack
            
            # Coverage vs ACToR
            defense, attack = cache[program][str(iteration_num)][cov_setting][actor_setting]
            results[program][cov_setting][actor_setting]['passed'] = defense
            results[program][cov_setting][actor_setting]['failed'] = attack
            results[program][cov_setting][actor_setting]['total'] = defense + attack
        except KeyError:
            pass
    
    return results


def create_visualization(results, benchmark_stats, output_dir):
    """Create symmetric horizontal bar chart visualization."""
    
    actor_setting = settings[0]
    cov_setting = settings[1]
    
    # Sort programs by ACToR passed tests (descending)
    def sort_key(program):
        prog_results = results[program]
        actor_on_cov = prog_results.get(actor_setting, {}).get(cov_setting, {})
        return -actor_on_cov.get('passed', 0)
    
    # Print totals
    actor_total = sum(results[p][actor_setting][cov_setting]['total'] for p in results)
    actor_passed = sum(results[p][actor_setting][cov_setting]['passed'] for p in results)
    cov_total = sum(results[p][cov_setting][actor_setting]['total'] for p in results)
    cov_passed = sum(results[p][cov_setting][actor_setting]['passed'] for p in results)
    
    print(f"ACToR: {actor_passed}/{actor_total} = {actor_passed/actor_total:.4f}" if actor_total > 0 else "ACToR: N/A")
    print(f"Coverage: {cov_passed}/{cov_total} = {cov_passed/cov_total:.4f}" if cov_total > 0 else "Coverage: N/A")
    
    # Normalize to percentages
    for program in results:
        for src in results[program]:
            for tgt in results[program][src]:
                total = results[program][src][tgt]['total']
                if total > 0:
                    results[program][src][tgt]['passed'] = results[program][src][tgt]['passed'] / total
                    results[program][src][tgt]['failed'] = results[program][src][tgt]['failed'] / total
                    results[program][src][tgt]['total'] = 1
    
    sorted_programs = sorted(results.keys(), key=sort_key)
    
    # Split into three chunks
    programs_chunk1 = sorted_programs[:20]
    programs_chunk2 = sorted_programs[20:40]
    programs_chunk3 = sorted_programs[40:60]
    
    fig = plt.figure(figsize=(16, max(6, 20 * 0.4)))
    
    ax1 = plt.subplot2grid((25, 3), (0, 0), rowspan=25, colspan=1)
    ax2 = plt.subplot2grid((25, 3), (0, 1), rowspan=25, colspan=1)  
    ax3 = plt.subplot2grid((25, 3), (0, 2), rowspan=21, colspan=1)
    
    colors = {
        'passed': '#2ecc71',
        'failed': '#e74c3c',
    }
    
    def create_bars_for_chunk(ax, programs_chunk, chunk_start_idx):
        y = np.arange(len(programs_chunk))
        height = 0.6
        scale_max = 45
        center_gap = 1.5
        
        for i, program in enumerate(programs_chunk):
            prog_results = results[program]
            
            # ACToR on Coverage (left side)
            actor_on_cov = prog_results.get(actor_setting, {}).get(cov_setting, {'passed': 0, 'failed': 0, 'total': 0})
            actor_total = actor_on_cov.get('total', 0)
            
            if actor_total > 0:
                pass_rate = actor_on_cov['passed']
                available_width = scale_max - center_gap/2
                passed_width = pass_rate * available_width
                failed_width = (1 - pass_rate) * available_width
                
                ax.barh(y[i], passed_width, height, 
                       left=-scale_max, color=colors['passed'], 
                       edgecolor='black', linewidth=0.5)
                ax.barh(y[i], failed_width, height, 
                       left=-scale_max + passed_width, color=colors['failed'], 
                       edgecolor='black', linewidth=0.5)
            
            # Coverage on ACToR (right side)
            cov_on_actor = prog_results.get(cov_setting, {}).get(actor_setting, {'passed': 0, 'failed': 0, 'total': 0})
            cov_total = cov_on_actor.get('total', 0)
            
            if cov_total > 0:
                pass_rate = cov_on_actor['passed']
                available_width = scale_max - center_gap/2
                passed_width = pass_rate * available_width
                failed_width = (1 - pass_rate) * available_width
                
                ax.barh(y[i], passed_width, height, 
                       left=scale_max - passed_width, color=colors['passed'], 
                       edgecolor='black', linewidth=0.5)
                ax.barh(y[i], failed_width, height, 
                       left=scale_max - passed_width - failed_width, color=colors['failed'], 
                       edgecolor='black', linewidth=0.5)
        
        ax.axvline(x=0, color='black', linestyle='-', linewidth=2, alpha=0.8)
        
        if chunk_start_idx == 0:
            ax.set_ylabel('Programs', fontsize=18, fontweight='bold')
        ax.set_xlabel('ACToR (left) vs. Coverage (right)', fontsize=16, fontweight='bold')
        
        title = 'Cross Comparison'
        if chunk_start_idx > 0:
            title += ' (continued)'
        ax.set_title(title, fontsize=18, fontweight='bold', pad=20)
        
        ax.set_xlim(-scale_max, scale_max)
        ticks = [-45, -30, -15, 0, 15, 30, 45]
        ax.set_xticks(ticks)
        tick_labels = ['0%', '33%', '67%', '100%', '67%', '33%', '0%']
        ax.set_xticklabels(tick_labels, fontsize=14)
        
        ax.set_yticks(y)
        program_labels = []
        for program in programs_chunk:
            loc = benchmark_stats.get(program, {}).get('loc', '?')
            program_labels.append(f"{program} ({loc})")
        ax.set_yticklabels(program_labels, fontsize=15)
        
        ax.grid(True, alpha=0.3, axis='x', linestyle='--')
        ax.set_axisbelow(True)
        ax.invert_yaxis()
        ax.set_ylim(len(programs_chunk) - 0.5, -0.5)
    
    create_bars_for_chunk(ax1, programs_chunk1, 0)
    create_bars_for_chunk(ax2, programs_chunk2, 20)
    create_bars_for_chunk(ax3, programs_chunk3, 40)

    from matplotlib.patches import Patch
    legend_elements = [
        Patch(facecolor=colors['passed'], label='Passed Tests', edgecolor='black'),
        Patch(facecolor=colors['failed'], label='Failed Tests', edgecolor='black'),
    ]
    fig.legend(handles=legend_elements, loc='lower right', ncol=2, fontsize=16, bbox_to_anchor=(0.95, 0.105))

    plt.subplots_adjust(left=0.08, right=0.92, top=0.95, bottom=0.15, wspace=0.52)
    
    output_path = os.path.join(output_dir, 'eval6.png')
    plt.savefig(output_path, dpi=300, bbox_inches='tight')
    plt.close()
    
    print(f"[INFO] Saved macro experiment figure to: {output_path}")


def main():
    """Main function."""
    cache = load_cache(CACHE_FILE)
    if not cache:
        print("[ERROR] No cache data found. Please run eval6.py first.")
        return
    
    benchmark_stats = load_benchmark_stats()
    
    output_dir = './scripts/figures/'
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)
    
    results = extract_results(cache)
    
    if not results:
        print("ERROR: No results found for any programs")
        return
    
    create_visualization(results, benchmark_stats, output_dir)
    
    print("\nMacro experiment visualization complete!")


if __name__ == "__main__":
    main()

