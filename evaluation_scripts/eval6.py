"""
Experiment 6: Section 4.3, Macro Experiment (Macro Benchmark, Relative)
Compares: Default ACToR vs Coverage Baseline on 57 BSD Core Utilities
"""
from scripts.utils import _copy_directory, white_list_for_copy_rs, white_list_for_copy_c, white_list_for_copy_test_cases
from collect_cost_CC import collect_cost_cc
from collect_time import collect_time_cost

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

### Settings for macro experiment
settings = [
    "CC-Sonnet-4.5+CC-Sonnet-4.5-ACToR",
    "CC-Sonnet-4.5+CC-Sonnet-4.5-Coverage",
]

result_instance = {} 

arena_sandbox_dir = "./__arena_sandbox/"
working_dir = ".working_BSD/"
backup_dir = ".backups_BSD/"
input_dir = "projects_input_BSD/"

import os
import glob
import json

### Cache for relative comparison on macro benchmark
CACHE_FILE = "./__cache/cross_comparison_cache_macro.json"

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

def get_cached_result(cache, program, defender_setting, attacker_setting, iteration):
    """Get cached result if it exists."""
    try:
        return tuple(cache[program][str(iteration)][defender_setting][attacker_setting])
    except KeyError:
        return None

def set_cached_result(cache, program, iteration, defender_setting, attacker_setting, defense_num, attack_num):
    """Set cached result."""
    if program not in cache:
        cache[program] = {}
    if str(iteration) not in cache[program]:
        cache[program][str(iteration)] = {}
    if defender_setting not in cache[program][str(iteration)]:
        cache[program][str(iteration)][defender_setting] = {}
    
    cache[program][str(iteration)][defender_setting][attacker_setting] = [defense_num, attack_num]


def cross_comparison(result_instance):
    """
    Cross comparison between ACToR and Coverage baseline on macro benchmark.
    """
    import shutil
    import subprocess
    
    print("Starting cross comparison on macro benchmark...")
    
    cache = load_cache()
    
    results_per_program = {}
    
    iter_num = iteration_num
    print(f"\n[INFO] ==== Processing iteration {iter_num} ====")
    
    total_defense_per_setting = {s: 0 for s in settings}
    total_attack_per_setting = {s: 0 for s in settings}
    
    for program in programs:
        print(f"[INFO] Processing program: {program}")
        
        defense_per_setting = {s: 0 for s in settings}
        attack_per_setting = {s: 0 for s in settings}
        
        for defense_setting in settings:
            for attack_setting in settings:
                if defense_setting == attack_setting:
                    continue  # Skip self comparison
                
                defense_instance = result_instance.get(program, {}).get(defense_setting, None)
                attack_instance = result_instance.get(program, {}).get(attack_setting, None)
                
                if not defense_instance or not attack_instance:
                    print(f"[WARNING] Missing instance for {program}")
                    continue
                
                print(f"[INFO] Processing pair: defense {defense_setting} vs attack {attack_setting}")

                cached_result = get_cached_result(cache, program, defense_setting, attack_setting, iter_num)
                if cached_result is not None:
                    defense_success, attack_success = cached_result
                    defense_per_setting[defense_setting] += defense_success
                    attack_per_setting[attack_setting] += attack_success
                    continue
                
                try:
                    defense_folder = working_dir + defense_instance
                    attack_folder = working_dir + attack_instance
                    
                    defense_rs_files = os.path.join(defense_folder, "rs_files")
                    defense_c_files = os.path.join(defense_folder, "c_files")
                    attack_test_cases_dir = os.path.join(attack_folder, "test_cases")
                    
                    assert os.path.exists(defense_rs_files), f"Defense Rust files not found: {defense_rs_files}"
                    assert os.path.exists(defense_c_files), f"Defense C files not found: {defense_c_files}"
                    assert os.path.exists(attack_test_cases_dir), f"Attack test cases dir not found: {attack_test_cases_dir}"
                    
                    if os.path.exists(arena_sandbox_dir):
                        shutil.rmtree(arena_sandbox_dir)
                    os.makedirs(arena_sandbox_dir)

                    _copy_directory(defense_rs_files, arena_sandbox_dir, whitelist=white_list_for_copy_rs)
                    _copy_directory(defense_c_files, arena_sandbox_dir, whitelist=white_list_for_copy_c)
                    _copy_directory(attack_test_cases_dir, arena_sandbox_dir, whitelist=white_list_for_copy_test_cases)

                    compile_result = subprocess.run(
                        f"make clean && make all", 
                        cwd=arena_sandbox_dir,
                        capture_output=True, 
                        shell=True
                    )
                    if compile_result.returncode != 0:
                        print(f"[WARNING] C compilation failed for {defense_setting} {program}")
                        continue

                    compile_result = subprocess.run(
                        f"cargo clean && cargo build --release", 
                        cwd=f"{arena_sandbox_dir}/ts", 
                        capture_output=True, 
                        shell=True
                    )
                    
                    if compile_result.returncode != 0:
                        print(f"[WARNING] Rust compilation failed for {defense_setting} {program}")
                        continue
                    
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
                    
                    defense_per_setting[defense_setting] += defense_success
                    attack_per_setting[attack_setting] += attack_success
                    
                    set_cached_result(cache, program, iter_num, defense_setting, attack_setting, defense_success, attack_success)
                    save_cache(cache)
                    
                except Exception as e:
                    print(f"[WARNING] Error: {e}")
                    set_cached_result(cache, program, iter_num, defense_setting, attack_setting, 0, 0)
                    save_cache(cache)
                    continue
    
        for setting in settings:
            total_defense_per_setting[setting] += defense_per_setting[setting]
            total_attack_per_setting[setting] += attack_per_setting[setting]
        
        results_per_program[program] = {s: (defense_per_setting[s], attack_per_setting[s]) for s in settings}
    
    print("="*20)
    print("Results Table: Macro Benchmark Cross Comparison")
    print("="*20)
    
    actor_setting = settings[0]
    cov_setting = settings[1]
    
    actor_defense = total_defense_per_setting[actor_setting]
    actor_attack = total_attack_per_setting[actor_setting]
    cov_defense = total_defense_per_setting[cov_setting]
    cov_attack = total_attack_per_setting[cov_setting]
    
    print(f"ACToR defense: {actor_defense}, ACToR attack: {actor_attack}")
    print(f"Coverage defense: {cov_defense}, Coverage attack: {cov_attack}")
    
    total_actor = actor_defense + cov_attack
    total_cov = cov_defense + actor_attack
    
    if total_actor > 0:
        actor_rate = actor_defense / total_actor
        print(f"ACToR relative pass rate: {actor_rate:.4f} ({actor_defense}/{total_actor})")
    
    if total_cov > 0:
        cov_rate = cov_defense / total_cov
        print(f"Coverage relative pass rate: {cov_rate:.4f} ({cov_defense}/{total_cov})")
    
    print("="*20)
    
    return results_per_program


def collect_result_instance():
    """Collect result instances from working directory."""
    result_instance = {}
    for program in programs:
        result_instance[program] = {}
        for setting in settings:
            setting_parts = setting.split("+")
            if len(setting_parts) != 2:
                continue
            expected_translator, expected_discriminator = setting_parts
            
            if not os.path.exists(working_dir):
                continue
            for subfolder in os.listdir(working_dir):
                if subfolder.startswith(program + "_"):
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
                                    result_instance[program][setting] = subfolder
                                    break
                        except Exception as e:
                            continue
    return result_instance


if __name__ == "__main__":
    result_instance = collect_result_instance()
    print(json.dumps(result_instance, indent=4))
    
    # Check for missing programs
    for program in programs:
        for setting in settings:
            if setting not in result_instance.get(program, {}):
                print(f"[WARNING] Missing {program} for {setting}")
    
    cross_comparison(result_instance)

    ### Collect the money/token cost for each setting
    print("[INFO] Collecting the money/token cost for each setting")
    
    # ACToR
    print(f"[INFO] ----- {settings[0]} -----")
    instances_actor = [result_instance[program][settings[0]] for program in programs if settings[0] in result_instance.get(program, {})]
    if instances_actor:
        collect_cost_cc(instances_actor, "claude-sonnet-4-5-20250929")
        collect_time_cost(instances_actor, working_dir=".working_BSD")

    # Coverage
    print(f"[INFO] ----- {settings[1]} -----")
    instances_cov = [result_instance[program][settings[1]] for program in programs if settings[1] in result_instance.get(program, {})]
    if instances_cov:
        collect_cost_cc(instances_cov, "claude-sonnet-4-5-20250929")
        collect_time_cost(instances_cov, working_dir=".working_BSD")

