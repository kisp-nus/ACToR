#!/usr/bin/env python3
"""
Draw script for Experiment 4: Ablation Study on Configurations (Micro Benchmark, Absolute)
Creates two bar charts:
1. Different test configurations (15+1, 15+3, 15+5)
2. Iteration progression (0, 5, 10, 15, 20)
"""

import os
import json
import matplotlib.pyplot as plt
import numpy as np

plt.rcParams['font.family'] = 'serif'
plt.rcParams['font.serif'] = ['DejaVu Serif', 'Liberation Serif', 'Times', 'serif']

programs = [
    "printf",
    "expr",
    "fmt",
    "test",
    "join",
    "csplit",
]

iteration_num = 10

settings_configs = [
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR-15_1",
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR",
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR-15_5",
]

setting_names_configs = {
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR": "3 New Tests\nper Iter",
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR-15_1": "1 New Test\nper Iter",
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR-15_5": "5 New Tests\nper Iter",
}

iterations_to_test = [0, 5, 10, 15, 20]
setting_for_iteration = "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR"

working_dir = ".working/"

CACHE_FILE = "./__cache/absolute_comparison_cache.json"

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


def collect_result_instance():
    """Collect result instances from working directory."""
    result_instance = {}
    all_settings = settings_configs + [setting_for_iteration]
    
    for program in programs:
        result_instance[program] = {}
        for setting in all_settings:
            result_instance[program][setting] = []
            setting_parts = setting.split("+")
            if len(setting_parts) != 2:
                continue
            expected_translator, expected_discriminator = setting_parts
            
            if not os.path.exists(working_dir):
                continue
            for subfolder in os.listdir(working_dir):
                if program in subfolder:
                    folder_path = os.path.join(working_dir, subfolder)
                    state_file = os.path.join(folder_path, ".translation_state.json")
                    if os.path.isfile(state_file):
                        try:
                            with open(state_file, "r") as f:
                                state = json.load(f)
                                translator = state.get("translator", "")
                                discriminator = state.get("discriminator", "")
                                cur_iter = state.get("current_iteration", 0)
                                min_iter = max(iterations_to_test) + 1 if setting == setting_for_iteration else iteration_num + 1
                                if translator == expected_translator and discriminator == expected_discriminator and cur_iter >= min_iter:
                                    result_instance[program][setting].append(subfolder)
                        except Exception as e:
                            continue
    return result_instance


def extract_config_data(cache, result_instance):
    """Extract data for configuration comparison."""
    results = {}
    
    for setting in settings_configs:
        results[setting] = {'passed': 0, 'total': 0}
        
        for program in programs:
            instance = result_instance.get(program, {}).get(setting, None)
            if not instance:
                continue
            
            try:
                passed, total = cache[program][str(iteration_num)][instance]
                results[setting]['passed'] += passed
                results[setting]['total'] += total
            except KeyError:
                continue
        
        results[setting]['rate'] = results[setting]['passed'] / results[setting]['total'] * 100 if results[setting]['total'] > 0 else 0
    
    return results


def extract_iteration_data(cache, result_instance):
    """Extract data for iteration progression."""
    results = {}
    
    instance = result_instance.get(programs[0], {}).get(setting_for_iteration, None)
    
    for iter_num in iterations_to_test:
        results[iter_num] = {'passed': 0, 'total': 0}
        
        for program in programs:
            instance = result_instance.get(program, {}).get(setting_for_iteration, None)
            if not instance:
                continue
            
            try:
                passed, total = cache[program][str(iter_num)][instance]
                results[iter_num]['passed'] += passed
                results[iter_num]['total'] += total
            except KeyError:
                continue
        
        results[iter_num]['rate'] = results[iter_num]['passed'] / results[iter_num]['total'] * 100 if results[iter_num]['total'] > 0 else 0
    
    return results


def create_config_bar_chart(results, output_dir):
    """Create bar chart for configuration comparison."""
    fig, ax = plt.subplots(figsize=(9, 7))
    
    setting_labels = [setting_names_configs.get(s, s) for s in settings_configs]
    pass_rates = [results[s]['rate'] for s in settings_configs]
    passed_counts = [results[s]['passed'] for s in settings_configs]
    total_counts = [results[s]['total'] for s in settings_configs]
    
    x_pos = np.arange(len(setting_labels))
    bar_width = 0.35
    
    colors = ['#2ecc71', '#2ecc71', '#2ecc71']
    
    bars = ax.bar(x_pos, pass_rates, bar_width, 
                  color=colors, alpha=0.8, 
                  edgecolor='black', linewidth=1.5)
    
    for i, (rate, passed, total) in enumerate(zip(pass_rates, passed_counts, total_counts)):
        y_pos = rate + 0.5
        ax.text(x_pos[i], y_pos, f'{rate:.1f}%\n({passed}/{total})', 
               ha='center', va='bottom', fontsize=20, fontweight='bold')
    
    ax.set_facecolor('#FAFAFA')
    ax.grid(True, alpha=0.3, axis='y', linestyle='--')
    ax.set_axisbelow(True)
    
    ax.set_xlabel('#New Tests per Iter', fontsize=24, fontweight='bold')
    ax.set_ylabel('Pass Rate (%)', fontsize=24, fontweight='bold')
    ax.set_title('Pass Rate vs. #New Tests per Iter\n(15 Seed Tests, 10 Iters)', fontsize=26, fontweight='bold', pad=20)
    
    ax.set_xticks(x_pos)
    ax.set_xticklabels(setting_labels, fontsize=22)
    ax.set_xlim(-0.5, len(setting_labels) - 0.5)
    
    max_rate = max(pass_rates) if pass_rates else 100
    ax.set_ylim(75, min(max_rate + 15, 104))
    ax.tick_params(axis='y', labelsize=20)
    
    plt.tight_layout()
    output_path = os.path.join(output_dir, 'eval4a.png')
    plt.savefig(output_path, dpi=300, bbox_inches='tight')
    plt.close()
    
    print(f"[INFO] Saved configuration comparison chart to: {output_path}")


def create_iteration_bar_chart(results, output_dir):
    """Create bar chart for iteration progression."""
    fig, ax = plt.subplots(figsize=(12, 7))
    
    setting_labels = [f"Initial (0 Iter)" if i == 0 else f"{i} Iters" for i in iterations_to_test]
    pass_rates = [results[i]['rate'] for i in iterations_to_test]
    passed_counts = [results[i]['passed'] for i in iterations_to_test]
    total_counts = [results[i]['total'] for i in iterations_to_test]
    
    x_pos = np.arange(len(setting_labels))
    bar_width = 0.4
    
    bars = ax.bar(x_pos, pass_rates, bar_width, 
                  color='#2ecc71', alpha=0.8, 
                  edgecolor='black', linewidth=1.5)
    
    for i, (rate, passed, total) in enumerate(zip(pass_rates, passed_counts, total_counts)):
        y_pos = rate + 0.5
        ax.text(x_pos[i], y_pos, f'{rate:.1f}%\n({passed}/{total})', 
               ha='center', va='bottom', fontsize=20, fontweight='bold')
    
    ax.set_facecolor('#FAFAFA')
    ax.grid(True, alpha=0.3, axis='y', linestyle='--')
    ax.set_axisbelow(True)
    
    ax.set_xlabel('#Iterations', fontsize=24, fontweight='bold')
    ax.set_ylabel('Pass Rate (%)', fontsize=24, fontweight='bold')
    ax.set_title('Pass Rate vs. #Iterations\n(15 Seed Tests, 3 New Tests per Iter)', fontsize=26, fontweight='bold', pad=20)
    
    ax.set_xticks(x_pos)
    ax.set_xticklabels(setting_labels, fontsize=22)
    ax.set_xlim(-0.5, len(setting_labels) - 0.5)
    
    max_rate = max(pass_rates) if pass_rates else 100
    ax.set_ylim(75, min(max_rate + 15, 104))
    ax.tick_params(axis='y', labelsize=20)
    
    plt.tight_layout()
    output_path = os.path.join(output_dir, 'eval4b.png')
    plt.savefig(output_path, dpi=300, bbox_inches='tight')
    plt.close()
    
    print(f"[INFO] Saved iteration progression chart to: {output_path}")


def main():
    """Main function."""
    cache = load_cache(CACHE_FILE)
    if not cache:
        print("[ERROR] No cache data found. Please run eval4.py first.")
        return
    
    result_instance = collect_result_instance()
    
    # Flatten instance lists
    for program in programs:
        for setting in list(result_instance[program].keys()):
            if result_instance[program][setting]:
                result_instance[program][setting] = result_instance[program][setting][0]
    
    output_dir = './scripts/figures/'
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)
    
    config_results = extract_config_data(cache, result_instance)
    iter_results = extract_iteration_data(cache, result_instance)
    
    create_config_bar_chart(config_results, output_dir)
    create_iteration_bar_chart(iter_results, output_dir)
    
    print("\nBar charts generated!")


if __name__ == "__main__":
    main()

