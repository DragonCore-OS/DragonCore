#!/bin/bash
# DragonCore Public Beta Verification Script
# 
# This script verifies all core functionality before public beta release.
# Run this on a clean machine to ensure installation and basic operations work.

set -e

DRAGONCORE_BIN="./target/release/dragoncore-runtime"
TEST_RUN_ID="beta-test-$(date +%s)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

pass_count=0
fail_count=0

# Helper functions
pass() {
    echo -e "${GREEN}✓ PASS${NC}: $1"
    ((pass_count++))
}

fail() {
    echo -e "${RED}✗ FAIL${NC}: $1"
    ((fail_count++))
}

warn() {
    echo -e "${YELLOW}⚠ WARN${NC}: $1"
}

# Test 1: Binary exists
echo "=== Test 1: Binary Exists ==="
if [ -f "$DRAGONCORE_BIN" ]; then
    pass "Binary found at $DRAGONCORE_BIN"
else
    fail "Binary not found. Run 'cargo build --release' first."
    exit 1
fi

# Test 2: Version command
echo ""
echo "=== Test 2: Version Command ==="
if $DRAGONCORE_BIN --version > /dev/null 2>&1; then
    VERSION=$($DRAGONCORE_BIN --version 2>&1)
    pass "Version command works: $VERSION"
else
    fail "Version command failed"
fi

# Test 3: Help command
echo ""
echo "=== Test 3: Help Command ==="
if $DRAGONCORE_BIN --help > /dev/null 2>&1; then
    pass "Help command works"
else
    fail "Help command failed"
fi

# Test 4: List seats command
echo ""
echo "=== Test 4: List Seats ==="
if $DRAGONCORE_BIN seats 2>&1 | grep -q "天枢"; then
    pass "Seats command works (found Chinese names)"
else
    fail "Seats command failed"
fi

# Test 5: Config initialization
echo ""
echo "=== Test 5: Config Initialization ==="
TEMP_DIR=$(mktemp -d)
if $DRAGONCORE_BIN --config "$TEMP_DIR/dragoncore.toml" init 2>&1; then
    if [ -f "$TEMP_DIR/dragoncore.toml" ]; then
        pass "Config initialization works"
    else
        fail "Config file not created"
    fi
else
    fail "Config init command failed"
fi
rm -rf "$TEMP_DIR"

# Test 6: Check dependencies
echo ""
echo "=== Test 6: Dependencies Check ==="

# Check tmux
if command -v tmux &> /dev/null; then
    TMUX_VERSION=$(tmux -V)
    pass "tmux installed: $TMUX_VERSION"
else
    warn "tmux not installed (required for isolation)"
fi

# Check git
if command -v git &> /dev/null; then
    GIT_VERSION=$(git --version)
    pass "git installed: $GIT_VERSION"
else
    fail "git not installed (required)"
fi

# Test 7: Runtime state directories
echo ""
echo "=== Test 7: Runtime State Structure ==="
if [ -d "./runtime_state" ]; then
    pass "runtime_state directory exists"
    
    if [ -d "./runtime_state/runs" ]; then
        pass "runs subdirectory exists"
    else
        warn "runs subdirectory missing"
    fi
    
    if [ -d "./runtime_state/events" ]; then
        pass "events subdirectory exists (DIBL)"
    else
        warn "events subdirectory missing"
    fi
    
    if [ -d "./runtime_state/ledger" ]; then
        pass "ledger subdirectory exists"
    else
        warn "ledger subdirectory missing"
    fi
else
    warn "runtime_state directory not found (will be created on first run)"
fi

# Test 8: DIBL test vectors
echo ""
echo "=== Test 8: DIBL Test Vectors ==="
if [ -f "./test_vectors/dragoncore_sample.jsonl" ]; then
    EVENT_COUNT=$(wc -l < "./test_vectors/dragoncore_sample.jsonl")
    pass "DragonCore test vector exists ($EVENT_COUNT events)"
else
    warn "DragonCore test vector missing"
fi

if [ -f "./test_vectors/axi_sample.jsonl" ]; then
    EVENT_COUNT=$(wc -l < "./test_vectors/axi_sample.jsonl")
    pass "AXI test vector exists ($EVENT_COUNT events)"
else
    warn "AXI test vector missing"
fi

# Test 9: Documentation
echo ""
echo "=== Test 9: Documentation ==="

if [ -f "./README.md" ]; then
    pass "README.md exists"
else
    warn "README.md missing"
fi

if [ -f "./STATUS.md" ]; then
    pass "STATUS.md exists"
else
    warn "STATUS.md missing"
fi

if [ -f "./DIBL_STATUS.md" ]; then
    pass "DIBL_STATUS.md exists"
else
    warn "DIBL_STATUS.md missing"
fi

if [ -f "./PUBLIC_BETA_ROADMAP.md" ]; then
    pass "PUBLIC_BETA_ROADMAP.md exists"
else
    warn "PUBLIC_BETA_ROADMAP.md missing"
fi

# Test 10: Schema validation (if jq available)
echo ""
echo "=== Test 10: JSON Schema Validation ==="
if command -v jq &> /dev/null; then
    if [ -f "./test_vectors/dragoncore_sample.jsonl" ]; then
        # Check first line is valid JSON
        if head -1 "./test_vectors/dragoncore_sample.jsonl" | jq . > /dev/null 2>&1; then
            pass "Test vector JSON is valid"
            
            # Check required fields
            FIRST_LINE=$(head -1 "./test_vectors/dragoncore_sample.jsonl")
            REQUIRED_FIELDS=("event_id" "run_id" "channel" "event_type" "scope" "actor")
            ALL_FIELDS_PRESENT=true
            
            for field in "${REQUIRED_FIELDS[@]}"; do
                if ! echo "$FIRST_LINE" | jq -e ".$field" > /dev/null 2>&1; then
                    warn "Missing field: $field"
                    ALL_FIELDS_PRESENT=false
                fi
            done
            
            if [ "$ALL_FIELDS_PRESENT" = true ]; then
                pass "All required DIBL fields present"
            fi
        else
            fail "Test vector JSON is invalid"
        fi
    fi
else
    warn "jq not installed, skipping JSON validation"
fi

# Summary
echo ""
echo "======================================"
echo "Verification Summary"
echo "======================================"
echo -e "Passed: ${GREEN}$pass_count${NC}"
echo -e "Failed: ${RED}$fail_count${NC}"

if [ $fail_count -eq 0 ]; then
    echo ""
    echo -e "${GREEN}✓ All critical tests passed!${NC}"
    echo "Ready for public beta."
    exit 0
else
    echo ""
    echo -e "${RED}✗ Some tests failed.${NC}"
    echo "Please fix issues before public beta."
    exit 1
fi
