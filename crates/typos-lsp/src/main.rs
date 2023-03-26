use tower_lsp::lsp_types::*;
use tower_lsp::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

mod check;

#[derive(Debug)]
struct Backend {
    client: Client,
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
        let dict = typos_cli::dict::BuiltIn::new(Default::default());
        let tokenizer = typos::tokens::Tokenizer::new();
        let policy = typos_cli::policy::Policy::new()
            .dict(&dict)
            .tokenizer(&tokenizer);

        check::check_file(std::path::Path::new("-"), true, &policy, &PrintTrace);

        self.create_diagnostics(TextDocumentItem {
            language_id: params.text_document.language_id,
            uri: params.text_document.uri,
            text: params.text_document.text,
            version: params.text_document.version,
        })
        .await
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct PrintTrace;

impl typos_cli::report::Report for PrintTrace {
    fn report(&self, _msg: typos_cli::report::Message) -> Result<(), std::io::Error> {
        tracing::info!("report: {:?}", _msg);
        Ok(())
    }
}

impl Backend {
    async fn create_diagnostics(&self, params: TextDocumentItem) {
        let diagnostics = vec![Diagnostic::default()];
        self.client
            .publish_diagnostics(params.uri.clone(), diagnostics, Some(params.version))
            .await;
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
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
        let req_init = with_headers(
            r#"{"jsonrpc":"2.0","method":"initialize","params":{"capabilities":{}},"id":1}"#,
        );

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
    async fn test_did_open() {
        let req_init = with_headers(
            r#"{"jsonrpc":"2.0","method":"initialize","params":{"capabilities":{"textDocumentSync":1}},"id":1}"#,
        );

        let req_open = with_headers(
            r#"{
                "jsonrpc": "2.0",
                "method": "textDocument/didOpen",
                "params": {
                  "textDocument": {
                    "uri": "file:///foo.rs",
                    "languageId": "rust",
                    "version": 1,
                    "text": "foobar"
                  }
                }
              }
              "#,
        );

        let (mut req_client, req_server) = tokio::io::duplex(1024);
        let (resp_server, mut resp_client) = tokio::io::duplex(1024);

        let (service, socket) = LspService::new(|client| Backend { client });

        // start server as concurrent task
        tokio::spawn(Server::new(req_server, resp_server, socket).serve(service));

        let mut buf = vec![0; 1024];

        req_client.write_all(req_init.as_ref()).await.unwrap();
        let n = resp_client.read(&mut buf).await.unwrap();
        println!("{}", String::from_utf8_lossy(&buf[..n]));

        req_client.write_all(req_open.as_ref()).await.unwrap();
        let n = resp_client.read(&mut buf).await.unwrap();
        println!("{}", String::from_utf8_lossy(&buf[..n]));
    }

    fn with_headers(msg: &str) -> Vec<u8> {
        format!("Content-Length: {}\r\n\r\n{}", msg.len(), msg).into_bytes()
    }

    fn body(mut src: &[u8]) -> Result<&str, anyhow::Error> {
        // parse headers to get headers length
        let mut dst = [httparse::EMPTY_HEADER; 2];

        let (headers_len, _) = match httparse::parse_headers(src, &mut dst)? {
            httparse::Status::Complete(output) => output,
            httparse::Status::Partial => return Err(anyhow::anyhow!("partial headers")),
        };

        // skip headers
        src = &src[headers_len..];

        // return the rest (ie: the body) as &str
        std::str::from_utf8(src).map_err(anyhow::Error::from)
    }
}
