use git_branch_cleanup_cli::branch::BranchInfo;
use chrono::Utc;

#[test] 
fn test_branch_info_struct() {
    let branch = BranchInfo {
        name: "feature/auth".to_string(),
        is_current: false,
        is_merged: true,
        last_commit_date: Some(Utc::now()),
        author: "Jane Doe".to_string(),
        upstream: Some("origin/feature/auth".to_string()),
    };

    assert_eq!(branch.name, "feature/auth");
    assert!(!branch.is_current);
    assert!(branch.is_merged);
    assert_eq!(branch.author, "Jane Doe");
}
