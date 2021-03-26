mod response;

use std::process::exit;

use ansi_term::Color;
use argh::FromArgs;
use eyre::Result;
use response::{WelcomeElement};

#[derive(FromArgs)]
/// def-rs Description
struct DictArgs {
    /// which word to define
    #[argh(positional)]
    word: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: DictArgs = argh::from_env();
    let url = format!(
        "https://www.dictionaryapi.com/api/v3/references/collegiate/json/{}?key={}",
        args.word,
        env!("DICTIONARY_KEY")
    );
    let res = reqwest::get(url).await?;
    let text = res.text().await?;
    let deser = serde_json::from_str::<Vec<WelcomeElement>>(text.as_str());
    let json = match deser {
        Ok(e) => {e}
        Err(_) => {
            eprintln!("Please check the spelling. Possible values are:");
            let possible: Vec<String> = serde_json::from_str(&text)?;
            for p in possible {
                println!("  {}", Color::Red.paint(p));
            }
            exit(1);
        }
    };

    let json = json
        .iter()
        .filter(|e| e.meta.id == args.word || e.meta.id.contains(":"))
        .collect::<Vec<&WelcomeElement>>();
    for def in json {
        println!("");
        println!(
            "{}  ({})",
            Color::Cyan.bold().paint(&def.meta.id),
            Color::White.dimmed().paint(&def.fl)
        );
        println!(
            "🕪 {} {}",
            Color::Yellow.paint(&def.hwi.hw),
            match &def.hwi.prs {
                Some(prs) => format!("| {}", Color::Yellow.paint(prs[0].mw.to_string())),
                None => String::from(""),
            }
        );
        println!("");
        println!(
            "Definition of {}:",
            Color::Yellow.italic().paint(&def.meta.id)
        );
        for i in &def.shortdef {
            println!("    {} {}", Color::Green.paint("+"), i);
        }
    }
    Ok(())
}
