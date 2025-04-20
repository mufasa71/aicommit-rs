use std::path::PathBuf;

use aicommit_rs::{
    commit::{generate_commit, read_template},
    config::get_config,
    diff::get_diff,
};
use clap::{Command, ValueHint, arg, value_parser};

fn build_cli() -> Command {
    let mut template_path = dirs::home_dir().expect("home dir expected");
    template_path.push(".aicommit-template");

    Command::new("aicommit-rs")
        .version("0.0.7")
        .about("Uses OpenAI or Google AI to generate commit message suggestions based on the diff between the current branch and master.
Then, you can select a commit message from the list and use it to commit your changes.")
        .next_line_help(true)
        .arg(
            arg!(-t --template <FILE> "Specify a custom template")
                .value_hint(ValueHint::AnyPath)
                .default_value(template_path.into_os_string())
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(--usage "Show usage").required(false))
}

#[derive(Debug)]
enum ErrorCode {
    ReadingTemplate,
    GeneratingCommit,
    GettingDiff,
}

#[tokio::main]
async fn main() {
    let matches = build_cli().get_matches();

    if matches.get_flag("usage") {
        let mut cmd = build_cli();
        eprintln!("Generating usage spec...");
        clap_usage::generate(&mut cmd, "aicommit-rs", &mut std::io::stdout());
        return;
    }

    let mut error_code: Option<ErrorCode> = None;

    if let Some(config) = get_config() {
        match get_diff() {
            Ok(diff) => match read_template(
                matches
                    .get_one::<PathBuf>("template")
                    .expect("no default template provided"),
            ) {
                Ok(template) => {
                    let result = generate_commit(template.replace("{{diff}}", &diff), config).await;

                    match result {
                        Ok(commit_message) => println!("{}", commit_message),
                        Err(_) => {
                            error_code = Some(ErrorCode::GeneratingCommit);
                        }
                    }
                }
                Err(_) => {
                    error_code = Some(ErrorCode::ReadingTemplate);
                }
            },
            Err(_) => {
                error_code = Some(ErrorCode::GettingDiff);
            }
        }
    }

    if let Some(ec) = error_code {
        match ec {
            ErrorCode::ReadingTemplate => {
                println!("Error reading template file");
            }
            ErrorCode::GeneratingCommit => {
                println!("Error generating commit message");
            }
            ErrorCode::GettingDiff => {
                println!("Error getting diff");
            }
        }
    }
}

#[test]
fn verify_cmd() {
    build_cli().debug_assert();
}
