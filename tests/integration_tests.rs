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

#[test]
fn test_branch_serialization() {
    let branch = BranchInfo {
        name: "feature/test".to_string(),
        is_current: false,
        is_merged: false,
        last_commit_date: None,
        author: "Test User".to_string(),
        upstream: None,
    };

    let json = serde_json::to_string(&branch).unwrap();
    assert!(json.contains("feature/test"));
    assert!(json.contains("Test User"));
}
