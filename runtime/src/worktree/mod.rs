use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Git worktree manager
pub struct WorktreeManager {
    base_path: PathBuf,
    main_repo: PathBuf,
}

impl WorktreeManager {
    /// Create a new worktree manager
    pub fn new(base_path: impl AsRef<Path>, main_repo: impl AsRef<Path>) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
            main_repo: main_repo.as_ref().to_path_buf(),
        }
    }
    
    /// Check if git is available
    pub fn check_git(&self) -> Result<()> {
        let output = Command::new("which")
            .arg("git")
            .output()
            .context("Failed to check git availability")?;
        
        if !output.status.success() {
            anyhow::bail!("git is not installed or not in PATH");
        }
        
        Ok(())
    }
    
    /// Create a new worktree for a run
    #[allow(dead_code)]
    pub fn create_worktree(&self, run_id: impl AsRef<str>, branch: impl AsRef<str>) -> Result<PathBuf> {
        let run_id = run_id.as_ref();
        let branch = branch.as_ref();
        let worktree_path = self.base_path.join(run_id);
        
        // Ensure base path exists
        std::fs::create_dir_all(&self.base_path)
            .with_context(|| format!("Failed to create worktree base directory: {:?}", self.base_path))?;
        
        // Check if worktree already exists
        if worktree_path.exists() {
            tracing::warn!("Worktree {} already exists, removing", run_id);
            self.remove_worktree(run_id)?;
        }
        
        // Create worktree (use -f to force override any stale registrations)
        let output = Command::new("git")
            .current_dir(&self.main_repo)
            .args(&[
                "worktree",
                "add",
                "-f",
                worktree_path.to_str().unwrap(),
                branch,
            ])
            .output()
            .with_context(|| format!("Failed to create worktree for {}", run_id))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to create worktree: {}", stderr);
        }
        
        tracing::info!("Created worktree for {} at {:?}", run_id, worktree_path);
        Ok(worktree_path)
    }
    
    /// Create a new worktree from current HEAD (for new runs)
    pub fn create_worktree_from_head(&self, run_id: impl AsRef<str>) -> Result<PathBuf> {
        let run_id = run_id.as_ref();
        
        // Check if we're in a git repository
        let git_check = Command::new("git")
            .current_dir(&self.main_repo)
            .args(&["rev-parse", "--git-dir"])
            .output();
        
        if git_check.is_err() || !git_check.unwrap().status.success() {
            anyhow::bail!(
                "Not a git repository. DragonCore requires:\n\
                 1. git init\n\
                 2. At least one commit (git add && git commit)\n\
                 Please initialize a git repository first."
            );
        }
        
        // Check if we have at least one commit
        let head_check = Command::new("git")
            .current_dir(&self.main_repo)
            .args(&["rev-parse", "HEAD"])
            .output()
            .context("Failed to check git HEAD")?;
        
        if !head_check.status.success() {
            anyhow::bail!(
                "No commits found. DragonCore requires at least one commit.\n\
                 Please run: git add . && git commit -m 'initial commit'"
            );
        }
        
        let worktree_path = self.base_path.join(run_id);
        
        // Ensure base path exists
        std::fs::create_dir_all(&self.base_path)
            .with_context(|| format!("Failed to create worktree base directory: {:?}", self.base_path))?;
        
        // Check if worktree already exists
        if worktree_path.exists() {
            tracing::warn!("Worktree {} already exists, removing", run_id);
            self.remove_worktree(run_id)?;
        }
        
        // Get current HEAD
        let head_output = Command::new("git")
            .current_dir(&self.main_repo)
            .args(&["rev-parse", "HEAD"])
            .output()
            .context("Failed to get current HEAD")?;
        
        if !head_output.status.success() {
            let stderr = String::from_utf8_lossy(&head_output.stderr);
            anyhow::bail!("Failed to get HEAD: {}", stderr);
        }
        
        let head = String::from_utf8_lossy(&head_output.stdout).trim().to_string();
        
        // Create worktree from HEAD (use -f to force override any stale registrations)
        let output = Command::new("git")
            .current_dir(&self.main_repo)
            .args(&[
                "worktree",
                "add",
                "-f",
                "--detach",
                worktree_path.to_str().unwrap(),
                &head,
            ])
            .output()
            .with_context(|| format!("Failed to create worktree for {}", run_id))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to create worktree: {}", stderr);
        }
        
        tracing::info!("Created worktree for {} at {:?} from HEAD", run_id, worktree_path);
        Ok(worktree_path)
    }
    
    /// Remove a worktree
    pub fn remove_worktree(&self, run_id: impl AsRef<str>) -> Result<()> {
        let run_id = run_id.as_ref();
        let worktree_path = self.base_path.join(run_id);
        
        if !worktree_path.exists() {
            tracing::debug!("Worktree {} does not exist, nothing to remove", run_id);
            return Ok(());
        }
        
        // Remove worktree
        let output = Command::new("git")
            .current_dir(&self.main_repo)
            .args(&[
                "worktree",
                "remove",
                "--force",
                worktree_path.to_str().unwrap(),
            ])
            .output()
            .with_context(|| format!("Failed to remove worktree {}", run_id))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // Try to remove manually if git worktree remove fails
            tracing::warn!("git worktree remove failed: {}, trying manual removal", stderr);
            std::fs::remove_dir_all(&worktree_path)
                .with_context(|| format!("Failed to manually remove worktree at {:?}", worktree_path))?;
        }
        
        tracing::info!("Removed worktree for {}", run_id);
        Ok(())
    }
    
    /// List all worktrees
    #[allow(dead_code)]
    pub fn list_worktrees(&self) -> Result<Vec<WorktreeInfo>> {
        let output = Command::new("git")
            .current_dir(&self.main_repo)
            .args(&["worktree", "list", "--porcelain"])
            .output()
            .context("Failed to list worktrees")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to list worktrees: {}", stderr);
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut worktrees = Vec::new();
        let mut current = None;
        
        for line in stdout.lines() {
            if line.starts_with("worktree ") {
                if let Some(wt) = current.take() {
                    worktrees.push(wt);
                }
                let path = line.strip_prefix("worktree ").unwrap().to_string();
                current = Some(WorktreeInfo {
                    path: PathBuf::from(path),
                    head: String::new(),
                    branch: None,
                });
            } else if line.starts_with("HEAD ") {
                if let Some(ref mut wt) = current {
                    wt.head = line.strip_prefix("HEAD ").unwrap().to_string();
                }
            } else if line.starts_with("branch ") {
                if let Some(ref mut wt) = current {
                    wt.branch = Some(line.strip_prefix("branch ").unwrap().to_string());
                }
            }
        }
        
        if let Some(wt) = current {
            worktrees.push(wt);
        }
        
        // Filter to only worktrees in our base path
        let base_str = self.base_path.to_string_lossy();
        let filtered: Vec<WorktreeInfo> = worktrees
            .into_iter()
            .filter(|wt| wt.path.to_string_lossy().starts_with(&*base_str))
            .collect();
        
        Ok(filtered)
    }
    
    /// Prune stale worktrees
    #[allow(dead_code)]
    pub fn prune_worktrees(&self) -> Result<()> {
        let output = Command::new("git")
            .current_dir(&self.main_repo)
            .args(&["worktree", "prune"])
            .output()
            .context("Failed to prune worktrees")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to prune worktrees: {}", stderr);
        }
        
        tracing::info!("Pruned stale worktrees");
        Ok(())
    }
    
    /// Get the path for a run's worktree
    pub fn get_worktree_path(&self, run_id: impl AsRef<str>) -> PathBuf {
        self.base_path.join(run_id.as_ref())
    }
    
    /// Check if a worktree exists
    #[allow(dead_code)]
    pub fn worktree_exists(&self, run_id: impl AsRef<str>) -> bool {
        self.get_worktree_path(run_id).exists()
    }
    
    /// Run git command in a worktree
    pub fn run_in_worktree(&self, run_id: impl AsRef<str>, args: &[&str]) -> Result<String> {
        let worktree_path = self.get_worktree_path(run_id);
        
        let output = Command::new("git")
            .current_dir(&worktree_path)
            .args(args)
            .output()
            .with_context(|| format!("Failed to run git command in worktree {:?}", worktree_path))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git command failed: {}", stderr);
        }
        
        String::from_utf8(output.stdout)
            .with_context(|| "Failed to decode git output as UTF-8")
    }
    
    /// Get the current commit hash in a worktree
    pub fn get_commit_hash(&self, run_id: impl AsRef<str>) -> Result<String> {
        self.run_in_worktree(run_id, &["rev-parse", "HEAD"])
            .map(|s| s.trim().to_string())
    }
    
    /// Archive a worktree (compress and move to archive)
    #[allow(dead_code)]
    pub fn archive_worktree(&self, run_id: impl AsRef<str>, archive_path: impl AsRef<Path>) -> Result<PathBuf> {
        let run_id = run_id.as_ref();
        let worktree_path = self.get_worktree_path(run_id);
        let archive_path = archive_path.as_ref();
        
        if !worktree_path.exists() {
            anyhow::bail!("Worktree {} does not exist", run_id);
        }
        
        std::fs::create_dir_all(archive_path)
            .with_context(|| format!("Failed to create archive directory: {:?}", archive_path))?;
        
        let archive_file = archive_path.join(format!("{}.tar.gz", run_id));
        
        // Create tarball
        let output = Command::new("tar")
            .current_dir(&self.base_path)
            .args(&[
                "-czf",
                archive_file.to_str().unwrap(),
                run_id,
            ])
            .output()
            .with_context(|| format!("Failed to archive worktree {}", run_id))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to archive worktree: {}", stderr);
        }
        
        // Remove the worktree (but not the archive)
        self.remove_worktree(run_id)?;
        
        tracing::info!("Archived worktree {} to {:?}", run_id, archive_file);
        Ok(archive_file)
    }
}

/// Information about a worktree
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WorktreeInfo {
    pub path: PathBuf,
    pub head: String,
    pub branch: Option<String>,
}

/// Execution context for a governance run
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RunContext {
    pub run_id: String,
    pub worktree_path: PathBuf,
    pub commit_hash: String,
}

impl RunContext {
    /// Create artifacts directory
    pub fn create_artifacts_dir(&self) -> Result<PathBuf> {
        let artifacts_path = self.worktree_path.join(".dragoncore").join("artifacts");
        std::fs::create_dir_all(&artifacts_path)
            .with_context(|| format!("Failed to create artifacts directory: {:?}", artifacts_path))?;
        Ok(artifacts_path)
    }
    
    /// Write artifact file
    pub fn write_artifact(&self, name: impl AsRef<str>, content: impl AsRef<[u8]>) -> Result<PathBuf> {
        let artifacts_path = self.create_artifacts_dir()?;
        let file_path = artifacts_path.join(name.as_ref());
        
        std::fs::write(&file_path, content)
            .with_context(|| format!("Failed to write artifact: {:?}", file_path))?;
        
        Ok(file_path)
    }
    
    /// Read artifact file
    #[allow(dead_code)]
    pub fn read_artifact(&self, name: impl AsRef<str>) -> Result<String> {
        let artifacts_path = self.worktree_path.join(".dragoncore").join("artifacts");
        let file_path = artifacts_path.join(name.as_ref());
        
        std::fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read artifact: {:?}", file_path))
    }
    
    /// Check if artifact exists
    #[allow(dead_code)]
    pub fn artifact_exists(&self, name: impl AsRef<str>) -> bool {
        let artifacts_path = self.worktree_path.join(".dragoncore").join("artifacts");
        artifacts_path.join(name.as_ref()).exists()
    }
}
