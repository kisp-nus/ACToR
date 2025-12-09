#!/usr/bin/env python3

import os
import json
import matplotlib.pyplot as plt
import numpy as np
from pathlib import Path

# Set font to serif (Times-like appearance) - matching plot_v2.py style
plt.rcParams['font.family'] = 'serif'
plt.rcParams['font.serif'] = ['DejaVu Serif', 'Liberation Serif', 'Times', 'serif']

# Configuration from eval1.py
programs = [
    "printf",
    "expr", 
    "fmt",
    "test",
    "join",
    "csplit",
]

iteration_num = 10

### settings = ["Translator+Discriminator", ...]
settings = [
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR",
    # "CC-Sonnet-4+CC-Sonnet-4-ACToR", ### not supported in the latest Claude Code version
    "SWE-Sonnet-4.5+SWE-Sonnet-4.5-ACToR",
    "SWE-Sonnet-4+SWE-Sonnet-4-ACToR",
    "SWE-GPT-5mini+SWE-GPT-5mini-ACToR",
]

model_names = {
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR": "Claude Code + Claude-Sonnet-4.5",
    "SWE-Sonnet-4.5+SWE-Sonnet-4.5-ACToR": "Mini-SWE-Agent + Claude-Sonnet-4.5",
    "SWE-Sonnet-4+SWE-Sonnet-4-ACToR": "Mini-SWE-Agent + Claude-Sonnet-4",
    "SWE-GPT-5mini+SWE-GPT-5mini-ACToR": "Mini-SWE-Agent + GPT-5 mini",
}

setting_names = {
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR": "@ClaudeCode+Sonnet4.5",
    "SWE-Sonnet-4.5+SWE-Sonnet-4.5-ACToR": "@SWE+Sonnet4.5",
    "SWE-Sonnet-4+SWE-Sonnet-4-ACToR": "@SWE+Sonnet4",
    "SWE-GPT-5mini+SWE-GPT-5mini-ACToR": "@SWE+GPT5mini",
}

working_dir = ".working/"

# Single cache file from eval1.py
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
    """Collect result instances from working directory (same as eval1.py)."""
    result_instance = {}
    for program in programs:
        result_instance[program] = {}
        for setting in settings:
            result_instance[program][setting] = []
            # Parse setting string "Translator+Discriminator"
            setting_parts = setting.split("+")
            if len(setting_parts) != 2:
                continue
            expected_translator, expected_discriminator = setting_parts
            
            # Find the folder in .working/{program} whose .translation_state.json's discriminator matches the setting
            if not os.path.exists(working_dir):
                continue
            for subfolder in os.listdir(working_dir):
                if program in subfolder:  # match prefix of the folder name
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


def extract_program_wise_data_from_cache(cache, result_instance):
    """Extract program-wise data for baseline (iter 0) and final (iter 10) results."""
    results = {}
    
    for setting in settings:
        results[setting] = {
            'programs': [],
            'baseline_rates': [],  # iter 0
            'final_rates': [],     # iter 10
            'baseline_counts': [], # [passed, total] for iter 0
            'final_counts': []     # [passed, total] for iter 10
        }
        
        for program in programs:
            try:
                # Get the instance name for this program and setting
                instances = result_instance.get(program, {}).get(setting, [])
                if not instances:
                    print(f"[WARNING] No instance found for {program}, setting {setting}")
                    continue
                
                instance = instances[0]  # Use the first instance if multiple exist
                
                # Get baseline data (iter 0)
                baseline_pass, baseline_total = cache[program]["0"][instance]
                baseline_rate = (baseline_pass / baseline_total) * 100 if baseline_total > 0 else 0
                
                # Get final data (iter 10)
                final_pass, final_total = cache[program]["10"][instance]
                final_rate = (final_pass / final_total) * 100 if final_total > 0 else 0
                
                results[setting]['programs'].append(program)
                results[setting]['baseline_rates'].append(baseline_rate)
                results[setting]['final_rates'].append(final_rate)
                results[setting]['baseline_counts'].append([baseline_pass, baseline_total])
                results[setting]['final_counts'].append([final_pass, final_total])
                    
            except Exception as e:
                print(f"[WARNING] No data found for {program}, setting {setting}: {e}")
                continue
    
    return results


def main():
    """Main function to generate setting curves in subplots."""
    
    # Load cache data
    cache = load_cache(CACHE_FILE)
    if not cache:
        print("[ERROR] No cache data found. Please run eval1.py first.")
        return
    
    # Collect result instances
    result_instance = collect_result_instance()
    
    # Extract program-wise data from cache
    results = extract_program_wise_data_from_cache(cache, result_instance)
    
    # Create output directory
    output_dir = './scripts/figures/'
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)
    
    print("Drawing program-wise bar charts for different settings in subplots...")
    
    # Create combined figure with subplots
    create_combined_figure(results, output_dir)
    
    print(f"\nCombined bar charts have been generated and saved to: {output_dir}")

def create_combined_figure(results, output_dir):
    """Create a combined figure with subplots showing program-wise bar charts."""
    
    num_settings = len(settings)
    # Calculate grid dimensions: 2 rows, enough columns for settings + legend
    ncols = 3
    nrows = 2
    
    # Create figure with subplots (2 rows, 3 columns)
    fig, axes = plt.subplots(nrows, ncols, figsize=(30, 15))
    axes = axes.flatten()  # Flatten 2D array to 1D for easier indexing
    
    # Professional color scheme
    colors = {
        'baseline': '#87ceeb',  # Light blue for baseline (iter 0)
        'final': '#27ae60',     # Green for final (iter 10)
    }
    
    for i, setting in enumerate(settings):
        ax = axes[i]
        
        if setting not in results or not results[setting]['programs']:
            # Create empty subplot if no data
            ax.text(0.5, 0.5, f'No data for\n{setting_names.get(setting, str(setting))}', 
                   ha='center', va='center', transform=ax.transAxes, fontsize=26)
            ax.set_title(setting_names.get(setting, str(setting)), fontsize=28, fontweight='bold')
            continue
            
        programs_list = results[setting]['programs']
        baseline_rates = results[setting]['baseline_rates']
        final_rates = results[setting]['final_rates']
        baseline_counts = results[setting]['baseline_counts']
        final_counts = results[setting]['final_counts']
        
        setting_name = setting_names.get(setting, str(setting))
        model_name = model_names.get(setting, str(setting))
        
        # Create bar positions
        x_pos = np.arange(len(programs_list))
        bar_width = 0.35
        
        # Create vertical bars for each program (horizontal layout)
        bars1 = ax.bar(x_pos - bar_width/2, baseline_rates, bar_width, 
                       color=colors['baseline'], alpha=0.8, 
                       edgecolor='black', linewidth=0.5,
                       label=f'Baseline{setting_name}')
        bars2 = ax.bar(x_pos + bar_width/2, final_rates, bar_width, 
                       color=colors['final'], alpha=0.8,
                       edgecolor='black', linewidth=0.5, 
                       label=f'ACToR{setting_name} (Iter 10)')
        
        # Add value labels on bars with advanced overlap prevention
        all_text_positions = []  # Store all text positions to avoid overlaps
        
        for j, (baseline_rate, final_rate, baseline_count, final_count) in enumerate(zip(
                baseline_rates, final_rates, baseline_counts, final_counts)):
            
            # Calculate text positions for both bars
            baseline_x = x_pos[j] - bar_width/2
            final_x = x_pos[j] + bar_width/2
            
            # Start with default positions (above bars)
            baseline_y = baseline_rate + 2
            final_y = final_rate + 2
            baseline_va = 'bottom'
            final_va = 'bottom'
            
            # Function to check if two text positions overlap
            def texts_overlap(x1, y1, x2, y2, threshold_x=0.6, threshold_y=5.0):
                return abs(x1 - x2) < threshold_x and abs(y1 - y2) < threshold_y
            
            # Get current program name
            program_name = programs_list[j]
            
            # Check overlap between baseline and final of same program
            if texts_overlap(baseline_x, baseline_y, final_x, final_y):
                if baseline_rate <= final_rate:
                    baseline_y = baseline_rate - 4
                    baseline_va = 'top'
                else:
                    final_y = final_rate - 4
                    final_va = 'top'
            
            # Special fix for overlapping cases
            if (setting == "SWE-Sonnet-4.5+SWE-Sonnet-4.5-ACToR" and 
                (program_name == 'csplit')):
                baseline_y = baseline_rate - 10
                baseline_va = 'top'

            if (setting == "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR" and 
                ((program_name == 'join' and abs(final_rate - 78.6) < 0.1) or
                 (program_name == 'csplit' and abs(baseline_rate - 78.6) < 0.1))):
                if program_name == 'join':
                    # Move join's final label (78.6%) slightly up
                    final_y = final_rate + 3
                    final_va = 'bottom'
                elif program_name == 'csplit':
                    # Move csplit's baseline label (78.6%) slightly down  
                    baseline_y = baseline_rate - 3
                    baseline_va = 'top'
            
            # Check overlap with all previously placed text
            for prev_x, prev_y in all_text_positions:
                if texts_overlap(baseline_x, baseline_y, prev_x, prev_y):
                    baseline_y = baseline_rate - 4
                    baseline_va = 'top'
                if texts_overlap(final_x, final_y, prev_x, prev_y):
                    final_y = final_rate - 4
                    final_va = 'top'
            
            # Double-check if baseline and final still overlap after adjustments
            if texts_overlap(baseline_x, baseline_y, final_x, final_y):
                # Spread them further apart
                if baseline_va == 'top' and final_va == 'bottom':
                    baseline_y = baseline_rate - 6
                elif baseline_va == 'bottom' and final_va == 'top':
                    final_y = final_rate - 6
                elif baseline_va == 'bottom' and final_va == 'bottom':
                    # Both above bars, put one below
                    if baseline_rate <= final_rate:
                        baseline_y = baseline_rate - 6
                        baseline_va = 'top'
                    else:
                        final_y = final_rate - 6
                        final_va = 'top'
            
            # Add the text labels
            ax.text(baseline_x, baseline_y, f'{baseline_rate:.1f}%', 
                   ha='center', va=baseline_va, fontsize=20, fontweight='bold')
            ax.text(final_x, final_y, f'{final_rate:.1f}%', 
                   ha='center', va=final_va, fontsize=20, fontweight='bold')
            
            # Record positions for future overlap checks
            all_text_positions.extend([(baseline_x, baseline_y), (final_x, final_y)])
        
        # Enhanced styling matching plot_v2.py
        ax.set_facecolor('#FAFAFA')
        ax.grid(True, alpha=0.3, axis='y', linestyle='--')
        ax.set_axisbelow(True)
        
        # Labels and title
        # Only show x-label on bottom row
        if i >= ncols:
            ax.set_xlabel('Programs', fontsize=28, fontweight='bold')
        else:
            ax.set_xlabel('')
        
        # Only show y-label on leftmost subplots
        if i % ncols == 0:
            ax.set_ylabel('Pass Rate (%)', fontsize=28, fontweight='bold')
        else:
            ax.set_ylabel('')
        
        ax.set_title(model_name, fontsize=28, fontweight='bold', pad=20)
        
        # Set x-axis labels
        ax.set_xticks(x_pos)
        ax.set_xticklabels(programs_list, fontsize=26, rotation=45, ha='right')
        
        # Set y-axis range from 50% to 110% for better visibility (extra space for text labels)
        ax.set_ylim(50, 106)
        
        # Set y-axis ticks
        ax.tick_params(axis='y', labelsize=24)
    
    # Hide any unused subplots and use the last one for legend
    for j in range(num_settings, nrows * ncols):
        axes[j].axis('off')
    
    # Get legend handles from one of the used subplots (use the last one with data)
    last_data_idx = min(num_settings - 1, nrows * ncols - 1)
    handles, labels = axes[last_data_idx].get_legend_handles_labels()
    
    # Place legend in the last subplot position (if unused)
    if num_settings < nrows * ncols:
        legend_ax = axes[nrows * ncols - 1]
        legend_ax.legend(handles, ["Naive Baseline", "ACToR (Iter 10)"], 
                       loc='center', fontsize=30, framealpha=0.9, 
                       ncol=1, frameon=True, fancybox=True, shadow=True)
    
    # Adjust layout and save (with extra space for legend)
    plt.tight_layout()
    output_path = os.path.join(output_dir, 'eval1.png')
    plt.savefig(output_path, dpi=300, bbox_inches='tight')
    plt.close()
    
    print(f"[INFO] Saved combined figure with bar chart subplots to: {output_path}")


if __name__ == "__main__":
    main()
