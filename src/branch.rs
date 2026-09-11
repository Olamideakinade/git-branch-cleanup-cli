use chrono::{DateTime, Utc, TimeZone};
use std::process::Command;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchInfo {
    pub name: String,
    pub is_current: bool,
    pub is_merged: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_commit_date: Option<DateTime<Utc>>,
    pub author: String,
    pub upstream: Option<String>,
}

pub struct GitRepository;

impl GitRepository {
    pub fn list_local_branches(base_branch: &str) -> Result<Vec<BranchInfo>, String> {
        let output = Command::new("git")
            .args(["branch", "--format=%(HEAD)|%(refname:short)|%(upstream:short)|%(committerdate:iso8601)|%(authorname)"])
            .output()
            .map_err(|e| format!("Failed to execute git command: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Git error: {}", stderr.trim()));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut branches = Vec::new();

        let merged_output = Command::new("git")
            .args(["branch", "--merged", base_branch])
            .output()
            .map_err(|e| format!("Failed to check merged branches: {}", e))?;
        
        let merged_str = String::from_utf8_lossy(&merged_output.stdout);
        let merged_branches: Vec<&str> = merged_str
            .lines()
            .map(|l| l.trim().trim_start_matches('*').trim())
            .collect();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() < 5 {
                continue;
            }

            let is_current = parts[0] == "*";
            let name = parts[1].to_string();
            let upstream = if parts[2].is_empty() { None } else { Some(parts[2].to_string()) };
            let last_commit_date = DateTime::parse_from_str(parts[3], "%Y-%m-%d %H:%M:%S %z")
                .ok()
                .map(|dt| dt.with_timezone(&Utc));
            let author = parts[4].to_string();

            let is_merged = merged_branches.contains(&name.as_str()) || is_current;

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

    pub fn delete_branch(branch_name: &str, force: bool) -> Result<(), String> {
        let flag = if force { "-D" } else { "-d" };
        let output = Command::new("git")
            .args(["branch", flag, branch_name])
            .output()
            .map_err(|e| format!("Failed to execute branch deletion: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to delete branch '{}': {}", branch_name, stderr.trim()));
        }

        Ok(())
    }
}
