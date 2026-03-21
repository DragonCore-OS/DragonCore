# Phase 3 執行計劃

## 選項 A: 預測試 (推薦先執行)

**時長**: 15 分鐘  
**目的**: 驗證 endurance runner 腳本正確性

```bash
python3 endurance_runner.py --duration 15 --interval 60
```

**預期結果**:
- 創建 ~15 runs
- 執行 ~45 seat calls
- 生成 metrics 文件
- 無錯誤終止

## 選項 B: 完整 4 小時測試

**時長**: 4 小時 (240 分鐘)  
**命令**:

```bash
# 後台執行，記錄日誌
nohup python3 endurance_runner.py --duration 240 --interval 300 > /tmp/endurance_nohup.log 2>&1 &

# 查看實時狀態
tail -f /tmp/endurance_live.log
tail -f /tmp/endurance_metrics.jsonl
```

**預期結果**:
- 創建 ~50 runs
- 執行 ~250 seat calls
- 成功率 ≥95%
- 無 provider 漂移
- 無 event 丟失

## 監控命令

```bash
# 實時日誌
tail -f /tmp/endurance_live.log

# 實時指標
tail -f /tmp/endurance_metrics.jsonl | jq .

# DragonCore 事件
dragoncore events --run-id endurance-<id>

# 系統資源
watch -n 10 'ps aux | grep dragoncore | wc -l; df -h .; free -m'
```

## 終止條件

自動終止:
- 成功率 <70% 且 run >10
- 磁盤空間 <1GB
- 內存使用 >2GB

手動終止:
```bash
pkill -f endurance_runner.py
```

---

**建議**: 先執行選項 A (15分鐘預測試)，驗證通過後再執行選項 B。
