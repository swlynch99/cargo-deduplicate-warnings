use std::collections::HashSet;

use cargo_metadata::Message;
use clap::Parser;

/// A simple cargo command to strip duplicate warnings from the json output of a
/// cargo command.
///
/// Cargo json input is read on stdin and deduplicated output is emitted to
/// stdout.
#[derive(Clone, Debug, clap::Parser)]
struct Args {
    // Ignore the `deduplicate-warnings` argument if present.
    #[arg(hide = true)]
    subcommand: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let _args = Args::parse();
    let stdin = std::io::stdin();
    let mut seen = HashSet::new();

    for message in Message::parse_stream(stdin.lock()) {
        let message = message?;
        let text = match message {
            Message::TextLine(line) => line,
            Message::CompilerMessage(msg) => {
                if !seen.insert(msg.clone()) {
                    continue;
                }

                serde_json::to_string(&Message::CompilerMessage(msg))?
            }
            message => serde_json::to_string(&message)?,
        };

        println!("{text}")
    }

    Ok(())
}
