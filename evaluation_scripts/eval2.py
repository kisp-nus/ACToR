"""
Experiment 2: Section 4.2, Ablation Study on Design (Micro Benchmark, Relative)
Compares: Default ACToR, ACToR noFuzz, Coverage Baseline
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

### settings = ["Translator+Discriminator", ...]
settings = [
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR",
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR-noFuzz",
    "CC-Sonnet-4.5+CC-Sonnet-4.5-Coverage",
]

result_instance = {} 

arena_sandbox_dir = "./__arena_sandbox/"
working_dir = ".working/"
backup_dir = ".backups/"
input_dir = "projects_input/"

import os
import glob
import json

### Cache for relative comparison
CACHE_FILE = "./__cache/cross_comparison_cache_eval2.json"

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

def get_cached_result(cache, program, defender_instance, attacker_instance, iteration):
    """Get cached result if it exists."""
    try:
        return tuple(cache[program][str(iteration)][defender_instance][attacker_instance])
    except KeyError:
        return None

def set_cached_result(cache, program, iteration, defender_instance, attacker_instance,  
        defense_success, defense_total, attack_success, attack_total):
    """Set cached result."""
    if program not in cache:
        cache[program] = {}
    if str(iteration) not in cache[program]:
        cache[program][str(iteration)] = {}
    if defender_instance not in cache[program][str(iteration)]:
        cache[program][str(iteration)][defender_instance] = {}
    
    cache[program][str(iteration)][defender_instance][attacker_instance] = [defense_success, defense_total, attack_success, attack_total]

def clear_cache():
    """Clear the cache file."""
    if os.path.exists(CACHE_FILE):
        os.remove(CACHE_FILE)
        print(f"[INFO] Cache file {CACHE_FILE} cleared")


def cross_comparison(result_instance):
    """
    Cross comparison between different settings.
    For each iteration and program, run test cases from each attacker setting 
    against the Rust code from each defender setting.
    """
    import shutil
    import subprocess
    
    print("Starting cross comparison...")
    
    cache = load_cache()
    
    results = {}
    results_per_program = {}
    
    for iter_num in [iteration_num]:
        print(f"\n[INFO] ==== Processing iteration {iter_num} ====")
        results[iter_num] = {}
        
        total_defense_success_per_setting = {i: 0 for i in settings}
        total_defense_total_per_setting = {i: 0 for i in settings}
        total_attack_success_per_setting = {i: 0 for i in settings}
        total_attack_total_per_setting = {i: 0 for i in settings}
        
        for program in programs:
            print(f"[INFO] Processing program: {program}")
            
            defense_success_per_setting = {i: 0 for i in settings}
            defense_total_per_setting = {i: 0 for i in settings}
            attack_success_per_setting = {i: 0 for i in settings}
            attack_total_per_setting = {i: 0 for i in settings}
            
            for defense_setting in settings:
                for attack_setting in settings:
                    defense_instance = result_instance[program][defense_setting]
                    attack_instance = result_instance[program][attack_setting]
                    
                    print(f"[INFO] Processing pair: defense {defense_setting} vs attack {attack_setting}")

                    cached_result = get_cached_result(cache, program, defense_instance, attack_instance, iter_num)
                    if cached_result is not None:
                        defense_success, defense_total, attack_success, attack_total = cached_result
                        defense_success_per_setting[defense_setting] += defense_success
                        defense_total_per_setting[defense_setting] += defense_total
                        attack_success_per_setting[attack_setting] += attack_success
                        attack_total_per_setting[attack_setting] += attack_total
                        continue
                    
                    try:
                        defense_folder = backup_dir + result_instance[program][defense_setting] + f"/iteration_{iter_num}"
                        attack_folder = backup_dir + result_instance[program][attack_setting] + f"/iteration_{iter_num}"
                        
                        _c_files = os.path.join(input_dir, program)
                        defense_rs_files = os.path.join(defense_folder, "rs_files")
                        attack_test_cases_dir = os.path.join(attack_folder, "test_cases")
                        
                        assert os.path.exists(defense_rs_files), f"Defense Rust files not found: {defense_rs_files}"
                        assert os.path.exists(_c_files), f"Defense C files not found: {_c_files}"
                        assert os.path.exists(attack_test_cases_dir), f"Attack test cases dir not found: {attack_test_cases_dir}"
                        
                        if os.path.exists(arena_sandbox_dir):
                            shutil.rmtree(arena_sandbox_dir)
                        os.makedirs(arena_sandbox_dir)

                        _copy_directory(defense_rs_files, arena_sandbox_dir, whitelist=white_list_for_copy_rs)
                        _copy_directory(_c_files, arena_sandbox_dir, whitelist=white_list_for_copy_c)
                        _copy_directory(attack_test_cases_dir, arena_sandbox_dir, whitelist=white_list_for_copy_test_cases)

                        compile_result = subprocess.run(
                            f"make clean && make all", 
                            cwd=arena_sandbox_dir,
                            capture_output=True, 
                            shell=True
                        )
                        if compile_result.returncode != 0:
                            print(f"[WARNING] C compilation failed for {defense_setting} {program} iter_{iter_num}")
                            print(f"Compile error: {compile_result.stderr}")
                            exit(1)

                        compile_result = subprocess.run(
                            f"cargo clean && cargo build --release", 
                            cwd=f"{arena_sandbox_dir}/ts", 
                            capture_output=True, 
                            shell=True
                        )
                        
                        if compile_result.returncode != 0:
                            print(f"[WARNING] Rust compilation failed for {defense_setting} {program} iter_{iter_num}")
                            print(f"Compile error: {compile_result.stderr}")
                            exit(1)
                        
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
                        total_tests2 = None
                        for line in output_text.split('\n'):
                            if "Results:" in line and "passed" in line and "failed" in line and "out of" in line:
                                parts = line.split()
                                for i, part in enumerate(parts):
                                    if part == "passed," and i > 0:
                                        try:
                                            passed_tests = int(parts[i - 1])
                                        except ValueError:
                                            continue
                                    if part == "of" and i + 1 < len(parts):
                                        try:
                                            total_tests2 = int(parts[i + 1])
                                            break
                                        except ValueError:
                                            continue
                                break
                        
                        if total_tests2 is not None:
                            assert total_tests == total_tests2, f"Total tests mismatch: {total_tests} vs {total_tests2}"
                        
                        failed_tests = total_tests - passed_tests
                        
                        if total_tests == 0:
                            print(f"[WARNING] No tests found for {attack_setting} vs {defense_setting} for {program} iter_{iter_num}")
                            defense_success = 0
                            attack_success = 0
                        elif passed_tests < 0 or failed_tests < 0:
                            print(f"[ERROR] Invalid test counts: passed={passed_tests}, failed={failed_tests}, total={total_tests}")
                            defense_success = 0
                            attack_success = 0
                        else:
                            defense_success = passed_tests
                            attack_success = failed_tests
                        
                        defense_success_per_setting[defense_setting] += defense_success
                        defense_total_per_setting[defense_setting] += total_tests
                        attack_success_per_setting[attack_setting] += attack_success
                        attack_total_per_setting[attack_setting] += total_tests
                        
                        set_cached_result(cache, program, iter_num, defense_instance, attack_instance, defense_success, total_tests, attack_success, total_tests)
                        save_cache(cache)
                        
                    except Exception as e:
                        print(f"[WARNING] Error testing {attack_setting} vs {defense_setting} for {program} iter_{iter_num}: {e}")
                        continue
        
            for setting in settings:
                total_defense_success_per_setting[setting] += defense_success_per_setting[setting]
                total_defense_total_per_setting[setting] += defense_total_per_setting[setting]
                total_attack_success_per_setting[setting] += attack_success_per_setting[setting]
                total_attack_total_per_setting[setting] += attack_total_per_setting[setting]
            
            if iter_num == iteration_num:
                results_per_program[program] = {i: (defense_success_per_setting[i], defense_total_per_setting[i], attack_success_per_setting[i], attack_total_per_setting[i]) for i in settings}
        
        results[iter_num] = {i: (total_defense_success_per_setting[i], total_defense_total_per_setting[i], total_attack_success_per_setting[i], total_attack_total_per_setting[i]) for i in settings}
    
    print("="*20)
    print("Results Table: Defense/Attack Success by Setting and Iteration")
    print("="*20)
    
    header = "Setting".ljust(50)
    for iter_num in [iteration_num]:
        header += f"Iter_{iter_num}".ljust(25)
    print(header)
    print("-" * len(header))
    
    for setting in settings:
        row = setting.ljust(50)
        for iter_num in [iteration_num]:
            if iter_num in results:
                defense_success, defense_total, attack_success, attack_total = results[iter_num][setting]
                cell = f"{defense_success}D/{defense_total}T/{attack_success}A/{attack_total}T"
                row += cell.ljust(25)
            else:
                row += "N/A".ljust(25)
        print(row)

    print("\nLegend: XD/YT means X defense successes out of Y total tests")
    print("="*20)


def collect_result_instance():
    result_instance = {}
    for program in programs:
        result_instance[program] = {}
        for setting in settings:
            result_instance[program][setting] = []
            setting_parts = setting.split("+")
            if len(setting_parts) != 2:
                continue
            expected_translator, expected_discriminator = setting_parts
            
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


if __name__ == "__main__":
    result_instance = collect_result_instance()
    print(json.dumps(result_instance, indent=4))

    # Flatten instance lists to single instance
    for program in programs:
        for setting in settings:
            if result_instance[program][setting]:
                result_instance[program][setting] = result_instance[program][setting][0]
            else:
                print(f"[WARNING] No instance found for {program} {setting}")

    cross_comparison(result_instance)

    print("[INFO] Collecting the money/token cost for each setting")
    for i, setting in enumerate(settings):
        print(f"[INFO] ----- {setting} -----")
        collect_cost_cc([result_instance[program][setting] for program in programs], "claude-sonnet-4-5-20250929")
        collect_time_cost([result_instance[program][setting] for program in programs])

