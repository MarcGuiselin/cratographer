use rmcp::{
    handler::server::router::tool::ToolRouter,
    model::{CallToolResult, Content, ErrorData as McpError, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router, ServerHandler, ServiceExt,
    transport::stdio,
};

/// Cratographer MCP Server
/// Provides tools for indexing and querying Rust code symbols
#[derive(Debug, Clone)]
struct CratographerServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl CratographerServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Find all occurrences of a symbol by name across the indexed codebase
    #[tool(description = "Find all occurrences of a Rust symbol (struct, enum, trait, function, method) by name")]
    async fn find_symbol(&self) -> Result<CallToolResult, McpError> {
        // No-op implementation for now
        Ok(CallToolResult::success(vec![Content::text(
            "find_symbol tool (not yet implemented)".to_string()
        )]))
    }

    /// List all symbols defined in a specific file
    #[tool(description = "Enumerate all Rust symbols defined in a specific file")]
    async fn enumerate_file(&self) -> Result<CallToolResult, McpError> {
        // No-op implementation for now
        Ok(CallToolResult::success(vec![Content::text(
            "enumerate_file tool (not yet implemented)".to_string()
        )]))
    }
}

#[tool_handler]
impl ServerHandler for CratographerServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "cratographer".to_string(),
                version: "0.1.0".to_string(),
                icons: None,
                title: None,
                website_url: None,
            },
            instructions: Some(
                "Cratographer: An MCP tool for indexing and analyzing Rust code. \
                Use find_symbol to locate symbol definitions and enumerate_file to list all symbols in a file."
                    .to_string(),
            ),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the server instance and start serving
    let service = CratographerServer::new().serve(stdio()).await?;

    // Wait for shutdown
    service.waiting().await?;

    Ok(())
}
