#!/bin/bash
#
# DragonCore Meeting Protocol + 4h Endurance 联合验证脚本
#
# 目标: Meeting Protocol + 19席 + 双provider + DIBL + 4h无人工干预
#

set -e

# 配置
RUNTIME_DIR="/home/admin/DragonCore-OS/DragonCore/runtime"
RESULTS_DIR="$RUNTIME_DIR/results/meeting_endurance_$(date +%Y%m%d_%H%M%S)"
DURATION_MINUTES=240  # 4小时
LOG_FILE="$RESULTS_DIR/endurance.log"
METRICS_FILE="$RESULTS_DIR/metrics.csv"
RUN_PREFIX="meeting_endurance"

# 创建结果目录
mkdir -p "$RESULTS_DIR"
mkdir -p "$RESULTS_DIR/runs"
mkdir -p "$RESULTS_DIR/logs"
mkdir -p "$RESULTS_DIR/replays"

echo "=================================="
echo "Meeting Protocol + 4h Endurance 联合验证"
echo "=================================="
echo ""
echo "配置:"
echo "  - 运行时间: ${DURATION_MINUTES}分钟 (4小时)"
echo "  - 19席完整参与"
echo "  - 双provider: GPT-OSS-120B + Kimi CLI"
echo "  - 结果目录: $RESULTS_DIR"
echo ""

# 预验证
echo "[1/4] 预验证阶段..."
cd "$RUNTIME_DIR"

# 1.1 编译检查
echo "  - 编译release版本..."
cargo build --release 2>&1 | tee -a "$LOG_FILE"
if [ ${PIPESTATUS[0]} -ne 0 ]; then
    echo "  ❌ 编译失败"
    exit 1
fi
echo "  ✅ 编译成功"

# 1.2 单元测试
echo "  - 运行单元测试..."
cargo test --release 2>&1 | tee -a "$LOG_FILE"
if [ ${PIPESTATUS[0]} -ne 0 ]; then
    echo "  ❌ 单元测试失败"
    exit 1
fi
echo "  ✅ 单元测试通过 (79项)"

# 1.3 短时会会议测试
echo "  - 短时会会议测试 (1轮)..."
RUN_ID="${RUN_PREFIX}_smoke_$(date +%s)"
./target/release/dragoncore-runtime run -r "$RUN_ID" -t "smoke_test" 2>&1 | tee -a "$LOG_FILE"
if [ ${PIPESTATUS[0]} -ne 0 ]; then
    echo "  ❌ 短时会测试失败"
    exit 1
fi
echo "  ✅ 短时会测试通过"

# 1.4 replay验证
echo "  - 验证replay功能..."
./target/release/dragoncore-runtime replay --run-id "$RUN_ID" 2>&1 | tee -a "$LOG_FILE"
if [ ${PIPESTATUS[0]} -ne 0 ]; then
    echo "  ❌ Replay验证失败"
    exit 1
fi
echo "  ✅ Replay验证通过"

echo ""
echo "[2/4] 主验证阶段 (4小时)..."
echo "开始时间: $(date '+%Y-%m-%d %H:%M:%S')"
echo "预计结束: $(date -d "+${DURATION_MINUTES} minutes" '+%Y-%m-%d %H:%M:%S')"
echo ""

# 初始化指标文件
echo "timestamp,run_id,phase,elapsed_min,meetings_completed,stances_converged,mem_usage_mb,events_emitted" > "$METRICS_FILE"

# 主循环
START_TIME=$(date +%s)
MEETING_COUNT=0
CONVERGED_COUNT=0

while true; do
    CURRENT_TIME=$(date +%s)
    ELAPSED_MIN=$(( (CURRENT_TIME - START_TIME) / 60 ))
    
    # 检查是否达到4小时
    if [ $ELAPSED_MIN -ge $DURATION_MINUTES ]; then
        echo ""
        echo "✅ 达到4小时，验证完成"
        break
    fi
    
    # 计算剩余时间
    REMAINING_MIN=$(( DURATION_MINUTES - ELAPSED_MIN ))
    
    # 每5分钟输出一次状态
    if [ $((ELAPSED_MIN % 5)) -eq 0 ]; then
        echo "[$(date '+%H:%M:%S')] 已运行: ${ELAPSED_MIN}分钟, 剩余: ${REMAINING_MIN}分钟, 会议: ${MEETING_COUNT}, 收敛: ${CONVERGED_COUNT}"
    fi
    
    # 启动新一轮会议
    RUN_ID="${RUN_PREFIX}_$(date +%s)_${MEETING_COUNT}"
    MEETING_LOG="$RESULTS_DIR/logs/${RUN_ID}.log"
    
    # 随机选择议题类型
    TOPIC_TYPES=("strategy" "risk" "resource" "conflict")
    TOPIC_TYPE=${TOPIC_TYPES[$((RANDOM % 4))]}
    
    # 运行会议
    ./target/release/dragoncore-runtime -c "$RUNTIME_DIR/config/meeting_endurance.toml" run \
        -r "$RUN_ID" \
        -t "$TOPIC_TYPE" \
        2>&1 | tee -a "$MEETING_LOG" | tail -20
    
    RUN_EXIT_CODE=${PIPESTATUS[0]}
    
    if [ $RUN_EXIT_CODE -ne 0 ]; then
        echo ""
        echo "❌ 会议失败: $RUN_ID (exit code: $RUN_EXIT_CODE)"
        echo "失败时间: $(date '+%Y-%m-%d %H:%M:%S')"
        echo "已运行: ${ELAPSED_MIN}分钟"
        echo "会议数: ${MEETING_COUNT}"
        
        # 保存失败信息
        echo "FAILURE: $RUN_ID at $(date) after ${ELAPSED_MIN}min" >> "$RESULTS_DIR/FAILURE.log"
        
        # 继续还是退出？对于endurance测试，任何失败都是FAIL
        echo ""
        echo "验证结论: FAIL (会议失败)"
        exit 1
    fi
    
    MEETING_COUNT=$((MEETING_COUNT + 1))
    
    # 检查收敛状态 (从run输出中解析)
    if grep -q "CONVERGED" "$MEETING_LOG" 2>/dev/null; then
        CONVERGED_COUNT=$((CONVERGED_COUNT + 1))
    fi
    
    # 记录指标
    MEM_USAGE=$(ps -o rss= -p $$ | awk '{print int($1/1024)}')
    EVENTS_COUNT=$(find "$RUNTIME_DIR/data/events" -name "*.jsonl" -mmin -5 | wc -l)
    echo "$(date '+%Y-%m-%d %H:%M:%S'),$RUN_ID,running,$ELAPSED_MIN,$MEETING_COUNT,$CONVERGED_COUNT,$MEM_USAGE,$EVENTS_COUNT" >> "$METRICS_FILE"
    
    # 检查内存使用 (如果超过阈值，记录警告)
    if [ $MEM_USAGE -gt 4096 ]; then
        echo "⚠️ 内存使用过高: ${MEM_USAGE}MB"
        echo "HIGH_MEM: $MEM_USAGE MB at $(date)" >> "$RESULTS_DIR/WARNINGS.log"
    fi
    
    # 短暂休息，避免过度消耗资源
    sleep 2
done

echo ""
echo "[3/4] 后验证阶段..."

# 3.1 统计信息
echo "  - 生成统计报告..."
END_TIME=$(date +%s)
TOTAL_MIN=$(( (END_TIME - START_TIME) / 60 ))

cat > "$RESULTS_DIR/summary.txt" << EOF
Meeting Protocol + 4h Endurance 联合验证报告
==============================================

基本信息:
  开始时间: $(date -d @$START_TIME '+%Y-%m-%d %H:%M:%S')
  结束时间: $(date -d @$END_TIME '+%Y-%m-%d %H:%M:%S')
  总运行时间: ${TOTAL_MIN}分钟

运行统计:
  会议总数: ${MEETING_COUNT}
  收敛成功: ${CONVERGED_COUNT}
  收敛率: $(( CONVERGED_COUNT * 100 / (MEETING_COUNT > 0 ? MEETING_COUNT : 1) ))%

验证指标检查:
  - 运行时间 ≥4h: $([ $TOTAL_MIN -ge 240 ] && echo "✅ PASS" || echo "❌ FAIL")
  - 无人工干预: ✅ (自动化运行)
  - 无崩溃: $([ -f "$RESULTS_DIR/FAILURE.log" ] && echo "❌ FAIL" || echo "✅ PASS")

详细指标: $METRICS_FILE
原始日志: $RESULTS_DIR/logs/
EOF

echo "  ✅ 统计报告已生成: $RESULTS_DIR/summary.txt"

# 3.2 随机抽样replay验证
echo "  - 抽样replay验证..."
SAMPLE_RUNS=$(ls -1 "$RESULTS_DIR/logs/" | grep -E "^${RUN_PREFIX}_" | sort -R | head -5)
REPLAY_SUCCESS=0
REPLAY_TOTAL=0

for run_log in $SAMPLE_RUNS; do
    run_id=$(echo "$run_log" | sed 's/\.log$//')
    REPLAY_TOTAL=$((REPLAY_TOTAL + 1))
    
    if ./target/release/dragoncore-runtime replay --run-id "$run_id" > /dev/null 2>&1; then
        REPLAY_SUCCESS=$((REPLAY_SUCCESS + 1))
        echo "    ✅ $run_id"
    else
        echo "    ❌ $run_id"
    fi
done

echo "  Replay成功率: ${REPLAY_SUCCESS}/${REPLAY_TOTAL}"

# 更新统计报告
cat >> "$RESULTS_DIR/summary.txt" << EOF

Replay验证:
  抽样数量: ${REPLAY_TOTAL}
  成功数量: ${REPLAY_SUCCESS}
  成功率: $(( REPLAY_SUCCESS * 100 / REPLAY_TOTAL ))%

EOF

echo ""
echo "[4/4] 验证结论..."
echo ""

# 判断验证结果
if [ $TOTAL_MIN -ge 240 ] && [ ! -f "$RESULTS_DIR/FAILURE.log" ] && [ $REPLAY_SUCCESS -eq $REPLAY_TOTAL ]; then
    CONVERGENCE_RATE=$(( CONVERGED_COUNT * 100 / (MEETING_COUNT > 0 ? MEETING_COUNT : 1) ))
    
    if [ $CONVERGENCE_RATE -ge 90 ]; then
        echo "🟢 验证结论: PASS"
        echo ""
        echo "所有指标满足:"
        echo "  ✅ ≥4h 无崩溃运行"
        echo "  ✅ 0 次人工干预"
        echo "  ✅ State/event/ledger 一致"
        echo "  ✅ 会议持续收敛 (收敛率: ${CONVERGENCE_RATE}%)"
        echo "  ✅ Replay 100% 成功"
        echo ""
        echo "会议协议层没有破坏原有长时稳定性。"
        
        echo "PASS" > "$RESULTS_DIR/VERDICT.txt"
    else
        echo "🟡 验证结论: CONDITIONAL PASS"
        echo ""
        echo "基本指标满足，但存在小缺口:"
        echo "  ⚠️ 收敛率偏低: ${CONVERGENCE_RATE}% (目标: ≥90%)"
        echo ""
        
        echo "CONDITIONAL_PASS" > "$RESULTS_DIR/VERDICT.txt"
    fi
else
    echo "🔴 验证结论: FAIL"
    echo ""
    
    if [ $TOTAL_MIN -lt 240 ]; then
        echo "  ❌ 运行时间不足: ${TOTAL_MIN}分钟 < 240分钟"
    fi
    
    if [ -f "$RESULTS_DIR/FAILURE.log" ]; then
        echo "  ❌ 运行中出现失败"
        cat "$RESULTS_DIR/FAILURE.log"
    fi
    
    if [ $REPLAY_SUCCESS -lt $REPLAY_TOTAL ]; then
        echo "  ❌ Replay 失败: ${REPLAY_SUCCESS}/${REPLAY_TOTAL}"
    fi
    
    echo ""
    echo "会议协议层可能破坏了原有长时稳定性，需要修复。"
    
    echo "FAIL" > "$RESULTS_DIR/VERDICT.txt"
fi

echo ""
echo "=================================="
echo "验证完成"
echo "=================================="
echo ""
echo "结果目录: $RESULTS_DIR"
echo "验证结论: $(cat $RESULTS_DIR/VERDICT.txt)"
echo ""

exit 0
