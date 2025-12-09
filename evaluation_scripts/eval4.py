"""
Experiment 4: Section 4.2, Ablation Study on Configurations (Micro Benchmark, Absolute)
Compares different test configurations: 15+3 (default), 15+1, 15+5, 1+3
Also shows iteration progression: 0, 5, 10, 15, 20
"""
from scripts.utils import _copy_directory, white_list_for_copy_rs, white_list_for_copy_c, white_list_for_copy_test_cases
from collect_cost_CC import collect_cost_cc
from collect_time import collect_time_cost

programs = [
    "printf",
    "expr",
    "fmt",
    "test",
    "join",
    "csplit",
]

iteration_num = 10

### Settings for different test configurations
settings_configs = [
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR",          # default 15+3
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR-15_1",     # 15+1
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR-15_5",     # 15+5
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR-1_3",      # 1+3
]

### Settings for iteration progression (same setting, different iterations)
iterations_to_test = [0, 5, 10, 15, 20]
setting_for_iteration = "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR"

result_instance = {} 

arena_sandbox_dir = "./__arena_sandbox/"
working_dir = ".working/"
backup_dir = ".backups/"
input_dir = "validation_tests/"

import os
import glob
import json

CACHE_FILE = "./__cache/absolute_comparison_cache.json"

def load_cache():
    """Load cached results from JSON file."""
    if os.path.exists(CACHE_FILE):
        try:
            with open(CACHE_FILE, 'r') as f:
                cache = json.load(f)
            print(f"[INFO] Loaded cache from {CACHE_FILE}")
            return cache
        except (json.JSONDecodeError, IOError) as e:
            print(f"[WARNING] Failed to load cache: {e}")
            return {}
    else:
        print(f"[INFO] No cache file found, starting with empty cache")
        os.makedirs(os.path.dirname(CACHE_FILE), exist_ok=True)
        with open(CACHE_FILE, 'w') as f:
            json.dump({}, f, indent=2)
        return {}

def save_cache(cache):
    """Save cache to JSON file."""
    try:
        with open(CACHE_FILE, 'w') as f:
            json.dump(cache, f, indent=2)
    except IOError as e:
        print(f"[WARNING] Failed to save cache: {e}")

def get_cached_result(cache, program, defender_instance, iteration):
    """Get cached result if it exists."""
    try:
        return tuple(cache[program][str(iteration)][defender_instance])
    except KeyError:
        return None

def set_cached_result(cache, program, defender_instance, iteration, pass_num, total_num):
    """Set cached result."""
    if program not in cache:
        cache[program] = {}
    if str(iteration) not in cache[program]:
        cache[program][str(iteration)] = {}
    
    cache[program][str(iteration)][defender_instance] = [pass_num, total_num]


def run_test_for_instance(program, instance, iter_num, cache):
    """Run tests for a specific instance and iteration."""
    import shutil
    import subprocess
    
    cached_result = get_cached_result(cache, program, instance, iter_num)
    if cached_result is not None:
        return cached_result
    
    try:
        defense_folder = backup_dir + instance + f"/iteration_{iter_num}"
        defense_rs_files = os.path.join(defense_folder, "rs_files")
        source_dir = os.path.join(input_dir, program)
        
        assert os.path.exists(defense_rs_files), f"Defense Rust files not found: {defense_rs_files}"
        assert os.path.exists(source_dir), f"Source dir not found: {source_dir}"
        
        if os.path.exists(arena_sandbox_dir):
            shutil.rmtree(arena_sandbox_dir)
        os.makedirs(arena_sandbox_dir)

        _copy_directory(defense_rs_files, arena_sandbox_dir, whitelist=white_list_for_copy_rs)
        _copy_directory(source_dir, arena_sandbox_dir, whitelist=white_list_for_copy_c)
        _copy_directory(source_dir, arena_sandbox_dir, whitelist=white_list_for_copy_test_cases)

        compile_result = subprocess.run(
            f"make clean && make all", 
            cwd=arena_sandbox_dir,
            capture_output=True, 
            shell=True
        )
        if compile_result.returncode != 0:
            print(f"[WARNING] C compilation failed")
            return (0, 0)

        compile_result = subprocess.run(
            f"cargo clean && cargo build --release", 
            cwd=f"{arena_sandbox_dir}/ts", 
            capture_output=True, 
            shell=True
        )
        
        if compile_result.returncode != 0:
            print(f"[WARNING] Rust compilation failed")
            return (0, 0)
        
        test_result = subprocess.run(
            ["./testcmp.sh", "compare", f"./ts/target/release/{program}"], 
            cwd=arena_sandbox_dir, 
            capture_output=True, 
            text=True
        )
        
        output_text = test_result.stdout
        
        total_tests = 0
        for line in output_text.split('\n'):
            if "Loaded" in line and "tests total" in line:
                parts = line.split()
                for i, part in enumerate(parts):
                    if part == "Loaded" and i + 1 < len(parts):
                        try:
                            total_tests = int(parts[i + 1])
                            break
                        except ValueError:
                            continue
                break
        
        passed_tests = 0
        for line in output_text.split('\n'):
            if "Results:" in line and "passed" in line and "failed" in line and "out of" in line:
                parts = line.split()
                for i, part in enumerate(parts):
                    if part == "passed," and i > 0:
                        try:
                            passed_tests = int(parts[i - 1])
                        except ValueError:
                            continue
                break
        
        set_cached_result(cache, program, instance, iter_num, passed_tests, total_tests)
        save_cache(cache)
        
        return (passed_tests, total_tests)
        
    except Exception as e:
        print(f"[WARNING] Error: {e}")
        return (0, 0)


def compare_configurations(result_instance):
    """Compare different test configurations at iteration 10."""
    print("\n" + "="*60)
    print("Configuration Comparison (at iter 10)")
    print("="*60)
    
    cache = load_cache()
    
    results = {}
    for setting in settings_configs:
        results[setting] = {'passed': 0, 'total': 0}
        
        for program in programs:
            instance = result_instance.get(program, {}).get(setting, None)
            if not instance:
                print(f"[WARNING] No instance for {program} {setting}")
                continue
            
            passed, total = run_test_for_instance(program, instance, iteration_num, cache)
            results[setting]['passed'] += passed
            results[setting]['total'] += total
        
        rate = results[setting]['passed'] / results[setting]['total'] * 100 if results[setting]['total'] > 0 else 0
        print(f"{setting}: {results[setting]['passed']}/{results[setting]['total']} ({rate:.2f}%)")
    
    return results


def compare_iterations(result_instance):
    """Compare different iterations for the same setting."""
    print("\n" + "="*60)
    print("Iteration Progression")
    print("="*60)
    
    cache = load_cache()
    
    results = {}
    for iter_num in iterations_to_test:
        results[iter_num] = {'passed': 0, 'total': 0}
        
        for program in programs:
            instance = result_instance.get(program, {}).get(setting_for_iteration, None)
            if not instance:
                print(f"[WARNING] No instance for {program} {setting_for_iteration}")
                continue
            
            passed, total = run_test_for_instance(program, instance, iter_num, cache)
            results[iter_num]['passed'] += passed
            results[iter_num]['total'] += total
        
        rate = results[iter_num]['passed'] / results[iter_num]['total'] * 100 if results[iter_num]['total'] > 0 else 0
        print(f"Iter {iter_num}: {results[iter_num]['passed']}/{results[iter_num]['total']} ({rate:.2f}%)")
    
    return results


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
                                # For iteration test, need at least 20 iterations
                                min_iter = max(iterations_to_test) + 1 if setting == setting_for_iteration else iteration_num + 1
                                if translator == expected_translator and discriminator == expected_discriminator and cur_iter >= min_iter:
                                    result_instance[program][setting].append(subfolder)
                        except Exception as e:
                            continue
    return result_instance


if __name__ == "__main__":
    result_instance = collect_result_instance()
    print(json.dumps(result_instance, indent=4))

    # Flatten instance lists
    for program in programs:
        for setting in list(result_instance[program].keys()):
            if result_instance[program][setting]:
                result_instance[program][setting] = result_instance[program][setting][0]
            else:
                print(f"[WARNING] No instance found for {program} {setting}")

    # Run comparisons
    config_results = compare_configurations(result_instance)
    iter_results = compare_iterations(result_instance)

