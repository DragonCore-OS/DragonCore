#![allow(unused)]

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod arbitration;
mod config;
mod entity;
mod events;
mod governance;
mod ledger;
mod models;
mod persistence;
mod runtime;
mod tmux;
mod worktree;
mod meeting;

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
    Meeting {
        #[command(subcommand)]
        command: MeetingCommand,
    },
    
    /// Entity management
    Entity {
        #[command(subcommand)]
        command: EntityCommand,
    },
    
    // Clean up all resources
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
        
        Commands::Entity { command } => {
            use crate::entity::{EntityStatus, StateTransitionRequest};
            
            match command {
                EntityCommand::Create { name, role, department } => {
                    println!("Creating entity: {} with role: {} in department: {}", name, role, department);
                    // TODO: Implement entity creation
                    Ok(())
                }
                EntityCommand::Status { entity_id } => {
                    println!("Showing status for entity: {}", entity_id);
                    // TODO: Implement entity status query
                    Ok(())
                }
                EntityCommand::Transition { entity_id, to_status, reason, initiated_by, approved_by } => {
                    println!("Transitioning entity: {} to status: {}", entity_id, to_status);
                    println!("Reason: {}", reason);
                    println!("Initiated by: {}", initiated_by);
                    if let Some(approver) = approved_by {
                        println!("Approved by: {}", approver);
                    }
                    // TODO: Implement status transition
                    Ok(())
                }
                EntityCommand::List { filter } => {
                    println!("Listing entities with filter: {}", filter);
                    // TODO: Implement entity listing
                    Ok(())
                }
                EntityCommand::Promote { entity_id, to_rank, reason, initiated_by } => {
                    println!("Promoting entity: {} to rank: {}", entity_id, to_rank);
                    println!("Reason: {}", reason);
                    println!("Initiated by: {}", initiated_by);
                    // TODO: Implement promotion
                    Ok(())
                }
                EntityCommand::Demote { entity_id, reason, initiated_by } => {
                    println!("Demoting entity: {}", entity_id);
                    println!("Reason: {}", reason);
                    println!("Initiated by: {}", initiated_by);
                    // TODO: Implement demotion
                    Ok(())
                }
                EntityCommand::Suspend { entity_id, reason, initiated_by } => {
                    println!("Suspending entity: {}", entity_id);
                    println!("Reason: {}", reason);
                    println!("Initiated by: {}", initiated_by);
                    // TODO: Implement suspension
                    Ok(())
                }
                EntityCommand::Terminate { entity_id, reason, initiated_by, approved_by } => {
                    println!("Terminating entity: {}", entity_id);
                    println!("Reason: {}", reason);
                    println!("Initiated by: {}", initiated_by);
                    println!("Approved by: {}", approved_by);
                    // TODO: Implement termination
                    Ok(())
                }
                EntityCommand::Kpi { entity_id, period } => {
                    use crate::entity::kpi::{PeriodKPI, KPICalculator};
                    use uuid::Uuid;
                    
                    let period = period.unwrap_or_else(|| "2026-03".to_string());
                    
                    // 创建示例 KPI (真实实现需要从事件计算)
                    let entity_uuid = Uuid::parse_str(&entity_id).unwrap_or_else(|_| Uuid::new_v4());
                    let kpi = PeriodKPI::new(entity_uuid, &period);
                    
                    println!("{}", serde_json::to_string_pretty(&kpi).unwrap_or_default());
                    Ok(())
                }
                EntityCommand::Attribution { decision_id } => {
                    use crate::entity::attribution::{DecisionAttribution, DecisionType};
                    use uuid::Uuid;
                    
                    // 尝试解析 decision_id，否则创建示例
                    let decision_uuid = Uuid::parse_str(&decision_id).unwrap_or_else(|_| {
                        println!("Note: Using sample decision ID (invalid UUID provided)");
                        Uuid::new_v4()
                    });
                    
                    // 创建示例归因 (真实实现需要从存储查询)
                    let attr = DecisionAttribution::new(
                        DecisionType::Proposal,
                        Uuid::new_v4(),
                        Uuid::new_v4(),
                    ).with_supporters(vec![Uuid::new_v4()]);
                    
                    let weights = attr.calculate_responsibility();
                    
                    let result = serde_json::json!({
                        "decision_id": attr.decision_id,
                        "primary_owner": attr.primary_owner,
                        "approving_authority": attr.approving_authority,
                        "supporting": attr.supporting,
                        "responsibility_weights": weights,
                        "total_weight": weights.values().sum::<f32>()
                    });
                    
                    println!("{}", serde_json::to_string_pretty(&result).unwrap_or_default());
                    Ok(())
                }
            }
        }
        Commands::Meeting { command } => {
            match command {
                MeetingCommand::Open { run_id, topic, moderator, required_seats } => {
                    println!("Opening meeting for run: {} with topic: {}", run_id, topic);
                    println!("Moderator: {}", moderator);
                    if let Some(seats) = required_seats {
                        println!("Required seats: {}", seats);
                    }
                    Ok(())
                }
                MeetingCommand::Assemble { run_id } => {
                    println!("Assembling meeting for run: {}", run_id);
                    Ok(())
                }
                MeetingCommand::RollCall { run_id } => {
                    println!("Roll call for run: {}", run_id);
                    Ok(())
                }
                MeetingCommand::TopicLock { run_id, confirmation } => {
                    println!("Locking topic for run: {}", run_id);
                    println!("CEO confirmation: {}", confirmation);
                    Ok(())
                }
                MeetingCommand::RequestSpeak { run_id, seat, intent, confidence, urgency, reason } => {
                    println!("Speak request from {} for run: {}", seat, run_id);
                    println!("Intent: {}, Reason: {}", intent, reason);
                    Ok(())
                }
                MeetingCommand::ScheduleRound { run_id, round, speakers } => {
                    println!("Scheduling round {} for run: {}", round, run_id);
                    Ok(())
                }
                MeetingCommand::Speak { run_id, seat, content_file } => {
                    println!("Speaking turn for {} in run: {}", seat, run_id);
                    Ok(())
                }
                MeetingCommand::ForceSpeak { run_id, seat, reason } => {
                    println!("Forcing {} to speak in run: {}", seat, run_id);
                    println!("Reason: {}", reason);
                    Ok(())
                }
                MeetingCommand::UpdateStance { run_id, seat, position, confidence, supports, challenges } => {
                    println!("Updating stance for {} in run: {}", seat, run_id);
                    Ok(())
                }
                MeetingCommand::ChallengeWindow { run_id } => {
                    println!("Opening challenge window for run: {}", run_id);
                    Ok(())
                }
                MeetingCommand::DraftResolution { run_id, seat, summary, action } => {
                    println!("Drafting resolution for run: {} by {}", run_id, seat);
                    Ok(())
                }
                MeetingCommand::CommitAction { run_id, action } => {
                    println!("Committing action {} for run: {}", action, run_id);
                    Ok(())
                }
                MeetingCommand::Status { run_id } => {
                    println!("Meeting status for run: {}", run_id);
                    Ok(())
                }
            }
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

/// Meeting protocol subcommands
#[derive(Subcommand, Debug)]
enum MeetingCommand {
    /// Open a new meeting session
    Open {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
        
        /// Meeting topic
        #[arg(short, long)]
        topic: String,
        
        /// Moderator seat (default: Tianshu)
        #[arg(short, long, default_value = "Tianshu")]
        moderator: String,
        
        /// Required seats (comma-separated, default: all)
        #[arg(short, long)]
        required_seats: Option<String>,
    },
    
    /// Assemble and check seat presence
    Assemble {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
    },
    
    /// Roll call to confirm seat readiness
    RollCall {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
    },
    
    /// Lock the meeting topic
    TopicLock {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
        
        /// CEO confirmation statement
        #[arg(short, long)]
        confirmation: String,
    },
    
    /// Request to speak
    RequestSpeak {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
        
        /// Seat requesting to speak
        #[arg(short, long)]
        seat: String,
        
        /// Speak intent
        #[arg(short, long)]
        intent: String,
        
        /// Confidence level (0.0-1.0)
        #[arg(short, long)]
        confidence: Option<f32>,
        
        /// Urgency level (0.0-1.0)
        #[arg(short, long)]
        urgency: Option<f32>,
        
        /// Reason for speaking
        #[arg(short, long)]
        reason: String,
    },
    
    /// Schedule a discussion round
    ScheduleRound {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
        
        /// Round number
        #[arg(short, long)]
        round: u32,
        
        /// Speaker order (comma-separated, optional)
        #[arg(short, long)]
        speakers: Option<String>,
    },
    
    /// Execute a speaking turn
    Speak {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
        
        /// Seat speaking
        #[arg(short, long)]
        seat: String,
        
        /// Content file path
        #[arg(short, long)]
        content_file: String,
    },
    
    /// Force a seat to speak
    ForceSpeak {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
        
        /// Seat to force
        #[arg(short, long)]
        seat: String,
        
        /// Reason for forcing
        #[arg(short, long)]
        reason: String,
    },
    
    /// Update seat stance
    UpdateStance {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
        
        /// Seat updating stance
        #[arg(short, long)]
        seat: String,
        
        /// Position statement
        #[arg(short, long)]
        position: String,
        
        /// Confidence level
        #[arg(short, long)]
        confidence: Option<f32>,
        
        /// Seats supported (comma-separated)
        #[arg(short, long)]
        supports: Option<String>,
        
        /// Seats challenged (comma-separated)
        #[arg(short, long)]
        challenges: Option<String>,
    },
    
    /// Open challenge window
    ChallengeWindow {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
    },
    
    /// Draft resolution
    DraftResolution {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
        
        /// Seat drafting
        #[arg(short, long)]
        seat: String,
        
        /// Resolution summary
        #[arg(short, long)]
        summary: String,
        
        /// Recommended action
        #[arg(short, long)]
        action: String,
    },
    
    /// Commit governance action
    CommitAction {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
        
        /// Action to commit (raise-risk, exercise-veto, open-final-gate, terminate, archive)
        #[arg(short, long)]
        action: String,
    },
    
    /// Show meeting status
    Status {
        /// Run ID
        #[arg(short, long)]
        run_id: String,
    },
}

/// Entity management subcommands
#[derive(Subcommand, Debug)]
enum EntityCommand {
    /// Create a new AI entity
    Create {
        /// Entity name
        #[arg(short, long)]
        name: String,
        
        /// Seat role
        #[arg(short, long)]
        role: String,
        
        /// Department
        #[arg(short, long)]
        department: String,
    },
    
    /// Show entity status
    Status {
        /// Entity ID
        #[arg(short, long)]
        entity_id: String,
    },
    
    /// Transition entity status
    Transition {
        /// Entity ID
        #[arg(short, long)]
        entity_id: String,
        
        /// Target status
        #[arg(short, long)]
        to_status: String,
        
        /// Transition reason
        #[arg(short, long)]
        reason: String,
        
        /// Initiator
        #[arg(short, long)]
        initiated_by: String,
        
        /// Approver (required for certain transitions)
        #[arg(short, long)]
        approved_by: Option<String>,
    },
    
    /// List entities
    List {
        /// Filter (all, active, alive)
        #[arg(short, long, default_value = "all")]
        filter: String,
    },
    
    /// Promote entity
    Promote {
        /// Entity ID
        #[arg(short, long)]
        entity_id: String,
        
        /// Target rank
        #[arg(short, long)]
        to_rank: String,
        
        /// Reason
        #[arg(short, long)]
        reason: String,
        
        /// Initiator
        #[arg(short, long)]
        initiated_by: String,
    },
    
    /// Demote entity
    Demote {
        /// Entity ID
        #[arg(short, long)]
        entity_id: String,
        
        /// Reason
        #[arg(short, long)]
        reason: String,
        
        /// Initiator
        #[arg(short, long)]
        initiated_by: String,
    },
    
    /// Suspend entity
    Suspend {
        /// Entity ID
        #[arg(short, long)]
        entity_id: String,
        
        /// Reason
        #[arg(short, long)]
        reason: String,
        
        /// Initiator
        #[arg(short, long)]
        initiated_by: String,
    },
    
    /// Terminate entity
    Terminate {
        /// Entity ID
        #[arg(short, long)]
        entity_id: String,
        
        /// Reason
        #[arg(short, long)]
        reason: String,
        
        /// Initiator
        #[arg(short, long)]
        initiated_by: String,
        
        /// Approver (required for termination)
        #[arg(short, long)]
        approved_by: String,
    },
    
    /// Query entity KPI
    Kpi {
        /// Entity ID
        #[arg(short, long)]
        entity_id: String,
        
        /// Period (e.g., "2026-03")
        #[arg(short, long)]
        period: Option<String>,
    },
    
    /// Query decision attribution
    Attribution {
        /// Decision ID
        #[arg(short, long)]
        decision_id: String,
    },
}
