
use tower_lsp::Server;
use tower_lsp::LspService;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_writer(std::io::stderr).init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| typos_lsp::Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
