#![allow(unused)]

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod config;
mod events;
mod governance;
mod ledger;
mod models;
mod persistence;
mod runtime;
mod tmux;
mod worktree;

use config::Config;
use governance::Seat;
use runtime::RuntimeBuilder;

/// DragonCore Runtime CLI
#[derive(Parser, Debug)]
#[command(name = "dragoncore")]
#[command(about = "DragonCore Runtime - Governance-first multi-agent AI operating system")]
#[command(version)]
struct Cli {
    /// Configuration file path
    #[arg(short, long)]
    config: Option<String>,
    
    /// Log level
    #[arg(short, long, default_value = "info")]
    log_level: String,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize a new DragonCore configuration
    Init {
        /// Output directory
        #[arg(short, long, default_value = ".")]
        output: String,
    },
    
    /// Start a new governance run
    Run {
        /// Run ID (auto-generated if not provided)
        #[arg(short, long)]
        run_id: Option<String>,
        
        /// Input type
        #[arg(short, long, default_value = "general")]
        input_type: String,
        
        /// Initial task description
        #[arg(short, long)]
        task: Option<String>,
    },
    
    /// Execute a specific seat's role
    Execute {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
        
        /// Seat to execute
        #[arg(short, long)]
        seat: String,
        
        /// Task description
        #[arg(short, long)]
        task: String,
    },
    
    /// Exercise veto
    Veto {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
        
        /// Seat exercising veto
        #[arg(short, long)]
        seat: String,
        
        /// Veto reason
        #[arg(short, long)]
        reason: String,
    },
    
    /// Execute final gate (Tianshu only)
    FinalGate {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
        
        /// Approve or reject
        #[arg(short, long)]
        approve: bool,
    },
    
    /// Archive a run
    Archive {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
        
        /// Seat archiving (default: Yaoguang)
        #[arg(short, long, default_value = "Yaoguang")]
        seat: String,
    },
    
    /// Terminate a run
    Terminate {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
        
        /// Seat terminating (default: Fengdudadi)
        #[arg(short, long, default_value = "Fengdudadi")]
        seat: String,
        
        /// Termination reason
        #[arg(short, long)]
        reason: String,
    },
    
    /// Show run status
    Status {
        /// Run ID (omit for all active runs)
        #[arg(short, long)]
        run_id: Option<String>,
    },
    
    /// Show stability metrics
    Metrics,
    
    /// Show DIBL events for a run
    Events {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
        
        /// Output format (json, table)
        #[arg(short, long, default_value = "table")]
        format: String,
    },
    
    /// Replay DIBL events for a run
    Replay {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
    },
    
    /// Attach to tmux session
    Attach {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
    },
    
    /// List all 19 seats
    Seats,
    
    /// Clean up all resources
    Cleanup,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    let log_level = match cli.log_level.as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };
    
    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set tracing subscriber")?;
    
    // Load configuration
    let config = if let Some(config_path) = cli.config {
        Config::load(&config_path)?
    } else {
        Config::init_default()?
    };
    
    match cli.command {
        Commands::Init { output } => {
            info!("Initializing DragonCore configuration");
            let config_path = std::path::Path::new(&output).join("dragoncore.toml");
            config.save(&config_path)?;
            println!("Configuration saved to: {:?}", config_path);
            Ok(())
        }
        
        Commands::Run { run_id, input_type, task } => {
            let runtime = RuntimeBuilder::new().with_config(config).build().await?;
            let run_id = run_id.unwrap_or_else(|| generate_run_id());
            let task_str = task.clone().unwrap_or_default();
            
            let context = runtime.init_run(&run_id, input_type, task_str).await?;
            println!("Started governance run: {}", run_id);
            println!("Worktree: {:?}", context.worktree_path);
            
            if let Some(task) = task {
                // Execute with Tianquan (CSO) as initial orchestrator
                let response = runtime.execute_seat(&run_id, Seat::Tianquan, &task).await?;
                println!("\nTianquan (CSO) Response:\n{}", response);
            }
            
            Ok(())
        }
        
        Commands::Execute { run_id, seat, task } => {
            let runtime = RuntimeBuilder::new().with_config(config).build().await?;
            let seat = parse_seat(&seat)?;
            let response = runtime.execute_seat(&run_id, seat, &task).await?;
            println!("{} Response:\n{}", seat.chinese_name(), response);
            Ok(())
        }
        
        Commands::Veto { run_id, seat, reason } => {
            let runtime = RuntimeBuilder::new().with_config(config).build().await?;
            let seat = parse_seat(&seat)?;
            runtime.exercise_veto(&run_id, seat, &reason).await?;
            println!("Veto exercised by {} on run {}", seat.chinese_name(), run_id);
            Ok(())
        }
        
        Commands::FinalGate { run_id, approve } => {
            let runtime = RuntimeBuilder::new().with_config(config).build().await?;
            runtime.final_gate(&run_id, approve).await?;
            println!("Final gate executed: {}", if approve { "APPROVED" } else { "REJECTED" });
            Ok(())
        }
        
        Commands::Archive { run_id, seat } => {
            let runtime = RuntimeBuilder::new().with_config(config).build().await?;
            let seat = parse_seat(&seat)?;
            runtime.archive_run(&run_id, seat).await?;
            println!("Run {} archived by {}", run_id, seat.chinese_name());
            Ok(())
        }
        
        Commands::Terminate { run_id, seat, reason } => {
            let runtime = RuntimeBuilder::new().with_config(config).build().await?;
            let seat = parse_seat(&seat)?;
            runtime.terminate_run(&run_id, seat, &reason).await?;
            println!("Run {} terminated by {}: {}", run_id, seat.chinese_name(), reason);
            Ok(())
        }
        
        Commands::Status { run_id } => {
            let runtime = RuntimeBuilder::new().with_config(config).build().await?;
            
            if let Some(run_id) = run_id {
                match runtime.get_run_status(&run_id).await {
                    Ok(Some(state)) => println!("Run {}: {:?}", run_id, state),
                    Ok(None) => println!("Run {} not found", run_id),
                    Err(e) => println!("Error getting run status: {}", e),
                }
            } else {
                // Show all runs from persistence, not just active in memory
                let all_runs = runtime.list_all_runs().await;
                let active = runtime.list_active_runs().await;
                
                println!("Total runs in storage: {}", all_runs.len());
                println!("Active runs (in memory): {}", active.len());
                println!();
                
                if !all_runs.is_empty() {
                    println!("All runs:");
                    for run_id in &all_runs {
                        match runtime.get_run_status(&run_id).await {
                            Ok(Some(state)) => {
                                let active_marker = if active.contains(run_id) { " [active]" } else { "" };
                                println!("  {}: {:?}{}", run_id, state, active_marker);
                            }
                            Ok(None) => println!("  {}: not found", run_id),
                            Err(e) => println!("  {}: error - {}", run_id, e),
                        }
                    }
                }
            }
            
            Ok(())
        }
        
        Commands::Metrics => {
            let runtime = RuntimeBuilder::new().with_config(config).build().await?;
            let metrics = runtime.get_stability_metrics().await?;
            
            println!("DragonCore Stability Metrics");
            println!("============================");
            println!("Total runs: {}", metrics.total_runs);
            println!("Clean runs: {}", metrics.clean_runs);
            println!("Authority violations: {}", metrics.authority_violations);
            println!("Fake closures: {}", metrics.fake_closures);
            println!("Rollbacks: {}", metrics.rollbacks);
            println!("Terminations: {}", metrics.terminations);
            
            Ok(())
        }
        
        Commands::Events { run_id, format } => {
            let runtime = RuntimeBuilder::new().with_config(config).build().await?;
            
            match runtime.load_run_events(&run_id) {
                Ok(events) => {
                    if events.is_empty() {
                        println!("No events found for run {}", run_id);
                        return Ok(());
                    }
                    
                    match format.as_str() {
                        "json" => {
                            for event in events {
                                println!("{}", serde_json::to_string(&event).unwrap());
                            }
                        }
                        _ => {
                            println!("Events for run {}:", run_id);
                            println!("{:<20} {:<12} {:<15} {:<10} {}", 
                                "Timestamp", "Channel", "Type", "Actor", "Summary");
                            println!("{}", "-".repeat(80));
                            for event in events {
                                println!("{:<20} {:<12} {:<15} {:<10} {}",
                                    event.created_at.format("%Y-%m-%d %H:%M:%S"),
                                    format!("{:?}", event.channel),
                                    format!("{:?}", event.event_type),
                                    event.actor.chars().take(10).collect::<String>(),
                                    event.summary.chars().take(40).collect::<String>()
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Error loading events: {}", e);
                }
            }
            
            Ok(())
        }
        
        Commands::Replay { run_id } => {
            let runtime = RuntimeBuilder::new().with_config(config).build().await?;
            
            println!("Replaying events for run {}...", run_id);
            
            match runtime.replay_run_events(&run_id) {
                Ok(events) => {
                    println!("Replayed {} events", events.len());
                    
                    // Show operator projection
                    match runtime.get_operator_projection(&run_id) {
                        Ok(proj) => {
                            println!("\nRun Projection:");
                            println!("  Current Phase: {}", proj.current_phase);
                            println!("  Current Seat: {:?}", proj.current_seat);
                            println!("  Veto Count: {}", proj.veto_count);
                            println!("  Terminated: {}", proj.terminate_flag);
                            println!("  Final Outcome: {:?}", proj.final_outcome);
                        }
                        Err(e) => {
                            println!("Error getting projection: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Error replaying events: {}", e);
                }
            }
            
            Ok(())
        }
        
        Commands::Attach { run_id } => {
            let session_name = format!("dragoncore_{}", run_id);
            println!("Attaching to tmux session: {}", session_name);
            println!("(Press Ctrl+B then D to detach)");
            tmux::attach_session(&session_name)?;
            Ok(())
        }
        
        Commands::Seats => {
            println!("DragonCore 19 Governance Seats");
            println!("==============================\n");
            
            use governance::{all_seats, Layer};
            
            println!("北斗七星 (Seven Northern Stars):");
            for seat in all_seats() {
                if seat.layer() == Layer::SevenStars {
                    println!("  {} - {} - {}", 
                        seat.chinese_name(),
                        format!("{:?}", seat),
                        seat.role()
                    );
                }
            }
            
            println!("\n四象 (Four Symbols):");
            for seat in all_seats() {
                if seat.layer() == Layer::FourSymbols {
                    println!("  {} - {} - {}", 
                        seat.chinese_name(),
                        format!("{:?}", seat),
                        seat.role()
                    );
                }
            }
            
            println!("\n八仙护法 (Eight Guardian Immortals):");
            for seat in all_seats() {
                if seat.layer() == Layer::EightImmortals {
                    println!("  {} - {} - {}", 
                        seat.chinese_name(),
                        format!("{:?}", seat),
                        seat.role()
                    );
                }
            }
            
            Ok(())
        }
        
        Commands::Cleanup => {
            info!("Cleaning up DragonCore resources");
            let runtime = RuntimeBuilder::new().with_config(config).build().await?;
            runtime.shutdown().await?;
            println!("Cleanup complete");
            Ok(())
        }
    }
}

fn generate_run_id() -> String {
    use chrono::Utc;
    use uuid::Uuid;
    
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let short_uuid = Uuid::new_v4().to_string().split('-').next().unwrap().to_string();
    format!("RUN-{}-{}", timestamp, &short_uuid[..8])
}

fn parse_seat(s: &str) -> Result<Seat> {
    match s {
        "Tianshu" | "tianshu" | "天枢" => Ok(Seat::Tianshu),
        "Tianxuan" | "tianxuan" | "天璇" => Ok(Seat::Tianxuan),
        "Tianji" | "tianji" | "天玑" => Ok(Seat::Tianji),
        "Tianquan" | "tianquan" | "天权" => Ok(Seat::Tianquan),
        "Yuheng" | "yuheng" | "玉衡" => Ok(Seat::Yuheng),
        "Kaiyang" | "kaiyang" | "开阳" => Ok(Seat::Kaiyang),
        "Yaoguang" | "yaoguang" | "瑶光" => Ok(Seat::Yaoguang),
        "Qinglong" | "qinglong" | "青龙" => Ok(Seat::Qinglong),
        "Baihu" | "baihu" | "白虎" => Ok(Seat::Baihu),
        "Zhuque" | "zhuque" | "朱雀" => Ok(Seat::Zhuque),
        "Xuanwu" | "xuanwu" | "玄武" => Ok(Seat::Xuanwu),
        "Yangjian" | "yangjian" | "杨戬" => Ok(Seat::Yangjian),
        "Baozheng" | "baozheng" | "包拯" => Ok(Seat::Baozheng),
        "Zhongkui" | "zhongkui" | "钟馗" => Ok(Seat::Zhongkui),
        "Luban" | "luban" | "鲁班" => Ok(Seat::Luban),
        "Zhugeliang" | "zhugeliang" | "诸葛亮" => Ok(Seat::Zhugeliang),
        "Nezha" | "nezha" | "哪吒" => Ok(Seat::Nezha),
        "Xiwangmu" | "xiwangmu" | "西王母" => Ok(Seat::Xiwangmu),
        "Fengdudadi" | "fengdudadi" | "丰都大帝" => Ok(Seat::Fengdudadi),
        _ => anyhow::bail!("Unknown seat: {}", s),
    }
}
