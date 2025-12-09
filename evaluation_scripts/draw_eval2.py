#!/usr/bin/env python3
"""
Draw script for Experiment 2: Ablation Study on Design (Micro Benchmark, Relative)
Creates a 3x3 heatmap showing cross-comparison results
"""

import os
import json
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns

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

settings = [
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR",
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR-noFuzz",
    "CC-Sonnet-4.5+CC-Sonnet-4.5-Coverage",
]

setting_names = {
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR": "ACToR",
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR-noFuzz": "ACToR\nNoFuzz",
    "CC-Sonnet-4.5+CC-Sonnet-4.5-Coverage": "Coverage\nBaseline",
}

working_dir = ".working/"

CACHE_FILE = "./__cache/cross_comparison_cache_eval2.json"

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
    for program in programs:
        result_instance[program] = {}
        for setting in settings:
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
                                if translator == expected_translator and discriminator == expected_discriminator and cur_iter == iteration_num + 1:
                                    result_instance[program][setting].append(subfolder)
                        except Exception as e:
                            continue
    return result_instance


def cross_comparison_heatmap(cache, result_instance):
    """Generate heatmap data from cache."""
    iter_num = iteration_num
    
    heatmap_data = {}
    raw_data = {}
    
    for test_setting in settings:
        heatmap_data[test_setting] = {}
        raw_data[test_setting] = {}
        for translation_setting in settings:
            heatmap_data[test_setting][translation_setting] = 0
            raw_data[test_setting][translation_setting] = {'passed': 0, 'total': 0}
            for program in programs:
                attack_instance = result_instance[program][test_setting]
                defense_instance = result_instance[program][translation_setting]

                try:
                    cached_result = cache[program][str(iter_num)][defense_instance][attack_instance]
                    defense_success, defense_total, attack_success, attack_total = cached_result
                    raw_data[test_setting][translation_setting]['passed'] += defense_success
                    raw_data[test_setting][translation_setting]['total'] += defense_total
                except KeyError:
                    print(f"[WARNING] Missing cache for {program} {defense_instance} {attack_instance}")
                    continue
            
            if raw_data[test_setting][translation_setting]['total'] > 0:
                heatmap_data[test_setting][translation_setting] = raw_data[test_setting][translation_setting]['passed'] / raw_data[test_setting][translation_setting]['total']
    
    return heatmap_data, raw_data


def create_heatmap(heatmap_data, raw_data, output_dir):
    """Create and save a 3x3 heatmap visualization."""
    matrix = np.zeros((len(settings), len(settings)))
    
    for i, translation_setting in enumerate(settings):
        for j, test_setting in enumerate(settings):
            matrix[i, j] = heatmap_data[test_setting][translation_setting]
    
    plt.figure(figsize=(10, 7))
    
    short_labels = [setting_names.get(s, s) for s in settings]
    
    from matplotlib.colors import LinearSegmentedColormap
    colors = ['#d73027', '#f46d43', '#fdae61', '#fee08b', '#e6f598', '#abdda4', '#66c2a5']
    n_bins = 100
    beautiful_cmap = LinearSegmentedColormap.from_list('green_to_red', colors, N=n_bins)
    
    ax = sns.heatmap(
        matrix, 
        annot=True, 
        fmt='.2f', 
        cmap=beautiful_cmap,
        xticklabels=short_labels,
        yticklabels=short_labels,
        cbar_kws={'label': 'Relative Pass Rate', 'shrink': 0.8, 'pad': 0.02},
        square=True,
        linewidths=2,
        linecolor='white',
        annot_kws={'fontsize': 18, 'fontweight': 'bold'},
        vmin=0.5,
        vmax=1
    )
    
    cbar = ax.collections[0].colorbar
    cbar.ax.tick_params(labelsize=14)
    cbar.set_label('Relative Pass Rate', fontsize=16, fontweight='bold')
    
    for i, translation_setting in enumerate(settings):
        for j, test_setting in enumerate(settings):
            passed = raw_data[test_setting][translation_setting]['passed']
            total = raw_data[test_setting][translation_setting]['total']
            if total > 0:
                ax.text(j + 0.5, i + 0.75, f'({passed}/{total})', 
                       ha='center', va='center', fontsize=14, color='black', fontweight='bold')
    
    plt.xlabel('Test Method', fontsize=16, fontweight='bold', labelpad=10)
    plt.ylabel('Translation Method', fontsize=16, fontweight='bold', labelpad=10)
    
    plt.xticks(rotation=0, fontsize=14)
    plt.yticks(rotation=0, fontsize=14)
    
    plt.subplots_adjust(left=0.15, right=0.85, top=0.9, bottom=0.2)
    plt.tight_layout()
    
    output_filename = os.path.join(output_dir, 'eval2.png')
    plt.savefig(output_filename, dpi=300, bbox_inches='tight', facecolor='white', pad_inches=0.1)
    print(f"\n[INFO] Heatmap saved as {output_filename}")
    
    plt.close()


def main():
    """Main function."""
    cache = load_cache(CACHE_FILE)
    if not cache:
        print("[ERROR] No cache data found. Please run eval2.py first.")
        return
    
    result_instance = collect_result_instance()
    
    # Flatten instance lists
    for program in programs:
        for setting in settings:
            if result_instance[program][setting]:
                result_instance[program][setting] = result_instance[program][setting][0]
            else:
                print(f"[WARNING] No instance found for {program} {setting}")
    
    output_dir = './scripts/figures/'
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)
    
    heatmap_data, raw_data = cross_comparison_heatmap(cache, result_instance)
    create_heatmap(heatmap_data, raw_data, output_dir)
    
    print("\nHeatmap Generation Complete!")


if __name__ == "__main__":
    main()

