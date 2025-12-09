#!/usr/bin/env python3
"""
Binary Fuzzing Template Script
Template for comparing outputs between two binaries using grammar-based input generation

Usage:
1. Customize the get_next_input() function to generate inputs according to your grammar
2. Run with: python fuzzer_template.py <binary1> <binary2>
"""

import subprocess
import random
import sys
import resource


def get_next_input():
    """
    Generator function that yields the next input case for testing.
    
    TODO: Customize this function to implement your specific input grammar.
    
    This function should yield lists of strings representing command line arguments.
    Each yielded list will be passed as arguments to both binaries for comparison.
    
    Example patterns you might implement:
    - Mathematical expressions: ['1', '+', '2']
    - File paths: ['/path/to/file']
    - Configuration options: ['--verbose', '--output', 'file.txt']
    - Network addresses: ['192.168.1.1', '8080']
    - JSON strings: ['{"key": "value"}']
    
    Design considerations:
    - If files are involved, you should create temporary files together with input arguments and clean them up after the test.
    - Include edge cases (empty inputs, boundary values, invalid syntax)
    - Consider both valid and invalid inputs to test error handling
    - Use randomization for better coverage
    - Include exhaustive testing for small input spaces
    - Add progression from simple to complex cases
    - Memory usage is limited to 5GB per process to prevent system overload
    - When running the fuzzer script, you should use `timeout` to set the timeout to 10~30 seconds.
    """
    
    # ============================================================================
    # TODO: IMPLEMENT YOUR INPUT GENERATION LOGIC HERE
    # ============================================================================
    
    # Example 1: Simple counter (replace with your logic)
    for i in range(10):
        yield [str(i)]
    
    # Example 2: Random combinations (replace with your logic)
    operators = ['+', '-', '*', '/']
    for _ in range(20):
        num1 = random.randint(1, 10)
        op = random.choice(operators)
        num2 = random.randint(1, 10)
        yield [str(num1), op, str(num2)]
    
    # Example 3: Edge cases (customize for your domain)
    edge_cases = [
        [],  # Empty input
        ['0'],  # Boundary value
        ['invalid_input'],  # Invalid input
        # Add more edge cases specific to your binaries
    ]
    
    for case in edge_cases:
        yield case
    
    # ============================================================================
    # END OF CUSTOMIZATION SECTION
    # ============================================================================


def set_memory_limit(limit_gb=10):
    """
    Set memory limit for child processes to prevent excessive memory usage.
    
    Args:
        limit_gb (int): Memory limit in GB (default: 10GB)
    """
    try:
        # Set memory limit in bytes (limit_gb * 1024^3)
        memory_limit = limit_gb * 1024 * 1024 * 1024
        
        # Set both virtual memory and resident set size limits
        resource.setrlimit(resource.RLIMIT_AS, (memory_limit, memory_limit))
        resource.setrlimit(resource.RLIMIT_RSS, (memory_limit, memory_limit))
        
    except (ValueError, OSError) as e:
        print(f"Warning: Could not set memory limit: {e}")
        print("Proceeding without memory limits...")


def input_runner(binary1_path, binary2_path):
    """
    Main runner function that:
    1. Gets input cases from get_next_input()
    2. Runs both binaries with each input
    3. Compares outputs (stdout, stderr, exit codes)
    4. Reports mismatches
    
    This function generally doesn't need modification unless you want to:
    - Change the comparison logic
    - Modify timeout settings
    - Add performance measurements
    - Change output format
    """
    # Set memory limits for safety
    set_memory_limit(5)  # 5GB limit
    
    mismatch_count = 0
    total_tests = 0
    
    print(f"Starting binary comparison fuzzing:")
    print(f"  Binary 1: {binary1_path}")
    print(f"  Binary 2: {binary2_path}")
    print("-" * 60)
    
    try:
        for input_case in get_next_input():
            total_tests += 1
            
            # Convert input case to string for display
            input_str = " ".join(input_case) if input_case else ""
            
            # Run first binary
            try:
                result1 = subprocess.run(
                    [binary1_path] + input_case,
                    capture_output=True,
                    text=True,
                    timeout=0.1,  # TODO: Adjust timeout as needed
                    preexec_fn=lambda: set_memory_limit(5)  # Apply memory limit to child process
                )
                output1 = result1.stdout.strip()
                stderr1 = result1.stderr.strip()
                exit_code1 = result1.returncode
            except subprocess.TimeoutExpired:
                output1 = "TIMEOUT"
                stderr1 = ""
                exit_code1 = -1
            except Exception as e:
                output1 = f"EXECUTION_ERROR: {e}"
                stderr1 = ""
                exit_code1 = -2
            
            # Run second binary
            try:
                result2 = subprocess.run(
                    [binary2_path] + input_case,
                    capture_output=True,
                    text=True,
                    timeout=0.1,  # TODO: Adjust timeout as needed
                    preexec_fn=lambda: set_memory_limit(5)  # Apply memory limit to child process
                )
                output2 = result2.stdout.strip()
                stderr2 = result2.stderr.strip()
                exit_code2 = result2.returncode
            except subprocess.TimeoutExpired:
                output2 = "TIMEOUT"
                stderr2 = ""
                exit_code2 = -1
            except Exception as e:
                output2 = f"EXECUTION_ERROR: {e}"
                stderr2 = ""
                exit_code2 = -2
            
            # Compare outputs
            # TODO: Customize comparison logic if needed
            def normalize(output:str) -> str:
                return output.replace("./main:", "").replace("main:", "")

            stdout_match = normalize(output1) == normalize(output2)
            stderr_match = stderr1 == stderr2
            exit_code_match = exit_code1 == exit_code2
            
            if not (stdout_match and stderr_match and exit_code_match):
                mismatch_count += 1
                print(f"MISMATCH #{mismatch_count}:")
                print(f"  Input: {input_str}")
                
                if not stdout_match:
                    print(f"  Binary 1 stdout: '{output1}'")
                    print(f"  Binary 2 stdout: '{output2}'")
                
                if not stderr_match:
                    print(f"  Binary 1 stderr: '{stderr1}'")
                    print(f"  Binary 2 stderr: '{stderr2}'")
                
                if not exit_code_match:
                    print(f"  Binary 1 exit code: {exit_code1}")
                    print(f"  Binary 2 exit code: {exit_code2}")
                
                print()
            
            # Progress indicator
            if total_tests % 50 == 0:
                print(f"Progress: {total_tests} tests completed, {mismatch_count} mismatches found")
    
    except KeyboardInterrupt:
        print("\nFuzzing interrupted by user.")
    
    print("-" * 60)
    print(f"Fuzzing complete!")
    print(f"Total tests: {total_tests}")
    print(f"Mismatches found: {mismatch_count}")
    if total_tests > 0:
        print(f"Mismatch rate: {(mismatch_count/total_tests)*100:.2f}%")
    
    return mismatch_count > 0


def main():
    """
    Main entry point - handles command line arguments and starts fuzzing
    """
    if len(sys.argv) != 3:
        print("Usage: python fuzzer_template.py <binary1_path> <binary2_path>")
        print()
        print("Examples:")
        print("  python fuzzer_template.py ./binary_c ./binary_rs")
        print("  python fuzzer_template.py ./binary_c ./ts/target/release/binary_rs")
        print()
        print("This script will:")
        print("1. Generate test inputs using the get_next_input() function")
        print("2. Run both binaries with each input")
        print("3. Compare outputs and report any differences")
        print()
        print("Customize the get_next_input() function to match your testing needs.")
        sys.exit(1)
    
    binary1 = sys.argv[1]
    binary2 = sys.argv[2]
    
    # Validate that binaries exist and are executable
    import os
    if not os.path.isfile(binary1):
        print(f"Error: Binary 1 '{binary1}' not found or not a file")
        sys.exit(1)
    if not os.access(binary1, os.X_OK):
        print(f"Error: Binary 1 '{binary1}' is not executable")
        sys.exit(1)
    if not os.path.isfile(binary2):
        print(f"Error: Binary 2 '{binary2}' not found or not a file")
        sys.exit(1)
    if not os.access(binary2, os.X_OK):
        print(f"Error: Binary 2 '{binary2}' is not executable")
        sys.exit(1)
    
    # Run the fuzzing
    found_mismatches = input_runner(binary1, binary2)
    
    # Exit with appropriate code
    sys.exit(1 if found_mismatches else 0)


if __name__ == "__main__":
    main()
