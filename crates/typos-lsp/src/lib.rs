use tower_lsp::lsp_types::*;
use tower_lsp::*;
use tower_lsp::{Client, LanguageServer};

mod check;

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..ServerCapabilities::default()
            },
            server_info: Some(ServerInfo {
                name: env!("CARGO_PKG_NAME").to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        tracing::info!("did_open: {:?}", params);
        self.report_diagnostics(params.text_document).await;
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }
}

impl Backend {
    async fn report_diagnostics(&self, params: TextDocumentItem) {
        let dict = typos_cli::dict::BuiltIn::new(Default::default());
        let tokenizer = typos::tokens::Tokenizer::new();
        let policy = typos_cli::policy::Policy::new()
            .dict(&dict)
            .tokenizer(&tokenizer);

        let diagnostics = check::check_text(&params.text, &policy);

        self.client
            .publish_diagnostics(
                params.uri.clone(),
                diagnostics,
                Some(params.version),
            )
            .await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_initialize() {
        let (service, _) = LspService::new(|client| Backend { client });

        let params = InitializeParams::default();
        let result = service.inner().initialize(params).await.unwrap();
        let server_info = result.server_info.unwrap();

        assert_eq!(server_info.name, "typos-lsp".to_string());
        assert_eq!(server_info.version, Some(env!("CARGO_PKG_VERSION").into()));
    }

    #[test_log::test(tokio::test)]
    async fn test_initialize_e2e() {
        let req_init =
            req(r#"{"jsonrpc":"2.0","method":"initialize","params":{"capabilities":{}},"id":1}"#);

        let mut output = Vec::new();

        let (service, socket) = LspService::new(|client| Backend { client });
        Server::new(req_init.as_ref(), &mut output, socket)
            .serve(service)
            .await;

        assert_eq!(
            body(&output).unwrap(),
            format!(
                r#"{{"jsonrpc":"2.0","result":{{"capabilities":{{"textDocumentSync":1}},"serverInfo":{{"name":"typos-lsp","version":"{}"}}}},"id":1}}"#,
                env!("CARGO_PKG_VERSION")
            )
        )
    }

    #[test_log::test(tokio::test)]
    async fn test_did_open_e2e() {
        let initialize = r#"{"jsonrpc":"2.0","method":"initialize","params":{"capabilities":{"textDocumentSync":1}},"id":1}"#;

        let did_open = r#"{
                "jsonrpc": "2.0",
                "method": "textDocument/didOpen",
                "params": {
                  "textDocument": {
                    "uri": "file:///foo.rs",
                    "languageId": "rust",
                    "version": 1,
                    "text": "this is a\ntest fo typos\n"
                  }
                }
              }
              "#;

        let (mut req_client, mut resp_client) = start_server();
        let mut buf = vec![0; 1024];

        req_client
            .write_all(req(initialize).as_bytes())
            .await
            .unwrap();
        let _ = resp_client.read(&mut buf).await.unwrap();

        tracing::info!("{}", did_open);
        req_client
            .write_all(req(did_open).as_bytes())
            .await
            .unwrap();
        let n = resp_client.read(&mut buf).await.unwrap();

        assert_eq!(
            body(&buf[..n]).unwrap(),
            r#"{"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"message":"`fo` should be `of`, `for`","range":{"end":{"character":7,"line":1},"start":{"character":5,"line":1}},"severity":2,"source":"typos-lsp"}],"uri":"file:///foo.rs","version":1}}"#,
        )
    }

    fn start_server() -> (tokio::io::DuplexStream, tokio::io::DuplexStream) {
        let (req_client, req_server) = tokio::io::duplex(1024);
        let (resp_server, resp_client) = tokio::io::duplex(1024);

        let (service, socket) = LspService::new(|client| Backend { client });

        // start server as concurrent task
        tokio::spawn(Server::new(req_server, resp_server, socket).serve(service));

        (req_client, resp_client)
    }

    fn req(msg: &str) -> String {
        format!("Content-Length: {}\r\n\r\n{}", msg.len(), msg)
    }

    fn body(src: &[u8]) -> Result<&str, anyhow::Error> {
        // parse headers to get headers length
        let mut dst = [httparse::EMPTY_HEADER; 2];

        let (headers_len, _) = match httparse::parse_headers(src, &mut dst)? {
            httparse::Status::Complete(output) => output,
            httparse::Status::Partial => return Err(anyhow::anyhow!("partial headers")),
        };

        // skip headers
        let skipped = &src[headers_len..];

        // return the rest (ie: the body) as &str
        std::str::from_utf8(skipped).map_err(anyhow::Error::from)
    }
}