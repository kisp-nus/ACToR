from scripts.utils import _copy_directory, white_list_for_copy_rs, white_list_for_copy_c, white_list_for_copy_test_cases
from collect_cost_CC import collect_cost_cc
from collect_cost_SWE import collect_cost_swe
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
    # "CC-Sonnet-4+CC-Sonnet-4-ACToR", ### not supported in the latest Claude Code version
    "SWE-Sonnet-4.5+SWE-Sonnet-4.5-ACToR",
    "SWE-Sonnet-4+SWE-Sonnet-4-ACToR",
    "SWE-GPT-5mini+SWE-GPT-5mini-ACToR",
]

### result_instance = {
#     "programName1": {
#         "Translator+Discriminator": "programName1_abc123",
#         "Translator+Discriminator": "programName1_def456",
#         "Translator+Discriminator": "programName1_ghi789",
#         "Translator+Discriminator": "programName1_jkl012",
#         "Translator+Discriminator": "programName1_mno345",
#         "Translator+Discriminator": "programName1_pqr678",
#     },
#     "programName2": {
#         "Translator+Discriminator": "programName2_abc123",
#         "Translator+Discriminator": "programName2_def456",
#         "Translator+Discriminator": "programName2_ghi789",
#         "Translator+Discriminator": "programName2_jkl012",
#         "Translator+Discriminator": "programName2_mno345",
#         "Translator+Discriminator": "programName2_pqr678",
#     },
#     ...
# }
result_instance = {} 

arena_sandbox_dir = "./__arena_sandbox/"
working_dir = ".working/"
backup_dir = ".backups/"
input_dir = "validation_tests/"

### sanity check
import os
import glob
import json


### Cache the Pass Rate for each setting
CACHE_FILE = "./__cache/absolute_comparison_cache.json"

def load_cache():
    """
    Load cached results from JSON file.
    Cache structure: {program: {iteration: {defender_setting: {attacker_setting: [defense_num, attack_num]}}}}
    """
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
        with open(CACHE_FILE, 'w') as f:
            json.dump({}, f, indent=2)
        return {}

def save_cache(cache):
    """
    Save cache to JSON file.
    """
    try:
        with open(CACHE_FILE, 'w') as f:
            json.dump(cache, f, indent=2)
        # print(f"[INFO] Cache saved to {CACHE_FILE}")
    except IOError as e:
        print(f"[WARNING] Failed to save cache: {e}")

def get_cached_result(cache, program, defender_instance, iteration):
    """
    Get cached result if it exists.
    Returns (defense_num, attack_num) if found, None otherwise.
    """
    # return None
    try:
        return tuple(cache[program][str(iteration)][defender_instance])
    except KeyError:
        return None

def set_cached_result(cache, program, defender_instance, iteration, pass_num, total_num):
    """
    Set cached result.
    """
    if program not in cache:
        cache[program] = {}
    if str(iteration) not in cache[program]:
        cache[program][str(iteration)] = {}
    if defender_instance not in cache[program][str(iteration)]:
        cache[program][str(iteration)][defender_instance] = {}
    
    cache[program][str(iteration)][defender_instance] = [pass_num, total_num]

def clear_cache():
    """
    Clear the cache file.
    """
    if os.path.exists(CACHE_FILE):
        os.remove(CACHE_FILE)
        print(f"[INFO] Cache file {CACHE_FILE} cleared")
    else:
        print(f"[INFO] No cache file to clear")


def absolute_comparison(result_instance):
    """
    For each iteration, we collect the results for the naive baseline.
    """
    import shutil
    import subprocess
    
    print("Starting cross comparison...")
    
    # Load cache
    cache = load_cache()
    
    # Store results: results[iter][defense_setting] = (defense_success, attack_success)
    results = {}
    results_per_program = {}
    
    for iter_num in [0, iteration_num]:
        print(f"\n[INFO] ==== Processing iteration {iter_num} ====")
        results[iter_num] = {}
        
        total_pass_number_per_setting = {i: 0 for i in settings}
        total_test_per_setting = {i: 0 for i in settings}

        for program in programs:
            print(f"[INFO] Processing program: {program}")
            
            pass_number_per_setting = {i: 0 for i in settings}
            test_per_setting = {i: 0 for i in settings}

            for defense_setting in settings:
                defense_instance = result_instance[program][defense_setting]
                
                print(f"[INFO] Processing: defense {defense_setting}")

                cached_result = get_cached_result(cache, program, defense_instance, iter_num)
                if cached_result is not None:
                    defense_success, total_num = cached_result
                    pass_number_per_setting[defense_setting] += defense_success
                    test_per_setting[defense_setting] += total_num
                    continue
                
                try:
                    # Get paths
                    defense_folder = backup_dir + result_instance[program][defense_setting] + f"/iteration_{iter_num}"
                    
                    defense_rs_files = os.path.join(defense_folder, "rs_files")
                    source_dir = os.path.join(input_dir, program)
                    
                    # Verify required files exist
                    assert os.path.exists(defense_rs_files), f"Defense Rust files not found: {defense_rs_files}"
                    assert os.path.exists(source_dir), f"Source dir not found: {source_dir}"
                    
                    # Clear the arena sandbox
                    if os.path.exists(arena_sandbox_dir):
                        shutil.rmtree(arena_sandbox_dir)
                    os.makedirs(arena_sandbox_dir)

                    # Copy the defense Rust files to the arena sandbox
                    _copy_directory(defense_rs_files, arena_sandbox_dir, whitelist=white_list_for_copy_rs)
                    _copy_directory(source_dir, arena_sandbox_dir, whitelist=white_list_for_copy_c)
                    _copy_directory(source_dir, arena_sandbox_dir, whitelist=white_list_for_copy_test_cases)

                    # Compile the C code
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

                    # Compile the Rust code
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
                    
                    # Run the tests
                    test_result = subprocess.run(
                        ["./testcmp.sh", "compare", f"./ts/target/release/{program}"], 
                        cwd=arena_sandbox_dir, 
                        capture_output=True, 
                        text=True
                    )
                    
                    # # Check if test execution was successful
                    # if test_result.returncode != 0:
                    #     print(f"[WARNING] Test execution failed for {defense_setting} {program} iter_{iter_num}")
                    #     print(f"Test error stdout: {test_result.stdout}")
                    #     print(f"Test error stderr: {test_result.stderr}")
                    #     # Still try to parse output in case there are partial results
                    
                    # Parse test results
                    output_text = test_result.stdout
                    
                    # Count total tests
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
                    
                    if total_tests == 0:
                        print(f"[WARNING] Could not parse total test count from output")
                        print(f"Output was: {output_text[:500]}...")  # Show first 200 chars for debugging
                    
                    # Alternative parsing for total tests from "Results:" line
                    passed_tests = 0
                    total_tests2 = None
                    for line in output_text.split('\n'):
                        if "Results:" in line and "passed" in line and "failed" in line and "out of" in line:
                            # Parse "Results: X passed, Y failed out of Z tests"
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
                    
                    # Only assert if we found both values
                    if total_tests2 is not None:
                        assert total_tests == total_tests2, f"Total tests mismatch: {total_tests} vs {total_tests2}"
                    else:
                        print(f"[WARNING] Could not parse Results line for verification")
                    failed_tests = total_tests - passed_tests
                    
                    # Validate results make sense
                    if total_tests == 0:
                        print(f"[WARNING] No tests found for {defense_setting} for {program} iter_{iter_num}")
                        defense_success = 0
                    elif passed_tests < 0 or failed_tests < 0:
                        print(f"[ERROR] Invalid test counts: passed={passed_tests}, failed={failed_tests}, total={total_tests}")
                        defense_success = 0
                    else:
                        # Defense success: tests that pass (attacker fails to break defender)
                        defense_success = passed_tests
                        # Attack success: tests that fail (attacker successfully breaks defender)  
                    
                    pass_number_per_setting[defense_setting] += defense_success
                    test_per_setting[defense_setting] += total_tests
                    
                    # Cache the result immediately
                    set_cached_result(cache, program, defense_instance, iter_num, defense_success, total_tests)
                    save_cache(cache)  # Save cache immediately after each test
                    
                    # print(f"[INFO] {attack_setting} vs {defense_setting} for {program} iter_{iter_num}: {defense_success}D/{attack_success}A")
                    
                except Exception as e:
                    print(f"[WARNING] Error testing {defense_setting} for {program} iter_{iter_num}: {e}")
                    continue
        
            # Add to totals
            for setting in settings:
                total_pass_number_per_setting[setting] += pass_number_per_setting[setting]
                total_test_per_setting[setting] += test_per_setting[setting]
            
            if iter_num == iteration_num: # only save the results for the last iteration
                results_per_program[program] = {i: (pass_number_per_setting[i], test_per_setting[i]) for i in settings}
        
        results[iter_num] = {i: (total_pass_number_per_setting[i], total_test_per_setting[i]) for i in settings}
    
    # # Print cache statistics
    # print(f"\n[CACHE STATS] Cache hits: {cache_hits}, Cache misses: {cache_misses}")
    # if cache_hits + cache_misses > 0:
    #     hit_rate = cache_hits / (cache_hits + cache_misses) * 100
    #     print(f"[CACHE STATS] Hit rate: {hit_rate:.1f}%")
    
    # Print results table
    
    print("="*20)
    print("Results Table: Defense/Attack Success by Setting and Iteration")
    print("="*20)
    
    # Print header
    header = "Setting".ljust(40)
    for iter_num in [0, 10]:
        header += f"Iter_{iter_num}".ljust(15)
    print(header)
    print("-" * len(header))
    
    # Print results for each setting
    for setting in settings:
        row = setting.ljust(40)
        for iter_num in [0, 10]:
            if iter_num in results:
                defense_num, attack_num = results[iter_num][setting]
                cell = f"{defense_num}D/{attack_num}A" + f" ({defense_num / attack_num * 100:.2f}%)"
                row += cell.ljust(15)
            else:
                row += "N/A".ljust(15)
        print(row)

    
    print("\nLegend: XD/YA means X defense successes, Y attack successes")
    print("Defense success = attacker's test passes on defender's code (defender holds)")  
    print("Attack success = attacker's test fails on defender's code (attacker breaks defender)")
    print("="*20)        

    ### print the results table for each program on last iteration
    print("="*20)
    print("Results Table: Defense/Attack Success by Setting on Last Iteration Across Programs")
    print("="*20)

    # Print header
    header = "Setting".ljust(25)
    for setting in settings:
        header += setting.ljust(25)
    print(header)
    print("-" * len(header))

    # Print results for each setting
    for program in programs:
        row = program.ljust(50)
        for setting in settings:
            defense_num, attack_num = results_per_program[program][setting]
            row += f"{defense_num}D/{attack_num}A".ljust(30)
        print(row)

def collect_result_instance():
    result_instance = {}
    for program in programs:
        result_instance[program] = {}
        for setting in settings:
            result_instance[program][setting] = []
            # Find the folder in .working/{program} whose .translation_state.json's discriminator matches the setting
            for subfolder in os.listdir(working_dir):
                if program in subfolder: ### match prefix of the folder name
                    folder_path = os.path.join(working_dir, subfolder)
                    state_file = os.path.join(folder_path, ".translation_state.json")
                    if os.path.isfile(state_file):
                        try:
                            with open(state_file, "r") as f:
                                import json
                                state = json.load(f)
                                translator = state.get("translator", "")
                                discriminator = state.get("discriminator", "")
                                cur_iter = state.get("current_iteration", 0)
                                if translator + "+" + discriminator == setting and cur_iter == iteration_num + 1:
                                    result_instance[program][setting].append(subfolder)
                                    # break
                        except Exception as e:
                            continue
    return result_instance

if __name__ == "__main__":
    ### Automatically obtain the result instance for each setting, in case you dont want to manually specify the instances
    result_instance = collect_result_instance()
    print(json.dumps(result_instance, indent=4))

    ### Validate the results using absolute pass rate
    absolute_comparison(result_instance)

    ### Collect the money/token cost for each setting
    print("[INFO] Collecting the money/token cost for each setting")
    print(f"[INFO] ----- {settings[0]} -----")
    collect_cost_cc([result_instance[program][settings[0]] for program in programs], "claude-sonnet-4-5-20250929")
    collect_time_cost([result_instance[program][settings[0]] for program in programs])

    # not supported in the latest Claude Code version
    # print(f"[INFO] -----{settings[1]} -----")
    # collect_cost_cc([result_instance[program][settings[1]] for program in programs], "claude-sonnet-4-20250514")
    # collect_time_cost([result_instance[program][settings[1]] for program in programs])

    print(f"[INFO] ----- {settings[1]} -----")
    collect_cost_swe([result_instance[program][settings[1]] for program in programs], "claude-sonnet-4-5-20250929")
    collect_time_cost([result_instance[program][settings[1]] for program in programs])

    print(f"[INFO] ----- {settings[2]} -----")
    collect_cost_swe([result_instance[program][settings[2]] for program in programs], "claude-sonnet-4-20250514")
    collect_time_cost([result_instance[program][settings[2]] for program in programs])
    
    print(f"[INFO] ----- {settings[3]} -----")
    collect_cost_swe([result_instance[program][settings[3]] for program in programs], "gpt-5-mini-2025-08-07")
    collect_time_cost([result_instance[program][settings[3]] for program in programs])




