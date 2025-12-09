"""
Experiment 3: Section 4.2, Ablation Study on Design - Equal Cost Comparison (Micro Benchmark, Relative)
Compares: Default ACToR (10 iter) vs Coverage Baseline (25 iter)
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

### Settings with their corresponding iteration numbers
### Format: ("setting_name", iteration_num)
settings_with_iter = [
    ("CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR", 10),
    ("CC-Sonnet-4.5+CC-Sonnet-4.5-Coverage", 25),
]

settings = [s[0] for s in settings_with_iter]

result_instance = {} 

arena_sandbox_dir = "./__arena_sandbox/"
working_dir = ".working/"
backup_dir = ".backups/"
input_dir = "projects_input/"

import os
import glob
import json

### Cache for relative comparison
CACHE_FILE = "./__cache/cross_comparison_cache_eval3.json"

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

def get_cached_result(cache, program, defender_instance, attacker_instance, defender_iter, attacker_iter):
    """Get cached result if it exists."""
    key = f"{defender_iter}_{attacker_iter}"
    try:
        return tuple(cache[program][key][defender_instance][attacker_instance])
    except KeyError:
        return None

def set_cached_result(cache, program, defender_iter, attacker_iter, defender_instance, attacker_instance,  
        defense_success, defense_total, attack_success, attack_total):
    """Set cached result."""
    key = f"{defender_iter}_{attacker_iter}"
    if program not in cache:
        cache[program] = {}
    if key not in cache[program]:
        cache[program][key] = {}
    if defender_instance not in cache[program][key]:
        cache[program][key][defender_instance] = {}
    
    cache[program][key][defender_instance][attacker_instance] = [defense_success, defense_total, attack_success, attack_total]


def cross_comparison(result_instance):
    """
    Cross comparison between different settings with different iterations.
    """
    import shutil
    import subprocess
    
    print("Starting cross comparison...")
    
    cache = load_cache()
    
    results = {}
    results_per_program = {}
    
    total_defense_success_per_setting = {s: 0 for s in settings}
    total_defense_total_per_setting = {s: 0 for s in settings}
    total_attack_success_per_setting = {s: 0 for s in settings}
    total_attack_total_per_setting = {s: 0 for s in settings}
    
    for program in programs:
        print(f"[INFO] Processing program: {program}")
        
        defense_success_per_setting = {s: 0 for s in settings}
        defense_total_per_setting = {s: 0 for s in settings}
        attack_success_per_setting = {s: 0 for s in settings}
        attack_total_per_setting = {s: 0 for s in settings}
        
        for defense_setting, defense_iter in settings_with_iter:
            for attack_setting, attack_iter in settings_with_iter:
                defense_instance = result_instance[program][defense_setting]
                attack_instance = result_instance[program][attack_setting]
                
                print(f"[INFO] Processing pair: defense {defense_setting}@{defense_iter} vs attack {attack_setting}@{attack_iter}")

                cached_result = get_cached_result(cache, program, defense_instance, attack_instance, defense_iter, attack_iter)
                if cached_result is not None:
                    defense_success, defense_total, attack_success, attack_total = cached_result
                    defense_success_per_setting[defense_setting] += defense_success
                    defense_total_per_setting[defense_setting] += defense_total
                    attack_success_per_setting[attack_setting] += attack_success
                    attack_total_per_setting[attack_setting] += attack_total
                    continue
                
                try:
                    defense_folder = backup_dir + defense_instance + f"/iteration_{defense_iter}"
                    attack_folder = backup_dir + attack_instance + f"/iteration_{attack_iter}"
                    
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
                        print(f"[WARNING] C compilation failed")
                        exit(1)

                    compile_result = subprocess.run(
                        f"cargo clean && cargo build --release", 
                        cwd=f"{arena_sandbox_dir}/ts", 
                        capture_output=True, 
                        shell=True
                    )
                    
                    if compile_result.returncode != 0:
                        print(f"[WARNING] Rust compilation failed")
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
                    
                    failed_tests = total_tests - passed_tests
                    
                    if total_tests == 0:
                        defense_success = 0
                        attack_success = 0
                    else:
                        defense_success = passed_tests
                        attack_success = failed_tests
                    
                    defense_success_per_setting[defense_setting] += defense_success
                    defense_total_per_setting[defense_setting] += total_tests
                    attack_success_per_setting[attack_setting] += attack_success
                    attack_total_per_setting[attack_setting] += total_tests
                    
                    set_cached_result(cache, program, defense_iter, attack_iter, defense_instance, attack_instance, defense_success, total_tests, attack_success, total_tests)
                    save_cache(cache)
                    
                except Exception as e:
                    print(f"[WARNING] Error: {e}")
                    continue
    
        for setting in settings:
            total_defense_success_per_setting[setting] += defense_success_per_setting[setting]
            total_defense_total_per_setting[setting] += defense_total_per_setting[setting]
            total_attack_success_per_setting[setting] += attack_success_per_setting[setting]
            total_attack_total_per_setting[setting] += attack_total_per_setting[setting]
        
        results_per_program[program] = {s: (defense_success_per_setting[s], defense_total_per_setting[s], attack_success_per_setting[s], attack_total_per_setting[s]) for s in settings}
    
    results = {s: (total_defense_success_per_setting[s], total_defense_total_per_setting[s], total_attack_success_per_setting[s], total_attack_total_per_setting[s]) for s in settings}
    
    print("="*20)
    print("Results Table: Equal Cost Comparison")
    print("="*20)
    
    for setting, iter_num in settings_with_iter:
        defense_success, defense_total, attack_success, attack_total = results[setting]
        print(f"{setting} @ {iter_num} iter: {defense_success}D/{defense_total}T ({defense_success/defense_total*100:.2f}%)")

    print("="*20)


def collect_result_instance():
    result_instance = {}
    for program in programs:
        result_instance[program] = {}
        for setting, iter_num in settings_with_iter:
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
                                if translator == expected_translator and discriminator == expected_discriminator and cur_iter >= iter_num + 1:
                                    result_instance[program][setting].append(subfolder)
                        except Exception as e:
                            continue
    return result_instance


if __name__ == "__main__":
    result_instance = collect_result_instance()
    print(json.dumps(result_instance, indent=4))

    for program in programs:
        for setting in settings:
            if result_instance[program][setting]:
                result_instance[program][setting] = result_instance[program][setting][0]
            else:
                print(f"[WARNING] No instance found for {program} {setting}")

    cross_comparison(result_instance)

    ### Collect the money/token cost for each setting
    print("[INFO] Collecting the money/token cost for each setting")
    
    # ACToR at 10 iterations
    setting_actor, iter_actor = settings_with_iter[0]
    print(f"[INFO] ----- {setting_actor} @ {iter_actor} iter -----")
    collect_cost_cc([result_instance[program][setting_actor] for program in programs], "claude-sonnet-4-5-20250929", iteration_num=iter_actor)
    collect_time_cost([result_instance[program][setting_actor] for program in programs])

    # Coverage at 25 iterations
    setting_cov, iter_cov = settings_with_iter[1]
    print(f"[INFO] ----- {setting_cov} @ {iter_cov} iter -----")
    collect_cost_cc([result_instance[program][setting_cov] for program in programs], "claude-sonnet-4-5-20250929", iteration_num=iter_cov)
    collect_time_cost([result_instance[program][setting_cov] for program in programs])

