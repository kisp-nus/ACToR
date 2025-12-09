"""
Experiment 5: Section 4.2, Stability of Results (Micro Benchmark, Absolute)
Runs default ACToR 3 times and measures variance across retries
"""
from scripts.utils import _copy_directory, white_list_for_copy_rs, white_list_for_copy_c, white_list_for_copy_test_cases

programs = [
    "printf",
    "expr",
    "fmt",
    "test",
    "join",
    "csplit",
]

iteration_num = 10

### Setting to test for stability
setting = "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR"

### Number of retries to test
num_retries = 3

result_instance = {} 

arena_sandbox_dir = "./__arena_sandbox/"
working_dir = ".working/"
backup_dir = ".backups/"
input_dir = "validation_tests/"

import os
import glob
import json
import statistics
import numpy as np

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


def stability_comparison(result_instance):
    """
    Compare stability across multiple retries.
    """
    print("Starting stability comparison...")
    
    cache = load_cache()
    
    pass_number_program_instance = {}
    total_test_program = {}

    for program in programs:
        print(f"[INFO] Processing program: {program}")
        
        pass_number_program_instance[program] = {}
        instances = result_instance.get(program, {}).get(setting, [])
        
        if len(instances) < num_retries:
            print(f"[WARNING] Only {len(instances)} retries found for {program}, expected {num_retries}")
        
        for instance in instances[:num_retries]:
            print(f"[INFO] Processing instance: {instance}")
            
            passed, total = run_test_for_instance(program, instance, iteration_num, cache)
            pass_number_program_instance[program][instance] = passed
            total_test_program[program] = total

    ### Print results table
    print("="*20)
    print("Results Table: Defense Success Across Retries")
    print("="*20)

    header = "Program".ljust(25)
    for i in range(num_retries):
        header += f"Retry_{i}".ljust(25)
    print(header)
    print("-" * len(header))

    total_pass = 0
    total_test = 0
    for program in programs:
        row = program.ljust(25)
        instances = result_instance.get(program, {}).get(setting, [])
        for instance in instances[:num_retries]:
            passed = pass_number_program_instance[program].get(instance, 0)
            total = total_test_program.get(program, 0)
            row += f"{passed}/{total}".ljust(25)
            total_pass += passed
            total_test += total
        print(row)
    
    print(f"Total Pass: {total_pass}/{total_test}")

    ### Compute variance
    variances = []
    program_pass_rates = []

    for program in programs:
        instances = result_instance.get(program, {}).get(setting, [])
        passes = [pass_number_program_instance[program].get(inst, 0) for inst in instances[:num_retries]]
        total = total_test_program.get(program, 1)
        
        if total > 0:
            rates = [p / total for p in passes]
            rate = sum(passes) / (len(passes) * total)
            program_pass_rates.append(rate)
            
            if len(rates) >= 2:
                var = statistics.stdev(rates)
                variances.append(var)

    if variances:
        avg_variance = np.mean(variances)
    else:
        avg_variance = float('nan')

    if total_test > 0:
        avg_pass_rate = total_pass / total_test
    else:
        avg_pass_rate = 0.0

    print("="*40)
    print(f"Average stdev across {num_retries} retries (across programs): {avg_variance:.6f}")
    print(f"Average pass rate (total passed tests/total tests): {avg_pass_rate:.3%}")
    print("="*40)


def collect_result_instance():
    """Collect result instances from working directory."""
    result_instance = {}
    for program in programs:
        result_instance[program] = {}
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
                            if translator == expected_translator and discriminator == expected_discriminator and cur_iter >= iteration_num + 1:
                                result_instance[program][setting].append(subfolder)
                    except Exception as e:
                        continue
    return result_instance


if __name__ == "__main__":
    result_instance = collect_result_instance()
    print(json.dumps(result_instance, indent=4))

    stability_comparison(result_instance)

