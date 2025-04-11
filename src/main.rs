use aicomment_rs::{
    commit::{generate_commit, read_template},
    config::get_config,
    diff::get_diff,
};

#[derive(Debug)]
enum ErrorCode {
    ReadingTemplate,
    GeneratingCommit,
    GettingDiff,
}

#[tokio::main]
async fn main() {
    let mut error_code: Option<ErrorCode> = None;

    match get_diff() {
        Ok(diff) => {
            if let Some(config) = get_config() {
                match read_template() {
                    Ok(template) => {
                        let result = generate_commit(
                            template.replace("{{diff}}", &diff),
                            config.openai_api_key,
                            config.openai_api_url,
                        )
                        .await;

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
                }
            }
        }
        Err(_) => {
            error_code = Some(ErrorCode::GettingDiff);
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
