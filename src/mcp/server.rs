use crate::engine::{fix_instructions, lint_content, lint_file};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::io::Write;
use std::path::Path;
use tokio::io::{AsyncBufReadExt, BufReader};

#[derive(Debug, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    #[serde(default)]
    pub params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

pub struct McpServer;

impl Default for McpServer {
    fn default() -> Self {
        Self::new()
    }
}

impl McpServer {
    pub fn new() -> Self {
        Self
    }

    pub fn get_tools() -> Vec<McpTool> {
        vec![
            McpTool {
                name: "lint_instructions".into(),
                description: "Lint agent instruction files (AGENTS.md, CLAUDE.md, .cursorrules) for dead paths, contradictions, and cache invalidation.".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the instruction file"
                        },
                        "project_root": {
                            "type": "string",
                            "description": "Root workspace directory (defaults to current dir)"
                        }
                    },
                    "required": ["path"]
                }),
            },
            McpTool {
                name: "fix_instructions".into(),
                description: "Automatically clean, deduplicate, and optimize agent instruction files for prompt caching and token efficiency.".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the instruction file"
                        },
                        "project_root": {
                            "type": "string",
                            "description": "Root workspace directory (defaults to current dir)"
                        }
                    },
                    "required": ["path"]
                }),
            },
            McpTool {
                name: "lint_raw_prompt".into(),
                description: "Lint raw prompt text in memory before sending to LLM context to detect conflicting directives.".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "prompt": {
                            "type": "string",
                            "description": "Raw prompt or system prompt text"
                        }
                    },
                    "required": ["prompt"]
                }),
            },
        ]
    }

    pub async fn run_stdio(&self) -> anyhow::Result<()> {
        let stdin = tokio::io::stdin();
        let mut reader = BufReader::new(stdin).lines();

        eprintln!("[agent-memgc] Native Model Context Protocol (MCP) server listening on stdio.");

        while let Some(line) = reader.next_line().await? {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Ok(req) = serde_json::from_str::<JsonRpcRequest>(line) {
                let id = req.id.clone();
                let resp = self.handle_request(req).await;
                if let Some(r) = resp {
                    println!("{}", serde_json::to_string(&r)?);
                    std::io::stdout().flush()?;
                } else if let Some(id_val) = id {
                    let err_resp = JsonRpcResponse {
                        jsonrpc: "2.0".into(),
                        id: id_val,
                        result: None,
                        error: Some(json!({ "code": -32601, "message": "Method not found" })),
                    };
                    println!("{}", serde_json::to_string(&err_resp)?);
                    std::io::stdout().flush()?;
                }
            }
        }

        Ok(())
    }

    async fn handle_request(&self, req: JsonRpcRequest) -> Option<JsonRpcResponse> {
        let id = req.id?;

        match req.method.as_str() {
            "initialize" => Some(JsonRpcResponse {
                jsonrpc: "2.0".into(),
                id,
                result: Some(json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": { "listChanged": false }
                    },
                    "serverInfo": {
                        "name": "agent-memgc",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                })),
                error: None,
            }),

            "tools/list" => Some(JsonRpcResponse {
                jsonrpc: "2.0".into(),
                id,
                result: Some(json!({
                    "tools": Self::get_tools()
                })),
                error: None,
            }),

            "tools/call" => {
                let params = req.params.unwrap_or_default();
                let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let args = params.get("arguments").cloned().unwrap_or(json!({}));

                let res = self.execute_tool(tool_name, args);
                Some(JsonRpcResponse {
                    jsonrpc: "2.0".into(),
                    id,
                    result: Some(res),
                    error: None,
                })
            }

            _ => None,
        }
    }

    fn execute_tool(&self, name: &str, args: Value) -> Value {
        match name {
            "lint_instructions" => {
                let path_str = match args.get("path").and_then(|v| v.as_str()) {
                    Some(p) => p,
                    None => return json!({ "error": "Missing 'path' argument" }),
                };
                let root_str = args.get("project_root").and_then(|v| v.as_str()).unwrap_or(".");
                let path = Path::new(path_str);
                let root = Path::new(root_str);

                match lint_file(path, root) {
                    Ok(rep) => json!({ "report": rep }),
                    Err(e) => json!({ "error": e.to_string() }),
                }
            }

            "fix_instructions" => {
                let path_str = match args.get("path").and_then(|v| v.as_str()) {
                    Some(p) => p,
                    None => return json!({ "error": "Missing 'path' argument" }),
                };
                let root_str = args.get("project_root").and_then(|v| v.as_str()).unwrap_or(".");
                let path = Path::new(path_str);
                let root = Path::new(root_str);

                let content = match fs::read_to_string(path) {
                    Ok(c) => c,
                    Err(e) => return json!({ "error": format!("Failed to read file: {e}") }),
                };

                let (fixed, tokens) = fix_instructions(&content, root);
                if let Err(e) = fs::write(path, &fixed) {
                    return json!({ "error": format!("Failed to write file: {e}") });
                }

                json!({
                    "status": "success",
                    "tokens_saved": tokens,
                    "file": path_str
                })
            }

            "lint_raw_prompt" => {
                let prompt = match args.get("prompt").and_then(|v| v.as_str()) {
                    Some(p) => p,
                    None => return json!({ "error": "Missing 'prompt' argument" }),
                };
                let root = Path::new(".");
                match lint_content(prompt, "in-memory-prompt", root) {
                    Ok(rep) => json!({ "report": rep }),
                    Err(e) => json!({ "error": e.to_string() }),
                }
            }

            _ => json!({ "error": format!("Unknown tool '{name}'") }),
        }
    }
}
