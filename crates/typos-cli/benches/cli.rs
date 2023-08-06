use calliper::{Runner, Scenario, ScenarioConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runner = Runner::default().config(ScenarioConfig::default().branch_sim(true));
    let benches = [
        Scenario::new_with_command(cmd(&["benches/data/empty.txt"])).name("empty.txt"),
        Scenario::new_with_command(cmd(&["benches/data/no_tokens.txt"])).name("no_tokens.txt"),
        Scenario::new_with_command(cmd(&["benches/data/single_token.txt"]))
            .name("single_token.txt"),
        Scenario::new_with_command(cmd(&["benches/data/sherlock.txt"])).name("sherlock.txt"),
        Scenario::new_with_command(cmd(&["benches/data/code.txt"])).name("code.txt"),
        Scenario::new_with_command(cmd(&["../typos-dict/assets/words.csv"])).name("words.csv"),
    ];
    if let Some(results) = runner.run(&benches)? {
        for res in results.into_iter() {
            println!("{}", res.parse());
        }
    }
    Ok(())
}

fn cmd(args: &[&str]) -> std::process::Command {
    let bin = std::path::Path::new(env!("CARGO_BIN_EXE_typos"));

    let mut cmd = std::process::Command::new(bin);
    cmd.args(args);

    cmd
}
