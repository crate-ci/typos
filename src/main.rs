use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "typos-lsp".to_string(),
                version: Some(VERSION.into()),
            }),
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}

#[tokio::test]
async fn test_initialize() {
    let (service, _) = LspService::new(|client| Backend { client });

    let params = InitializeParams::default();

    let result = service.inner().initialize(params).await.unwrap();

    let server_info = result.server_info.unwrap();

    assert_eq!(server_info.name, "typos-lsp".to_string());
    assert_eq!(server_info.version, Some(env!("CARGO_PKG_VERSION").into()));
}
