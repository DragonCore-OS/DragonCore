#!/bin/bash
# DragonCore Beta Release Verification Script
# 
# This is a HARD GATE - all checks must pass before tagging beta.
# Run this script before any beta release.

set -e

DRAGONCORE_BIN="./target/release/dragoncore-runtime"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

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

info() {
    echo "ℹ INFO: $1"
}

# Check 1: No compiler warnings
echo "======================================"
echo "CHECK 1: Compiler Warnings (MUST BE 0)"
echo "======================================"

WARNING_COUNT=$(cargo build --release 2>&1 | grep -c "^warning:" || true)

if [ "$WARNING_COUNT" -eq 0 ]; then
    pass "No compiler warnings"
else
    fail "Found $WARNING_COUNT compiler warnings"
    cargo build --release 2>&1 | grep "^warning:" | head -10
fi

# Check 2: All tests pass
echo ""
echo "======================================"
echo "CHECK 2: All Tests Must Pass"
echo "======================================"

if cargo test 2>&1 | grep -q "test result: ok"; then
    TEST_COUNT=$(cargo test 2>&1 | grep "test result:" | grep -oP '\d+ passed' | grep -oP '\d+')
    pass "All tests passed ($TEST_COUNT tests)"
else
    fail "Some tests failed"
    cargo test 2>&1 | grep "test result:"
fi

# Check 3: Binary builds
echo ""
echo "======================================"
echo "CHECK 3: Release Binary Exists"
echo "======================================"

if [ -f "$DRAGONCORE_BIN" ]; then
    BIN_SIZE=$(du -h "$DRAGONCORE_BIN" | cut -f1)
    pass "Release binary exists ($BIN_SIZE)"
else
    fail "Release binary not found at $DRAGONCORE_BIN"
    echo "Run: cargo build --release"
fi

# Check 4: CLI commands work
echo ""
echo "======================================"
echo "CHECK 4: CLI Commands"
echo "======================================"

# Version
if $DRAGONCORE_BIN --version > /dev/null 2>&1; then
    VERSION=$($DRAGONCORE_BIN --version 2>&1)
    pass "Version command: $VERSION"
else
    fail "Version command failed"
fi

# Seats
if $DRAGONCORE_BIN seats 2>&1 | grep -q "天枢"; then
    pass "Seats command works"
else
    fail "Seats command failed"
fi

# Check 5: DIBL test vectors
echo ""
echo "======================================"
echo "CHECK 5: DIBL Test Vectors"
echo "======================================"

SAMPLE_RUNS=("sample-run-created" "sample-risk-veto" "sample-archive")
for run_id in "${SAMPLE_RUNS[@]}"; do
    if [ -f "./test_vectors/${run_id}.jsonl" ]; then
        EVENT_COUNT=$(wc -l < "./test_vectors/${run_id}.jsonl")
        pass "Sample run '$run_id' exists ($EVENT_COUNT events)"
        
        # Validate JSON format
        if command -v jq &> /dev/null; then
            if jq -e . "./test_vectors/${run_id}.jsonl" > /dev/null 2>&1; then
                pass "Sample run '$run_id' has valid JSON"
            else
                fail "Sample run '$run_id' has invalid JSON"
            fi
        fi
    else
        fail "Sample run '$run_id' not found"
    fi
done

# Check 6: AXI interop sample
echo ""
echo "======================================"
echo "CHECK 6: AXI Interop Sample"
echo "======================================"

if [ -f "./test_vectors/axi_sample.jsonl" ]; then
    pass "AXI sample exists"
    
    # Validate it can be parsed
    if command -v jq &> /dev/null; then
        if head -1 "./test_vectors/axi_sample.jsonl" | jq -e '.actor' > /dev/null 2>&1; then
            pass "AXI sample has actor field"
        else
            fail "AXI sample missing actor field"
        fi
    fi
else
    fail "AXI sample not found"
fi

# Check 7: Documentation
echo ""
echo "======================================"
echo "CHECK 7: Required Documentation"
echo "======================================"

REQUIRED_DOCS=(
    "README.md"
    "STATUS.md"
    "DIBL_STATUS.md"
    "PUBLIC_BETA_ROADMAP.md"
    "src/events/README.md"
)

for doc in "${REQUIRED_DOCS[@]}"; do
    if [ -f "$doc" ]; then
        pass "Documentation: $doc"
    else
        fail "Missing documentation: $doc"
    fi
done

# Check 8: Runtime state directories
echo ""
echo "======================================"
echo "CHECK 8: Runtime State Structure"
echo "======================================"

if [ -d "./runtime_state" ]; then
    pass "runtime_state directory exists"
    
    for subdir in runs events ledger; do
        if [ -d "./runtime_state/$subdir" ]; then
            pass "Subdirectory: $subdir"
        else
            warn "Missing subdirectory: $subdir (will be auto-created)"
        fi
    done
else
    warn "runtime_state directory not found (will be auto-created on first run)"
fi

# Check 9: Events CLI commands
echo ""
echo "======================================"
echo "CHECK 9: DIBL CLI Commands"
echo "======================================"

# Test events command help
if $DRAGONCORE_BIN events --help > /dev/null 2>&1; then
    pass "Events command exists"
else
    fail "Events command not found"
fi

# Test replay command help
if $DRAGONCORE_BIN replay --help > /dev/null 2>&1; then
    pass "Replay command exists"
else
    fail "Replay command not found"
fi

# Check 10: Test sample run projection
echo ""
echo "======================================"
echo "CHECK 10: Sample Run Projection"
echo "======================================"

# Copy sample to runtime_state/events for testing
mkdir -p ./runtime_state/events
cp ./test_vectors/sample-archive.jsonl ./runtime_state/events/sample-archive.jsonl

if $DRAGONCORE_BIN events --run-id sample-archive 2>&1 | grep -q "Events for run"; then
    pass "Events CLI displays sample run"
else
    fail "Events CLI failed to display sample run"
fi

if $DRAGONCORE_BIN replay --run-id sample-archive 2>&1 | grep -q "Replayed"; then
    pass "Replay CLI works on sample run"
else
    fail "Replay CLI failed on sample run"
fi

# Summary
echo ""
echo "======================================"
echo "VERIFICATION SUMMARY"
echo "======================================"
echo -e "Passed: ${GREEN}$pass_count${NC}"
echo -e "Failed: ${RED}$fail_count${NC}"

if [ $fail_count -eq 0 ]; then
    echo ""
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Ready for beta release."
    echo ""
    echo "Next steps:"
    echo "  1. Tag release: git tag v0.3.0-beta.1"
    echo "  2. Push tag: git push origin v0.3.0-beta.1"
    echo "  3. Create GitHub release"
    exit 0
else
    echo ""
    echo -e "${RED}✗ VERIFICATION FAILED${NC}"
    echo "Fix all failures before beta release."
    exit 1
fi
