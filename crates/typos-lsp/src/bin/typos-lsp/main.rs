use tower_lsp::LspService;
use tower_lsp::Server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| typos_lsp::Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
