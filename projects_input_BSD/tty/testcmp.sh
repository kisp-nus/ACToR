#!/bin/bash

# Test script for tty terminal checking utility program
# Usage: ./testcmp.sh [help|compare|select|coverage] [args...]
# Supports "alias_name" field in test JSON to run program under different names

REFERENCE_BINARY="./tty.ref"
SOURCE_FILE="tty.c"
# TESTS_JSONL will be auto-discovered

# Check if we're in the correct directory
if [ ! -f "$SOURCE_FILE" ]; then
    echo "Error: $SOURCE_FILE not found in current directory"
    echo "Please run this script from the directory containing $SOURCE_FILE"
    exit 1
fi

# Auto-discover testsXX.jsonl files
discover_test_files() {
    local test_files=()
    
    # Find all testsXX.jsonl files and sort them numerically
    for file in tests[0-9][0-9].jsonl; do
        if [ -f "$file" ]; then
            test_files+=("$file")
        fi
    done
    
    # Sort files numerically by the XX part
    IFS=$'\n' test_files=($(printf '%s\n' "${test_files[@]}" | sort -V))
    
    # Return the sorted array
    printf '%s\n' "${test_files[@]}"
}

# Validate that JSONL files end with newline
validate_jsonl_files() {
    local file
    for file in "${TESTS_FILES[@]}"; do
        echo "  Validating file ending of $file..."
        
        # Check if file is empty
        if [ ! -s "$file" ]; then
            echo "Error: JSONL file '$file' is empty"
            return 1
        fi
        
        # Check if file ends with newline
        if [ "$(tail -c 1 "$file" | wc -l)" -eq 0 ]; then
            echo "Error: JSONL file '$file' does not end with newline (\\n)"
            echo "Please add a newline at the end of the file"
            return 1
        fi
    done
    
    echo "  All JSONL files are properly formatted"
    return 0
}

# Get discovered test files
TESTS_FILES_OUTPUT=$(discover_test_files)
if [ -z "$TESTS_FILES_OUTPUT" ]; then
    echo "Error: No testsXX.jsonl files found in current directory"
    echo "Please ensure at least one file matching pattern testsXX.jsonl exists"
    echo "Examples: tests00.jsonl, tests01.jsonl, etc."
    exit 1
fi

# Convert output to array
TESTS_FILES=($TESTS_FILES_OUTPUT)

# Validate JSONL files format
echo "Validating JSONL files..."
if ! validate_jsonl_files; then
    exit 1
fi

# Check if jq is available for JSONL parsing
if ! command -v jq >/dev/null 2>&1; then
    echo "Error: jq is required for JSONL parsing but not found"
    echo "Please install jq: sudo apt-get install jq (or equivalent for your system)"
    exit 1
fi

# Load test data from JSONL files
declare -A TEST_NAMES
declare -A TEST_ARGS
declare -A TEST_ALIAS_NAMES
declare -A TEST_NORM_RULES
declare -a TEST_ORDER

load_tests_from_jsonl() {
    local line
    local test_name
    local description
    local args_json
    local file
    local file_content
    
    echo "Loading tests from: ${TESTS_FILES[*]}"
    
    # Read each file in order
    for file in "${TESTS_FILES[@]}"; do
        echo "  Processing $file..."
        
        # Read entire file content to handle files that may not end with newline
        file_content=$(cat "$file")
        
        # Process each line, using a more robust approach
        # This handles both files with and without final newlines
        while IFS= read -r line || [ -n "$line" ]; do
            # Skip empty lines
            [ -z "$line" ] && continue
            
            # Validate JSON format before parsing
            if ! echo "$line" | jq empty >/dev/null 2>&1; then
                echo "    Error: Invalid JSON in $file: $line"
                return 1
            fi
            
            # Get test name, description, args, alias_name, and norm_rules from each JSON line
            test_name=$(echo "$line" | jq -r '.name')
            description=$(echo "$line" | jq -r '.description')
            args_json=$(echo "$line" | jq -c '.args')
            alias_name=$(echo "$line" | jq -r '.alias_name // ""')
            norm_rules_json=$(echo "$line" | jq -c '.norm_rules // []')
            
            # Validate required fields
            if [ "$test_name" = "null" ] || [ -z "$test_name" ]; then
                echo "    Error: Missing or null 'name' field in $file: $line"
                return 1
            fi
            
            if [ "$description" = "null" ]; then
                description=""
            fi
            
            if [ "$args_json" = "null" ]; then
                args_json="[]"
            fi
            
            if [ "$norm_rules_json" = "null" ]; then
                norm_rules_json="[]"
            fi
            
            # Check if test name already exists
            if [ -n "${TEST_NAMES[$test_name]}" ]; then
                echo "    Warning: Test '$test_name' already exists, skipping duplicate"
                continue
            fi
            
            TEST_NAMES["$test_name"]="$description"
            TEST_ARGS["$test_name"]="$args_json"
            TEST_ALIAS_NAMES["$test_name"]="$alias_name"
            TEST_NORM_RULES["$test_name"]="$norm_rules_json"
            TEST_ORDER+=("$test_name")
        done <<< "$file_content"
    done
    
    echo "Loaded ${#TEST_NAMES[@]} tests total"
}

get_test_args() {
    local test_name="$1"
    local args_json="${TEST_ARGS[$test_name]}"
    
    # Convert JSON array to bash array
    # Use jq to output each array element on a separate line, then read into array
    local args=()
    while IFS= read -r arg; do
        args+=("$arg")
    done < <(echo "$args_json" | jq -r '.[]')
    
    # Return the array by printing each element
    # Fix: Only print if there are elements to avoid empty string for empty arrays
    if [ ${#args[@]} -gt 0 ]; then
        printf '%s\n' "${args[@]}"
    fi
}

# Load tests on script start
if ! load_tests_from_jsonl; then
    echo "Failed to load tests from JSONL files"
    exit 1
fi

print_help() {
    echo "Test script for tty terminal checking utility program"
    echo "Usage: $0 [command] [args...]"
    echo ""
    echo "Commands:"
    echo "  help                           - Show this help message"
    echo "  compare <path_to_binary>       - Compare binary with reference on all tests"
    echo "  select <path_to_binary> <test> - Run specific test and show diff"
    echo "  coverage                       - Show test coverage for reference binary"
    echo ""
    echo "Note: All tests have a 500ms timeout to prevent hanging"
    echo "      Binaries run under an alias name via symlink (defaults to 'main' if not specified in the test JSON)"
    echo ""
    echo "Available tests:"
    for test_name in "${TEST_ORDER[@]}"; do
        local alias_info=""
        local alias_name="${TEST_ALIAS_NAMES[$test_name]}"
        if [ -z "$alias_name" ] || [ "$alias_name" = "" ]; then
            alias_info="(as main - default)"
        else
            alias_info="(as $alias_name)"
        fi
        echo "    $test_name - ${TEST_NAMES[$test_name]} $alias_info"
    done
}

check_binary() {
    local binary="$1"
    
    if [ ! -f "$binary" ]; then
        echo "Error: Binary '$binary' not found"
        return 1
    fi
    
    if [ ! -x "$binary" ]; then
        echo "Error: Binary '$binary' does not have execution permission"
        return 1
    fi
    
    return 0
}

build_reference() {
    echo "Building reference binary..."
    make all
    if [ $? -ne 0 ]; then
        echo "Failed to build reference binary"
        exit 1
    fi
}

run_test() {
    local binary="$1"
    local test_name="$2"
    local actual_binary="$binary"
    
    # Get test arguments from JSON
    local args=()
    while IFS= read -r arg; do
        args+=("$arg")
    done < <(get_test_args "$test_name")
    
    # Use alias name, defaulting to 'main' if empty
    local alias_name="${TEST_ALIAS_NAMES[$test_name]}"
    if [ -z "$alias_name" ] || [ "$alias_name" = "" ]; then
        alias_name="main"
    fi

    rm -f "./$alias_name"
    
    # Always create alias binary by symlinking (preserves coverage instrumentation)
    if ln -s "$binary" "./$alias_name" 2>/dev/null; then
        actual_binary="./$alias_name"
    else
        echo "ALIAS_FAILED: Could not create alias '$alias_name' from '$binary'" >&2
        return 1
    fi
    
    # Run the test with timeout (500ms)
    if [ ${#args[@]} -eq 0 ]; then
        output=$(timeout 0.5 "$actual_binary" 2>&1)
    else
        output=$(timeout 0.5 "$actual_binary" "${args[@]}" 2>&1)
    fi
    actual_exit=$?
    
    # Cleanup alias binary if created
    rm -f "$actual_binary"
    
    # Check if timeout occurred
    if [ $actual_exit -eq 124 ]; then
        echo "TIMEOUT: Test '$test_name' exceeded 500ms limit" >&2
        return 124
    fi
    
    echo "$output"
    return $actual_exit
}

# Function to normalize output using rules from test
normalize_output() {
    local output="$1"
    local progname="$2"
    local test_name="$3"
    local normalized_output="$output"
    local norm_rules_json="${TEST_NORM_RULES[$test_name]}"
    
    # Apply normalization rules if they exist
    if [ "$norm_rules_json" != "[]" ] && [ -n "$norm_rules_json" ]; then
        # Parse the array of rules
        local rule_count=$(echo "$norm_rules_json" | jq 'length')
        
        for ((i=0; i<rule_count; i++)); do
            # Extract rule components using jq
            local pattern=$(echo "$norm_rules_json" | jq -r ".[$i].pattern")
            local replacement=$(echo "$norm_rules_json" | jq -r ".[$i].replacement")

            # Replace placeholders in pattern and replacement
            pattern="${pattern//\{progname\}/$progname}"
            replacement="${replacement//\{progname\}/$progname}"
            
            # Apply the rule using sed with global replacement
            normalized_output="$(echo "$normalized_output" | sed -E "s#${pattern}#${replacement}#g")"
        done
    fi
    
    echo "$normalized_output"
}

compare_binaries() {
    local test_binary="$1"
    
    # Check test binary
    if ! check_binary "$test_binary"; then
        exit 1
    fi
    
    if [ ! -f "$REFERENCE_BINARY" ]; then
        build_reference
    fi
    
    # Check reference binary
    if ! check_binary "$REFERENCE_BINARY"; then
        echo "Error: Reference binary '$REFERENCE_BINARY' is not valid"
        exit 1
    fi
    
    local passed=0
    local failed=0
    local total=${#TEST_ORDER[@]}
    local normal_exits=0
    
    echo "Comparing $test_binary with $REFERENCE_BINARY"
    echo "Running $total tests..."
    echo ""
    
    for test_name in "${TEST_ORDER[@]}"; do
        # Run test on reference binary
        ref_output=$(run_test "$REFERENCE_BINARY" "$test_name")
        ref_exit=$?
        
        # Run test on test binary
        test_output=$(run_test "$test_binary" "$test_name")
        test_exit=$?
        
        # Count normal exits
        if [ "$test_exit" -eq 0 ]; then
            ((normal_exits++))
        fi
        
        # Get alias name
        local progname="${TEST_ALIAS_NAMES[$test_name]}"
        if [ -z "$progname" ] || [ "$progname" = "" ]; then
            progname="main"
        fi
        
        # Normalize outputs for comparison
        local normalized_ref_output=$(normalize_output "$ref_output" "$progname" "$test_name")
        local normalized_test_output=$(normalize_output "$test_output" "$progname" "$test_name")
        
        # Compare results
        if [ "$ref_exit" -eq 124 ]; then
            echo "TIMEOUT: $test_name (reference binary)"
            ((failed++))
        elif [ "$test_exit" -eq 124 ]; then
            echo "TIMEOUT: $test_name (test binary)"
            ((failed++))
        elif [ "$ref_exit" -eq "$test_exit" ] && [ "$normalized_ref_output" = "$normalized_test_output" ]; then
            echo "PASS: $test_name"
            ((passed++))
        else
            echo "FAIL: $test_name"
            echo "  Reference exit: $ref_exit, Test exit: $test_exit"
            if [ "$normalized_ref_output" != "$normalized_test_output" ]; then
                echo "  Output differs"
            fi
            ((failed++))
        fi
    done
    
    echo ""
    echo "Results: $passed passed, $failed failed out of $total tests"
    
    if [ $normal_exits -eq 0 ]; then
        echo "WARNING: None of the tests resulted in normal exit (exit code 0) of the tested program"
    fi
    
    if [ $failed -eq 0 ]; then
        echo "All tests passed!"
        return 0
    else
        echo "Some tests failed!"
        return 1
    fi
}

select_test() {
    local test_binary="$1"
    local test_name="$2"
    
    # Check test binary
    if ! check_binary "$test_binary"; then
        exit 1
    fi
    
    if [ ! -f "$REFERENCE_BINARY" ]; then
        build_reference
    fi
    
    # Check reference binary
    if ! check_binary "$REFERENCE_BINARY"; then
        echo "Error: Reference binary '$REFERENCE_BINARY' is not valid"
        exit 1
    fi
    
    # Check if test exists
    if [ -z "${TEST_NAMES[$test_name]}" ]; then
        echo "Error: Test '$test_name' not found"
        echo "Available tests:"
        for name in "${TEST_ORDER[@]}"; do
            echo "  $name - ${TEST_NAMES[$name]}"
        done
        exit 1
    fi
    
    local args=()
    while IFS= read -r arg; do
        args+=("$arg")
    done < <(get_test_args "$test_name")
    
    echo "Running test: $test_name"
    echo "Description: ${TEST_NAMES[$test_name]}"
    echo "Arguments: ${args[*]}"
    echo ""
    
    # Run test on reference binary
    echo "=== Reference binary output ==="
    ref_output=$(run_test "$REFERENCE_BINARY" "$test_name")
    ref_exit=$?
    echo "Output omitted: will be compared below"
    echo "Exit code: $ref_exit"
    echo ""
    
    # Run test on test binary
    echo "=== Test binary output ==="
    test_output=$(run_test "$test_binary" "$test_name")
    test_exit=$?
    echo "Output omitted: will be compared below"
    echo "Exit code: $test_exit"
    echo ""
    
    # Get alias name
    local progname="${TEST_ALIAS_NAMES[$test_name]}"
    if [ -z "$progname" ] || [ "$progname" = "" ]; then
        progname="main"
    fi
    
    # Normalize outputs for comparison
    local normalized_ref_output=$(normalize_output "$ref_output" "$progname" "$test_name")
    local normalized_test_output=$(normalize_output "$test_output" "$progname" "$test_name")
    
    # Show diff
    echo "=== Comparison ==="
    if [ "$ref_exit" -eq 124 ]; then
        echo "TIMEOUT: Reference binary exceeded 500ms limit"
    elif [ "$test_exit" -eq 124 ]; then
        echo "TIMEOUT: Test binary exceeded 500ms limit"
    elif [ "$ref_exit" -eq "$test_exit" ] && [ "$normalized_ref_output" = "$normalized_test_output" ]; then
        # print out the normalized test output
        echo "(normalized) Test output:"
        echo "$normalized_test_output"
        echo "PASS: Test outputs match with reference"
    else
        echo "FAIL: Differences found"
        if [ "$ref_exit" -ne "$test_exit" ]; then
            echo "Exit codes differ: Reference=$ref_exit, Test=$test_exit"
        fi
        if [ "$normalized_ref_output" != "$normalized_test_output" ]; then
            echo "Output differs:"
            echo "--- Reference (normalized) ---"
            echo "$normalized_ref_output"
            echo "--- Test (normalized) ---"
            echo "$normalized_test_output"
            echo "--- End ---"
        fi
    fi
}

show_coverage() {
    # Build reference binary if it doesn't exist
    if [ ! -f "$REFERENCE_BINARY" ]; then
        build_reference
    fi
    
    # clean previous coverage data
    rm -f *.gcda *.gcov
    echo "Cleaned previous coverage data"
    
    echo "Running all tests to generate coverage data..."
    # Run all tests to generate coverage data
    for test_name in "${TEST_ORDER[@]}"; do
        printf "Running test: $test_name...\n"
        local output
        local exit_code
        output=$(run_test "$REFERENCE_BINARY" "$test_name")
        exit_code=$?
        if [ $exit_code -eq 124 ]; then
            printf " TIMEOUT (500ms exceeded)\n"
        else
            printf " exit_code: $exit_code\n"
        fi
    done
    
    echo "Generating coverage report..."
    # Generate coverage report for all source files (excluding util.c and system headers)
    if command -v gcov >/dev/null 2>&1; then
        local gcov_output
        local gcov_exit
        
        # Find .gcda files that correspond to .c files (excluding util.c) to handle multi-file programs
        local gcda_files=""
        for c_file in *.c; do
            if [ -f "$c_file" ] && [ "$c_file" != "util.c" ]; then
                # Convert .c filename to corresponding .gcda pattern (e.g., cp.c -> *.ref-cp.gcda)
                local base_name=$(basename "$c_file" .c)
                local matching_gcda=$(ls *.ref-${base_name}.gcda 2>/dev/null || echo "")
                if [ -n "$matching_gcda" ]; then
                    gcda_files="$gcda_files $matching_gcda"
                fi
            fi
        done
        
        # Trim leading/trailing whitespace
        gcda_files=$(echo "$gcda_files" | xargs)
        
        if [ -n "$gcda_files" ]; then
            # Generate coverage for selected files only
            gcov_output=$(gcov -b -c $gcda_files 2>&1)
            gcov_exit=${PIPESTATUS[0]}
            if [ $gcov_exit -ne 0 ]; then
                echo "Error: Failed to generate coverage data with gcov (exit code: $gcov_exit)"
                echo "gcov output: $gcov_output"
                return 1
            fi
            echo "Coverage report:"
            echo ""
            
            # Display the filtered coverage summary
            echo "=== Coverage Summary ==="
            echo "$gcov_output"
            echo ""
            echo "=== Detailed Coverage ==="
            echo "See coverage reports in *.gcov files (excluding system headers)"
        else
            echo "Error: No coverage data files (.gcda) found"
            echo "Make sure the binary was compiled with coverage flags and tests were run"
            return 1
        fi
    else
        echo "Error: gcov not found. Please install gcov for coverage analysis."
        return 1
    fi
}

# Main script logic
case "${1:-help}" in
    "help"|"")
        print_help
        ;;
    "compare")
        if [ $# -ne 2 ]; then
            echo "Usage: $0 compare <path_to_binary>"
            exit 1
        fi
        compare_binaries "$2"
        ;;
    "select")
        if [ $# -ne 3 ]; then
            echo "Usage: $0 select <path_to_binary> <test_name>"
            exit 1
        fi
        select_test "$2" "$3"
        ;;
    "coverage")
        if [ $# -ne 1 ]; then
            echo "Usage: $0 coverage"
            exit 1
        fi
        show_coverage
        ;;
    *)
        echo "Unknown command: $1"
        print_help
        exit 1
        ;;
esac
