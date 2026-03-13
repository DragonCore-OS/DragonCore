#!/bin/bash
# DragonCore Launch Script
# Governance-First Operating System for Multi-Agent AI

set -e

echo "🐉 DragonCore — Governance-First OS for Multi-Agent AI"
echo "======================================================="
echo ""

# Check environment
if [ -z "$KIMI_API_KEY" ] && [ -z "$DEEPSEEK_API_KEY" ] && [ -z "$DASHSCOPE_API_KEY" ]; then
    echo "⚠️  Warning: No domestic model API key configured."
    echo "Please set one of:"
    echo "  - KIMI_API_KEY (Moonshot AI)"
    echo "  - DEEPSEEK_API_KEY (DeepSeek)"
    echo "  - DASHSCOPE_API_KEY (Aliyun Qwen)"
    exit 1
fi

# Create run directory
RUN_ID="RUN-$(date +%Y%m%d)-$(printf '%03d' $(( $(ls -1 ./runs 2>/dev/null | wc -l) + 1 ))))"
mkdir -p "runs/$RUN_ID"

echo "📁 Created run: $RUN_ID"
echo ""
echo "Next steps:"
echo "1. Define your task in runs/$RUN_ID/task.md"
echo "2. Run governance flow: ./scripts/governance.sh $RUN_ID"
echo "3. Check decision log: runs/$RUN_ID/decision_log.md"
echo ""
echo "🐉 Dragon, not Claw."
echo "真龙，不是龙虾。"
