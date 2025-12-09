# claudix-sandv2.py - Test Guide

This document provides step-by-step commands to test the three restart features of `claudix-sandv2.py` with the new proxy argument syntax.

## Prerequisites

- Ensure `sand` CLI is available in PATH
- Ensure `claude` CLI is available
- The `lproc.py` script is executable and in your PATH or current directory

## New Proxy Argument Syntax

The proxy now supports passing arguments using `::` as separator:

```bash
# Default sandbox config (uses default 'sand.config')
./lproc.py -s task1 "[proxies/claudix-sandv2.py]"

# Specify sandbox config (sand1.config)
./lproc.py -s task2 "[proxies/claudix-sandv2.py::sand1]"

# Pass additional claude arguments
./lproc.py -s task4 "[proxies/claudix-sandv2.py::sand::--help]"
```

## Test Setup

All tests should be run from the `_lproc` directory:

```bash
cd /data/__utils/_lproc
```

---

## Test 1: Normal Operation (No Restart)

**Purpose**: Verify the proxy works normally without restart commands.

```bash
# Start the LProc with claudix-sandv2
./lproc.py -s test1 "[proxies/claudix-sandv2.py]"

# Send a normal user message
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"What is 2+2?"}]}}' >> .lproc/test1.stdin

# Wait a moment for response
sleep 3

# Check the output (should see normal claude response)
tail -20 .lproc/test1.stdout

# Check stderr for proxy logs
tail -20 .lproc/test1.stderr

# Kill the LProc
./lproc.py -k test1

# Clean up
./lproc.py -d test1
```

**Expected behavior**:
- Claude should start normally (stderr shows PID)
- Normal response to "What is 2+2?"
- No restart messages

---

## Test 2: [CLAUDIX:RESTART] - Wait and Send

**Purpose**: Test graceful restart with message forwarding.

```bash
# Start the LProc
./lproc.py -s test2 "[proxies/claudix-sandv2.py]"

# Send first message (will get a result)
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"Count to 3"}]}}' >> .lproc/test2.stdin

# Wait for response
# sleep 3

# Send RESTART command with a new task
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"[CLAUDIX:RESTART] What is the capital of France?"}]}}' >> .lproc/test2.stdin

# Wait for restart to complete
sleep 5

# Check stderr for restart logs
tail -30 .lproc/test2.stderr | grep -E "\[claudix-sandv2\]"

# Check stdout for responses
tail -40 .lproc/test2.stdout

# Clean up
./lproc.py -k test2
./lproc.py -d test2
```

**Expected behavior**:
- First message gets normal response
- Stderr shows: "RESTART requested; waiting for result..."
- Stderr shows: "All results received; restarting claude..."
- Stderr shows: "Sending modified message to new instance (counted as user message)"
- New claude instance receives: "What is the capital of France?" (without [CLAUDIX:RESTART])
- Response to the modified message

---

## Test 3: [CLAUDIX:FORCE_RESTART] - Immediate Kill with Send

**Purpose**: Test forced restart when claude might be stuck.

```bash
# Start the LProc
./lproc.py -s test3 "[proxies/claudix-sandv2.py]"

# Send first message
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"Start counting to 100"}]}}' >> .lproc/test3.stdin

# IMMEDIATELY send FORCE_RESTART (don't wait for result)
sleep 0.5
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"[CLAUDIX:FORCE_RESTART] What is 5*5?"}]}}' >> .lproc/test3.stdin

# Wait for processing
sleep 5

# Check stderr for force restart logs
tail -40 .lproc/test3.stderr | grep -E "\[claudix-sandv2\]"

# Check stdout - should see failure injection
tail -50 .lproc/test3.stdout | grep -E "(CLAUDIX_FAIL|result)"

# Clean up
./lproc.py -k test3
./lproc.py -d test3
```

**Expected behavior**:
- First message might not get a complete response
- Stderr shows: "FORCE_RESTART requested; killing claude immediately (missing N results)..."
- Stdout shows N injected failure messages: `{"type":"result","subtype":"CLAUDIX_FAIL",...}`
- Stderr shows: "Injected N failure messages and balanced counters"
- Stderr shows: "Restarting claude..."
- Stderr shows: "Sending modified message to new instance (counted as user message)"
- New claude instance receives: "What is 5*5?" (without [CLAUDIX:FORCE_RESTART])
- Response to "What is 5*5?"

---

## Test 4: [CLAUDIX:FORCE_RESTART_NO_SEND] - Kill and Wait

**Purpose**: Test forced restart that discards the current message.

```bash
# Start the LProc
./lproc.py -s test4 "[proxies/claudix-sandv2.py]"

# Send first message
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"Write a long essay"}]}}' >> .lproc/test4.stdin

# IMMEDIATELY send FORCE_RESTART_NO_SEND (don't wait)
sleep 0.5
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"[CLAUDIX:FORCE_RESTART_NO_SEND] This message should not be sent"}]}}' >> .lproc/test4.stdin

# Wait for restart
sleep 3

# Check stderr
tail -40 .lproc/test4.stderr | grep -E "\[claudix-sandv2\]"

# Check stdout - should see failure messages plus reminder
tail -50 .lproc/test4.stdout | jq -r '.result' 2>/dev/null || tail -50 .lproc/test4.stdout | grep -o '"result":"[^"]*"'

# Send a new message after restart
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"What is the weather like?"}]}}' >> .lproc/test4.stdin

# Wait and check response
sleep 5
tail -20 .lproc/test4.stdout

# Clean up
./lproc.py -k test4
./lproc.py -d test4
```

**Expected behavior**:
- First message might not complete
- Stderr shows: "FORCE_RESTART_NO_SEND requested; killing claude immediately (missing N results)..."
- Stdout shows N failure messages (one per missing result)
- Stdout shows additional reminder: `"result":"Claude Code was force-restarted. The previous message was NOT sent..."`
- Stderr shows: "Injected reminder message about NO_SEND"
- Stderr shows: "Restarting claude and waiting for new input..."
- The message "This message should not be sent" is NOT forwarded to claude
- New message "What is the weather like?" gets a normal response

---

## Test 5: Multiple Missing Results

**Purpose**: Test failure injection when multiple results are missing.

```bash
# Start the LProc
./lproc.py -s test5 "[proxies/claudix-sandv2.py]"

# Send THREE messages quickly (without waiting for results)
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"Message 1"}]}}' >> .lproc/test5.stdin
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"Message 2"}]}}' >> .lproc/test5.stdin
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"Message 3"}]}}' >> .lproc/test5.stdin

# Wait just a tiny bit (not enough for all results)
sleep 1

# Send FORCE_RESTART
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"[CLAUDIX:FORCE_RESTART] Continue"}]}}' >> .lproc/test5.stdin

# Wait for processing
sleep 5

# Check stderr - should show missing count
tail -40 .lproc/test5.stderr | grep -E "missing.*results"

# Count injected failure messages
tail -50 .lproc/test5.stdout | grep -c "CLAUDIX_FAIL"

# Clean up
./lproc.py -k test5
./lproc.py -d test5
```

**Expected behavior**:
- Stderr shows: "killing claude immediately (missing 2-3 results)..." (depends on how many completed)
- Stdout shows 2-3 failure messages (one for each missing result)
- All failures have identical text
- Counters are balanced after injection

---

## Test 6: Verify Command Pattern Removal

**Purpose**: Confirm that [CLAUDIX:*] patterns are properly stripped before forwarding.

```bash
# Start the LProc
./lproc.py -s test6 "[proxies/claudix-sandv2.py]"

# Send a RESTART message with text before and after
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"Please help me. [CLAUDIX:RESTART] Continue with the analysis of the code."}]}}' >> .lproc/test6.stdin

# Wait for restart
sleep 5

# Use a converter to see the assistant's response (if available)
# Or check raw output
tail -30 .lproc/test6.stdout | grep '"type":"assistant"'

# The assistant should have received:
# "Please help me.  Continue with the analysis of the code."
# (note: pattern is removed, extra spaces might remain depending on implementation)

# Clean up
./lproc.py -k test6
./lproc.py -d test6
```

**Expected behavior**:
- The `[CLAUDIX:RESTART]` pattern is removed from the text
- Claude receives the cleaned message
- Response is relevant to the cleaned text

---

## Test 7: [CLAUDIX:FORCE_RESTART_RESUME] - Kill and Resume Session

**Purpose**: Test forced restart with session resumption to preserve conversation context.

```bash
# Start the LProc
./lproc.py -s test7 "[proxies/claudix-sandv2.py]"

# Send first message to establish a session
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"My name is Alice. Remember this."}]}}' >> .lproc/test7.stdin

# Wait for response and session_id
sleep 5

# Check that session_id was captured
tail -50 .lproc/test7.stdout | grep -o '"session_id":"[^"]*"' | head -1

# Send second message (to establish context)
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"Start counting to 100"}]}}' >> .lproc/test7.stdin

# IMMEDIATELY send FORCE_RESTART_RESUME (don't wait for counting to finish)
sleep 1
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"[CLAUDIX:FORCE_RESTART_RESUME] What is my name?"}]}}' >> .lproc/test7.stdin

# Wait for processing
sleep 5

# Check stderr for resume logs
tail -50 .lproc/test7.stderr | grep -E "FORCE_RESTART_RESUME|--resume"

# Check if claude remembered the name (context preserved)
tail -30 .lproc/test7.stdout | grep -i alice

# Clean up
./lproc.py -k test7
./lproc.py -d test7
```

**Expected behavior**:
- First message establishes session, session_id is captured
- Stderr shows: "FORCE_RESTART_RESUME requested; killing and resuming (missing N results)..."
- Stderr shows: "restarted with --resume <session_id>; new PID XXXXX: ... --resume <session_id>"
- Failure messages injected for interrupted counting
- New claude instance should remember "Alice" (context preserved via --resume)
- Response to "What is my name?" should mention Alice

---

## Test 8: [CLAUDIX:FORCE_RESTART_RESUME_NO_SEND] - Kill, Resume, Don't Send

**Purpose**: Test forced restart with resume but without sending the trigger message.

```bash
# Start the LProc
./lproc.py -s test8 "[proxies/claudix-sandv2.py]"

# Establish session with context
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"The secret code is BLUE42. Remember this."}]}}' >> .lproc/test8.stdin

# Wait for response
sleep 5

# Send a long-running message
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"Write a very long story"}]}}' >> .lproc/test8.stdin

# IMMEDIATELY send FORCE_RESTART_RESUME_NO_SEND
sleep 1
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"[CLAUDIX:FORCE_RESTART_RESUME_NO_SEND] This message should not be sent"}]}}' >> .lproc/test8.stdin

# Wait for restart
sleep 3

# Check stderr
tail -50 .lproc/test8.stderr | grep -E "FORCE_RESTART_RESUME_NO_SEND|--resume|NO_SEND"

# Check stdout - should see reminder about message not being sent
tail -30 .lproc/test8.stdout | grep "was NOT sent"

# Send a new message to verify session was resumed
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"What was the secret code?"}]}}' >> .lproc/test8.stdin

# Wait and check response - should remember BLUE42
sleep 5
tail -20 .lproc/test8.stdout | grep -i "blue42"

# Clean up
./lproc.py -k test8
./lproc.py -d test8
```

**Expected behavior**:
- Session established with "secret code"
- Stderr shows: "FORCE_RESTART_RESUME_NO_SEND requested; killing and resuming..."
- Stdout shows reminder: "Claude Code was force-restarted with resume. The previous message was NOT sent..."
- Stderr shows: "restarted with --resume <session_id>..."
- Message "This message should not be sent" is NOT forwarded
- New message "What was the secret code?" gets response
- Claude remembers "BLUE42" (context preserved via --resume)

---

## Test 9: Session Tracking Verification

**Purpose**: Verify that session_id is properly tracked from stdout messages.

```bash
# Start the LProc
./lproc.py -s test9 "[proxies/claudix-sandv2.py]"

# Send first message
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"Hello"}]}}' >> .lproc/test9.stdin

# Wait for init message with session_id
sleep 3

# Extract session_id from stdout
SESSION_ID=$(tail -50 .lproc/test9.stdout | grep -o '"session_id":"[^"]*"' | head -1 | cut -d'"' -f4)
echo "Captured session_id: $SESSION_ID"

# Trigger a FORCE_RESTART_RESUME
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"[CLAUDIX:FORCE_RESTART_RESUME] Continue"}]}}' >> .lproc/test9.stdin

# Wait for restart
sleep 3

# Verify the session_id appears in the restart command
tail -30 .lproc/test9.stderr | grep "restarted with --resume $SESSION_ID"

# Clean up
./lproc.py -k test9
./lproc.py -d test9
```

**Expected behavior**:
- session_id appears in initial stdout (system/init message)
- Proxy captures and tracks the session_id
- On FORCE_RESTART_RESUME, the exact session_id is used in --resume flag
- Stderr shows the full command with --resume <captured-session-id>

---

## Test 10: RESUME Without Session (Graceful Fallback)

**Purpose**: Test RESUME behavior when no session_id has been captured yet.

```bash
# Start the LProc
./lproc.py -s test10 "[proxies/claudix-sandv2.py]"

# IMMEDIATELY send FORCE_RESTART_RESUME before any session is established
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"[CLAUDIX:FORCE_RESTART_RESUME] Test early resume"}]}}' >> .lproc/test10.stdin

# Wait for processing
sleep 3

# Check stderr for warning about no session_id
tail -30 .lproc/test10.stderr | grep "No session_id tracked"

# Verify it fell back to normal restart (no --resume flag)
tail -30 .lproc/test10.stderr | grep "restarted; new PID" | grep -v "\--resume"

# Clean up
./lproc.py -k test10
./lproc.py -d test10
```

**Expected behavior**:
- Stderr shows: "Warning: No session_id tracked yet, restarting without resume"
- Restart happens normally without --resume flag
- No crash or error
- Message is still sent to new instance

---

## Debugging Tips

### View real-time stderr (in separate terminal):
```bash
tail -f .lproc/test*.stderr
```

### View real-time stdout (in separate terminal):
```bash
tail -f .lproc/test*.stdout
```

### Pretty-print stdout with converter:
```bash
tail -50 .lproc/test1.stdout | ./converters/cc__stdout_r.py --color
```

### Check for failure messages:
```bash
tail -100 .lproc/test*.stdout | grep -A2 "CLAUDIX_FAIL"
```

### Monitor counter messages:
```bash
tail -100 .lproc/test*.stderr | grep "balanced counters"
```

### Check session_id tracking:
```bash
# Extract captured session_id from stdout
tail -100 .lproc/test*.stdout | grep -o '"session_id":"[^"]*"' | head -1

# Verify --resume flag in restart command
tail -50 .lproc/test*.stderr | grep "restarted with --resume"

# Check for session tracking warnings
tail -50 .lproc/test*.stderr | grep "session_id"
```

### Verify context preservation (RESUME tests):
```bash
# Check if claude remembers information after resume
tail -50 .lproc/test7.stdout | grep -i "alice"
tail -50 .lproc/test8.stdout | grep -i "blue42"
```

---

## Expected Log Patterns

### RESTART logs:
```
[claudix-sandv2] RESTART requested; waiting for result... 2s
[claudix-sandv2] All results received; restarting claude...
[claudix-sandv2] restarted; new PID XXXXX: sand --config ...
[claudix-sandv2] Sending modified message to new instance (counted as user message)
```

### FORCE_RESTART logs:
```
[claudix-sandv2] FORCE_RESTART requested; killing claude immediately (missing N results)...
[claudix-sandv2] Injected N failure messages and balanced counters
[claudix-sandv2] Restarting claude...
[claudix-sandv2] restarted; new PID XXXXX: sand --config ...
[claudix-sandv2] Sending modified message to new instance (counted as user message)
```

### FORCE_RESTART_NO_SEND logs:
```
[claudix-sandv2] FORCE_RESTART_NO_SEND requested; killing claude immediately (missing N results)...
[claudix-sandv2] Injected N failure messages and balanced counters
[claudix-sandv2] Injected reminder message about NO_SEND
[claudix-sandv2] Restarting claude and waiting for new input...
```

### FORCE_RESTART_RESUME logs:
```
[claudix-sandv2] FORCE_RESTART_RESUME requested; killing and resuming (missing N results)...
[claudix-sandv2] Injected N failure messages and balanced counters
[claudix-sandv2] Restarting claude with --resume...
[claudix-sandv2] restarted with --resume abc-123-session-id; new PID XXXXX: sand --config ... --resume abc-123-session-id
[claudix-sandv2] Sending modified message to new instance (counted as user message)
```

### FORCE_RESTART_RESUME_NO_SEND logs:
```
[claudix-sandv2] FORCE_RESTART_RESUME_NO_SEND requested; killing and resuming (missing N results)...
[claudix-sandv2] Injected N failure messages and balanced counters
[claudix-sandv2] Injected reminder message about NO_SEND with resume
[claudix-sandv2] Restarting claude with --resume and waiting for new input...
[claudix-sandv2] restarted with --resume abc-123-session-id; new PID XXXXX: sand --config ... --resume abc-123-session-id
```

### Session tracking:
```
# When session_id is first received from claude
# (automatically tracked, no explicit log)

# When RESUME command is used without session_id
[claudix-sandv2] Warning: No session_id tracked yet, restarting without resume
[claudix-sandv2] restarted; new PID XXXXX: sand --config ...
```

---

## Troubleshooting

### If sand is not found:
```bash
# Check if sand is in PATH
which sand

# If not, ensure sand is installed and add to PATH
```

### If LProc fails to start:
```bash
# Check if lptail symlink exists
ls -la lptail

# Create if missing
ln -s /usr/bin/tail lptail
```

### If messages aren't being processed:
```bash
# Check if claude is running
./lproc.py -l

# Check stderr for errors
cat .lproc/testX.stderr
```

### Clean all test LProcs:
```bash
for i in test1 test2 test3 test4 test5 test6 test7 test8 test9 test10; do
    ./lproc.py -k $i 2>/dev/null || true
    ./lproc.py -d $i 2>/dev/null || true
done
```

---

## Summary

The test suite covers:
1. ✅ Normal operation without restart
2. ✅ Graceful restart with message forwarding (RESTART)
3. ✅ Forced restart with message forwarding (FORCE_RESTART)
4. ✅ Forced restart without message forwarding (FORCE_RESTART_NO_SEND)
5. ✅ Multiple missing results handling
6. ✅ Command pattern removal verification
7. ✅ Forced restart with session resume (FORCE_RESTART_RESUME)
8. ✅ Forced restart with resume, no send (FORCE_RESTART_RESUME_NO_SEND)
9. ✅ Session ID tracking verification
10. ✅ RESUME graceful fallback (no session_id)

Each test validates:
- Correct stderr logging
- Proper failure message injection
- Counter management
- Message modification and forwarding
- New claude instance startup
- Session ID tracking and --resume flag usage (Tests 7-10)
- Context preservation across restarts (Tests 7-8)
