#!/usr/bin/env python3
"""
DragonCore Phase 3: 4-Hour Endurance Test Runner

Usage:
    python3 endurance_runner.py --duration 240 --interval 300

Args:
    --duration: Test duration in minutes (default: 240 = 4 hours)
    --interval: Seconds between new runs (default: 300 = 5 minutes)
"""

import argparse
import json
import subprocess
import sys
import time
import threading
import queue
from datetime import datetime, timedelta
from pathlib import Path
from typing import Dict, List, Optional
import signal

# Configuration
RUNTIME_BIN = "./target/release/dragoncore-runtime"
CONFIG_FILE = "./dragoncore.toml"
METRICS_FILE = "/tmp/endurance_metrics.jsonl"
LOG_FILE = "/tmp/endurance_live.log"

# Test task templates
TASKS = [
    {
        "type": "code_review",
        "weight": 60,
        "seats": ["Tianquan", "Kaiyang", "Yuheng"],
        "task": "Review src/events/mod.rs for error handling patterns. Suggest improvements."
    },
    {
        "type": "architecture",
        "weight": 25,
        "seats": ["Tianji", "Tianxuan", "Zhugeliang", "Luban"],
        "task": "Design a caching layer for DIBL event replay. Consider consistency and performance."
    },
    {
        "type": "conflict",
        "weight": 10,
        "seats": ["Tianquan", "Nezha", "Yuheng", "Xuanwu", "Baihu", "Tianshu"],
        "task": "Urgent: Deploy feature with tight deadline but incomplete testing. Risk assessment required."
    },
    {
        "type": "emergency",
        "weight": 5,
        "seats": ["Nezha", "Zhongkui"],
        "task": "Quick fix: Update timeout config from 60s to 120s. Minimal review needed."
    }
]

class EnduranceRunner:
    def __init__(self, duration_minutes: int, interval_seconds: int):
        self.duration = timedelta(minutes=duration_minutes)
        self.interval = interval_seconds
        self.start_time = None
        self.end_time = None
        self.run_count = 0
        self.success_count = 0
        self.failure_count = 0
        self.seat_count = 0
        self.metrics: List[Dict] = []
        self.active_runs: set = set()
        self.lock = threading.Lock()
        self.stop_event = threading.Event()
        
    def log(self, message: str, level: str = "INFO"):
        """Log message to console and file"""
        timestamp = datetime.now().isoformat()
        log_line = f"[{timestamp}] [{level}] {message}"
        print(log_line)
        with open(LOG_FILE, "a") as f:
            f.write(log_line + "\n")
            
    def record_metric(self, metric: Dict):
        """Record metric to JSONL file"""
        metric["timestamp"] = datetime.now().isoformat()
        with open(METRICS_FILE, "a") as f:
            f.write(json.dumps(metric) + "\n")
        with self.lock:
            self.metrics.append(metric)
            
    def execute_dragoncore(self, args: List[str]) -> tuple[bool, str]:
        """Execute dragoncore-runtime command"""
        cmd = [RUNTIME_BIN, "-c", CONFIG_FILE] + args
        try:
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=180
            )
            return result.returncode == 0, result.stdout + result.stderr
        except subprocess.TimeoutExpired:
            return False, "Timeout"
        except Exception as e:
            return False, str(e)
            
    def create_run(self) -> Optional[str]:
        """Create a new governance run"""
        run_id = f"endurance-{int(time.time())}"
        task_template = self.select_task()
        
        success, output = self.execute_dragoncore([
            "run",
            "-r", run_id,
            "-i", task_template["type"],
            "-t", task_template["task"]
        ])
        
        if success:
            self.log(f"Created run: {run_id} (type: {task_template['type']})")
            return run_id
        else:
            self.log(f"Failed to create run: {output}", "ERROR")
            return None
            
    def select_task(self) -> Dict:
        """Select task based on weight distribution"""
        import random
        total_weight = sum(t["weight"] for t in TASKS)
        r = random.uniform(0, total_weight)
        cumulative = 0
        for task in TASKS:
            cumulative += task["weight"]
            if r <= cumulative:
                return task
        return TASKS[0]
        
    def execute_seats(self, run_id: str, seats: List[str]):
        """Execute multiple seats for a run"""
        task_template = self.select_task()
        
        for seat in seats:
            if self.stop_event.is_set():
                break
                
            start_time = time.time()
            success, output = self.execute_dragoncore([
                "execute",
                "-r", run_id,
                "-s", seat,
                "-t", task_template["task"]
            ])
            latency = time.time() - start_time
            
            with self.lock:
                self.seat_count += 1
                
            self.record_metric({
                "type": "seat_execution",
                "run_id": run_id,
                "seat": seat,
                "success": success,
                "latency_seconds": latency,
                "output_chars": len(output)
            })
            
            if success:
                self.log(f"  Seat {seat} completed ({latency:.1f}s)")
            else:
                self.log(f"  Seat {seat} failed: {output[:200]}", "WARN")
                
    def run_governance_cycle(self, run_id: str):
        """Complete governance cycle for a run"""
        task = self.select_task()
        seats = task["seats"]
        
        # Execute seats
        self.execute_seats(run_id, seats)
        
        # Randomly add veto for conflict tasks
        if task["type"] == "conflict" and len(seats) >= 4:
            veto_seat = "Yuheng"
            success, _ = self.execute_dragoncore([
                "veto",
                "-r", run_id,
                "-s", veto_seat,
                "--reason", "Quality gate: Insufficient validation time for production deployment"
            ])
            if success:
                self.log(f"  Veto exercised by {veto_seat}")
                
        # Final gate
        success, _ = self.execute_dragoncore([
            "final-gate",
            "-r", run_id,
            "--approve"
        ])
        if success:
            self.log(f"  Final gate: APPROVED")
            
        # Archive
        success, _ = self.execute_dragoncore([
            "archive",
            "-r", run_id,
            "-s", "Yaoguang"
        ])
        if success:
            self.log(f"  Archived by Yaoguang")
            
    def worker_thread(self, run_queue: queue.Queue):
        """Worker thread to process runs"""
        while not self.stop_event.is_set():
            try:
                run_id = run_queue.get(timeout=5)
                if run_id is None:
                    break
                    
                with self.lock:
                    self.active_runs.add(run_id)
                    
                self.run_governance_cycle(run_id)
                
                with self.lock:
                    self.active_runs.discard(run_id)
                    self.success_count += 1
                    
            except queue.Empty:
                continue
            except Exception as e:
                self.log(f"Worker error: {e}", "ERROR")
                with self.lock:
                    self.failure_count += 1
                    
    def monitor_thread(self):
        """Monitor and report status periodically"""
        while not self.stop_event.is_set():
            time.sleep(60)  # Report every minute
            
            if self.stop_event.is_set():
                break
                
            elapsed = datetime.now() - self.start_time
            remaining = self.duration - elapsed
            
            with self.lock:
                status = {
                    "elapsed_minutes": elapsed.total_seconds() / 60,
                    "remaining_minutes": remaining.total_seconds() / 60,
                    "total_runs": self.run_count,
                    "successful_runs": self.success_count,
                    "failed_runs": self.failure_count,
                    "total_seat_executions": self.seat_count,
                    "active_runs": len(self.active_runs),
                    "success_rate": self.success_count / max(self.run_count, 1) * 100
                }
                
            self.log(f"STATUS: {status['elapsed_minutes']:.0f}min elapsed, "
                    f"{status['success_rate']:.1f}% success, "
                    f"{status['total_seat_executions']} seats executed")
            
            # Check termination conditions
            if status["success_rate"] < 70 and self.run_count > 10:
                self.log("FAILURE RATE TOO HIGH - TERMINATING", "CRITICAL")
                self.stop_event.set()
                break
                
    def run(self):
        """Main endurance test loop"""
        self.start_time = datetime.now()
        self.end_time = self.start_time + self.duration
        
        self.log(f"=== DragonCore Phase 3 Endurance Test ===")
        self.log(f"Start time: {self.start_time.isoformat()}")
        self.log(f"End time: {self.end_time.isoformat()}")
        self.log(f"Duration: {self.duration.total_seconds() / 3600:.1f} hours")
        self.log(f"Interval: {self.interval} seconds between runs")
        
        # Start monitor thread
        monitor = threading.Thread(target=self.monitor_thread)
        monitor.daemon = True
        monitor.start()
        
        # Create worker threads
        run_queue = queue.Queue()
        workers = []
        for _ in range(3):  # 3 concurrent workers
            t = threading.Thread(target=self.worker_thread, args=(run_queue,))
            t.daemon = True
            t.start()
            workers.append(t)
            
        # Main loop
        try:
            while datetime.now() < self.end_time and not self.stop_event.is_set():
                run_id = self.create_run()
                if run_id:
                    with self.lock:
                        self.run_count += 1
                    run_queue.put(run_id)
                    
                # Wait for next interval
                for _ in range(self.interval):
                    if self.stop_event.is_set():
                        break
                    time.sleep(1)
                    
        except KeyboardInterrupt:
            self.log("Interrupted by user", "WARN")
            self.stop_event.set()
            
        # Cleanup
        self.log("Shutting down workers...")
        for _ in workers:
            run_queue.put(None)
        for t in workers:
            t.join(timeout=30)
            
        # Final report
        self.generate_report()
        
    def generate_report(self):
        """Generate final endurance test report"""
        end_time = datetime.now()
        duration = end_time - self.start_time
        
        report = {
            "test_name": "Phase 3 Endurance Test",
            "start_time": self.start_time.isoformat(),
            "end_time": end_time.isoformat(),
            "duration_minutes": duration.total_seconds() / 60,
            "total_runs": self.run_count,
            "successful_runs": self.success_count,
            "failed_runs": self.failure_count,
            "success_rate_percent": self.success_count / max(self.run_count, 1) * 100,
            "total_seat_executions": self.seat_count,
            "status": "PASSED" if self.success_count / max(self.run_count, 1) >= 0.95 else "FAILED"
        }
        
        self.log("=== FINAL REPORT ===")
        self.log(json.dumps(report, indent=2))
        
        with open("PHASE3_REPORT.json", "w") as f:
            json.dump(report, f, indent=2)
            
        return report["status"] == "PASSED"

def main():
    parser = argparse.ArgumentParser(description="DragonCore Phase 3 Endurance Test")
    parser.add_argument("--duration", type=int, default=240,
                       help="Test duration in minutes (default: 240 = 4 hours)")
    parser.add_argument("--interval", type=int, default=300,
                       help="Seconds between runs (default: 300 = 5 minutes)")
    args = parser.parse_args()
    
    runner = EnduranceRunner(args.duration, args.interval)
    
    # Handle signals
    def signal_handler(sig, frame):
        print("\nReceived signal, shutting down...")
        runner.stop_event.set()
        sys.exit(0)
        
    signal.signal(signal.SIGINT, signal_handler)
    signal.signal(signal.SIGTERM, signal_handler)
    
    # Run test
    success = runner.run()
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()
