use agent_memgc::mcp::McpServer;

#[test]
fn test_mcp_tool_catalog() {
    let tools = McpServer::get_tools();
    let names: Vec<String> = tools.iter().map(|t| t.name.clone()).collect();

    assert!(names.contains(&"lint_instructions".to_string()));
    assert!(names.contains(&"fix_instructions".to_string()));
    assert!(names.contains(&"lint_raw_prompt".to_string()));
}
