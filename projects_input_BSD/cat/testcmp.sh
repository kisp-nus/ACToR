#!/bin/bash

# Test script for cat file concatenation program
# Usage: ./testcmp.sh [help|compare|select|coverage] [args...]
# Supports "alias_name" field in test JSON to run program under different names

REFERENCE_BINARY="./cat.ref"
SOURCE_FILE="cat.c"
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
declare -A TEST_CMD_PREP
declare -A TEST_CMD_TARGET
declare -A TEST_CMD_POST
declare -A TEST_ALIAS_NAME
declare -A TEST_CHECK_FILE
declare -A TEST_NORM_RULES
declare -a TEST_ORDER

load_tests_from_jsonl() {
    local line
    local test_name
    local description
    local cmd_prep
    local cmd_target
    local cmd_post
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
            
            # Get test name, description, commands, check_file, alias_name, and norm_rules from each JSON line
            test_name=$(echo "$line" | jq -r '.name')
            description=$(echo "$line" | jq -r '.description')
            cmd_prep=$(echo "$line" | jq -r '.cmd_prep // ""')
            # Decide how to parse cmd_target based on check_file
            check_file=$(echo "$line" | jq -r '.check_file // false')
            if [ "$check_file" = "true" ]; then
                cmd_target=$(echo "$line" | jq -c '.cmd_target')
            else
                cmd_target=$(echo "$line" | jq -r '.cmd_target')
            fi
            cmd_post=$(echo "$line" | jq -r '.cmd_post // ""')
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
            
            if [ "$cmd_target" = "null" ] || [ -z "$cmd_target" ]; then
                echo "    Error: Missing or null 'cmd_target' field in $file: $line"
                return 1
            fi
            
            if [ "$cmd_prep" = "null" ]; then
                cmd_prep=""
            fi
            
            if [ "$cmd_post" = "null" ]; then
                cmd_post=""
            fi
            
            if [ "$alias_name" = "null" ]; then
                alias_name=""
            fi
            
            if [ "$check_file" = "null" ]; then
                check_file="false"
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
            TEST_CMD_PREP["$test_name"]="$cmd_prep"
            TEST_CMD_TARGET["$test_name"]="$cmd_target"
            TEST_CMD_POST["$test_name"]="$cmd_post"
            TEST_ALIAS_NAME["$test_name"]="$alias_name"
            TEST_CHECK_FILE["$test_name"]="$check_file"
            TEST_NORM_RULES["$test_name"]="$norm_rules_json"
            TEST_ORDER+=("$test_name")
        done <<< "$file_content"
    done
    
    echo "Loaded ${#TEST_NAMES[@]} tests total"
}


get_test_commands() {
    local test_name="$1"
    local cmd_type="$2"  # prep, target, post, alias, or check_file
    
    case "$cmd_type" in
        "prep")
            echo "${TEST_CMD_PREP[$test_name]}"
            ;;
        "target")
            echo "${TEST_CMD_TARGET[$test_name]}"
            ;;
        "post")
            echo "${TEST_CMD_POST[$test_name]}"
            ;;
        "alias")
            echo "${TEST_ALIAS_NAME[$test_name]}"
            ;;
        "check_file")
            echo "${TEST_CHECK_FILE[$test_name]}"
            ;;
        *)
            echo "Error: Invalid command type '$cmd_type'" >&2
            return 1
            ;;
    esac
}

# Load tests on script start
if ! load_tests_from_jsonl; then
    echo "Failed to load tests from JSONL files"
    exit 1
fi

print_help() {
    echo "Test script for cat file concatenation program"
    echo "Usage: $0 [command] [args...]"
    echo ""
    echo "Commands:"
    echo "  help                           - Show this help message"
    echo "  compare <path_to_binary>       - Compare binary with reference on all tests"
    echo "  select <path_to_binary> <test> - Run specific test and show diff"
    echo "  coverage                       - Show test coverage for reference binary"
    echo ""
    echo "Note: Target commands have a 500ms timeout to prevent hanging"
    echo "      Test format includes prep/target/post commands for file operations"
    echo "      Binaries run under an alias name via symlink (defaults to 'main' if not specified in the test JSON)"
    echo ""
    echo "Available tests:"
    for test_name in "${TEST_ORDER[@]}"; do
        local alias_info=""
        local alias_name="${TEST_ALIAS_NAME[$test_name]}"
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
    
    # Get test commands and alias name
    local cmd_prep=$(get_test_commands "$test_name" "prep")
    local cmd_target=$(get_test_commands "$test_name" "target")
    local cmd_post=$(get_test_commands "$test_name" "post")
    local alias_name=$(get_test_commands "$test_name" "alias")
    local check_file=$(get_test_commands "$test_name" "check_file")
    
    local actual_binary="$binary"
    
    # Use alias name, defaulting to 'main' if empty
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
    
    local prep_output=""
    local target_output=""
    local post_output=""
    local target_exit=0
    
    # Run prep command if it exists
    if [ -n "$cmd_prep" ] && [ "$cmd_prep" != "null" ]; then
        prep_output=$(eval "$cmd_prep" 2>&1)
        local prep_exit=$?
        if [ $prep_exit -ne 0 ]; then
            # Clean up alias before returning
                rm -f "./$alias_name" 2>/dev/null
            echo "PREP_FAILED: $prep_output" >&2
            return $prep_exit
        fi
    fi
    
    # Handle cmd_target based on check_file flag
    if [ "$check_file" = "true" ]; then
        # cmd_target is a JSON array, extract first element for main command
        local main_cmd=$(echo "$cmd_target" | jq -r '.[0]')
        main_cmd="${main_cmd//BINARY/$actual_binary}"
        
        # Run main command with timeout (500ms)
        if [ -n "$main_cmd" ] && [ "$main_cmd" != "null" ]; then
            target_output=$(timeout 0.5 bash -c "$main_cmd" 2>&1)
        target_exit=$?
        
        # Check if timeout occurred
        if [ $target_exit -eq 124 ]; then
            # Run cleanup before returning
            if [ -n "$cmd_post" ] && [ "$cmd_post" != "null" ]; then
                    post_output=$(eval "$cmd_post" 2>&1)
                    # debug
                    # printf "POST_OUTPUT: $post_output\n"
            fi
            # Clean up alias before returning
                rm -f "./$alias_name" 2>/dev/null
                echo "TIMEOUT: Test '$test_name' exceeded 500ms limit" >&2
                return 124
            fi
        fi
        
        # Extract file content check commands (remaining elements)
        local file_check_count=$(echo "$cmd_target" | jq '. | length')
        if [ "$file_check_count" -gt 1 ]; then
            # Append file contents to target_output
            local i=1
            while [ $i -lt "$file_check_count" ]; do
                local check_cmd=$(echo "$cmd_target" | jq -r ".[$i]")
                # only allow commands having cat in it
                if ! echo "$check_cmd" | grep -q 'cat'; then
                    echo "Error: Only cat command is supported for file content checking"
                    rm -f "./$alias_name" 2>/dev/null
                    return 1
                fi
                if [ -n "$check_cmd" ] && [ "$check_cmd" != "null" ]; then
                    local file_content=$(eval "$check_cmd" 2>&1)
                    if [ $i -eq 1 ]; then
                        target_output="$target_output"$'\n'"FILE_CONTENT_START:"$'\n'"$file_content"
                    else
                        target_output="$target_output"$'\n'"$file_content"
                    fi
                fi
                ((i++))
            done
        fi
    else
        # Traditional mode: cmd_target is a simple raw string command
        local simple_cmd="$cmd_target"
        simple_cmd="${simple_cmd//BINARY/$actual_binary}"
        
        # Run target command with timeout (500ms)
        if [ -n "$simple_cmd" ] && [ "$simple_cmd" != "null" ]; then
            target_output=$(timeout 0.5 bash -c "$simple_cmd" 2>&1)
            target_exit=$?
            
            # Check if timeout occurred
            if [ $target_exit -eq 124 ]; then
                # Run cleanup before returning
                if [ -n "$cmd_post" ] && [ "$cmd_post" != "null" ]; then
                    post_output=$(eval "$cmd_post" 2>&1)
                    # debug
                    # printf "POST_OUTPUT: $post_output\n"
                fi
                # Clean up alias before returning
                rm -f "./$alias_name" 2>/dev/null
            echo "TIMEOUT: Test '$test_name' exceeded 500ms limit" >&2
            return 124
            fi
        fi
    fi
    
    # Run post command if it exists (cleanup)
    if [ -n "$cmd_post" ] && [ "$cmd_post" != "null" ]; then
        post_output=$(eval "$cmd_post" 2>&1)
        # debug
        # printf "POST_OUTPUT: $post_output\n"
        # We don't check post command exit code since it's cleanup
    fi
    
    # Clean up alias at the end
        rm -f "./$alias_name" 2>/dev/null
    
    echo "$target_output"
    return $target_exit
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
        
        # Get alias name
        local progname="${TEST_ALIAS_NAME[$test_name]}"
        if [ -z "$progname" ] || [ "$progname" = "" ]; then
            progname="main"
        fi
        
        # Run test on test binary
        test_output=$(run_test "$test_binary" "$test_name")
        test_exit=$?
        
        # Count normal exits
        if [ "$test_exit" -eq 0 ]; then
            ((normal_exits++))
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
    
    local cmd_prep=$(get_test_commands "$test_name" "prep")
    local cmd_target=$(get_test_commands "$test_name" "target")
    local cmd_post=$(get_test_commands "$test_name" "post")
    local progname=$(get_test_commands "$test_name" "alias")
    
    # Get alias name
    if [ -z "$progname" ] || [ "$progname" = "" ]; then
        progname="main"
    fi
    
    echo "Running test: $test_name"
    echo "Description: ${TEST_NAMES[$test_name]}"
    echo "Prep command: $cmd_prep"
    echo "Target command: $cmd_target"
    echo "Post command: $cmd_post"
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
        printf "Running test: $test_name..."
        local output
        local exit_code
        output=$(run_test "$REFERENCE_BINARY" "$test_name" 2>&1)
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
