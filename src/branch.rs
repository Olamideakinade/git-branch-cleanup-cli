use chrono::{DateTime, Utc};
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub struct BranchInfo {
    pub name: String,
    pub is_current: bool,
    pub is_merged: bool,
    pub last_commit_date: Option<DateTime<Utc>>,
    pub author: String,
    pub upstream: Option<String>,
}

pub struct GitRepository;

impl GitRepository {
    pub fn list_local_branches(base_branch: &str) -> Result<Vec<BranchInfo>, String> {
        let output = Command::new("git")
            .args([
                "for-each-ref",
                "--format=%(HEAD)|%(refname:short)|%(upstream:short)|%(authordate:iso8601)|%(authorname)",
                "refs/heads/",
            ])
            .output();

        let output = match output {
            Ok(o) => o,
            Err(e) => return Err(format!("Failed to execute git command: {}", e)),
        };

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Git command failed: {}", stderr.trim()));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut branches = Vec::new();

        let merged_branches = Self::get_merged_branch_names(base_branch)?;

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() < 5 {
                continue;
            }

            let is_current = parts[0] == "*";
            let name = parts[1].to_string();
            let upstream = if parts[2].is_empty() {
                None
            } else {
                Some(parts[2].to_string())
            };
            
            let date_str = parts[3];
            let last_commit_date = DateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S %z")
                .map(|dt| dt.with_timezone(&Utc))
                .ok();
            
            let author = parts[4].to_string();
            let is_merged = merged_branches.contains(&name);

            branches.push(BranchInfo {
                name,
                is_current,
                is_merged,
                last_commit_date,
                author,
                upstream,
            });
        }

        Ok(branches)
    }

    fn get_merged_branch_names(base_branch: &str) -> Result<Vec<String>, String> {
        let output = Command::new("git")
            .args(["branch", "--merged", base_branch])
            .output();

        let output = match output {
            Ok(o) => o,
            Err(e) => return Err(format!("Failed to list merged branches: {}", e)),
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut merged = Vec::new();

        for line in stdout.lines() {
            let cleaned = line.trim().trim_start_matches('*').trim().to_string();
            if !cleaned.is_empty() {
                merged.push(cleaned);
            }
        }

        Ok(merged)
    }

    pub fn delete_branch(branch: &str, force: bool) -> Result<(), String> {
        let flag = if force { "-D" } else { "-d" };
        let output = Command::new("git")
            .args(["branch", flag, branch])
            .output();

        match output {
            Ok(o) if o.status.success() => Ok(()),
            Ok(o) => {
                let err = String::from_utf8_lossy(&o.stderr);
                Err(format!("Failed to delete {}: {}", branch, err.trim()))
            }
            Err(e) => Err(format!("Failed to execute delete command: {}", e)),
        }
    }
}
