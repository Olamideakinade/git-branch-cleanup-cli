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
        name: "bugfix/login-crash".to_string(),
        is_current: false,
        is_merged: false,
        last_commit_date: Some(Utc::now()),
        author: "John Smith".to_string(),
        upstream: None,
    };

    let serialized = serde_json::to_string(&branch).unwrap();
    assert!(serialized.contains("bugfix/login-crash"));
    assert!(serialized.contains("John Smith"));

    let deserialized: BranchInfo = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.name, branch.name);
    assert_eq!(deserialized.author, branch.author);
}
