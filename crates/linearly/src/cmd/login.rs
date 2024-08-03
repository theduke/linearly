use std::io::Write;

use anyhow::Context;
use colored::Colorize;
use cynic::QueryBuilder;

use linear_api::Client;

use crate::{
    cmd::CliCommand,
    config::{UserConfig, LINEAR_API_KEY_SETTINGS_URL},
};

#[derive(clap::Parser)]
pub struct CmdLogin {
    #[clap(long, env = "LINEAR_TOKEN")]
    token: Option<String>,
}

impl CliCommand for CmdLogin {
    type Output = ();

    async fn run(self) -> Result<(), anyhow::Error> {
        let token = if let Some(v) = self.token {
            v
        } else {
            eprintln!(
                "Create an personal API KEY at {}",
                LINEAR_API_KEY_SETTINGS_URL
            );
            loop {
                eprint!("API KEY: ");
                std::io::stdout().flush()?;
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf)?;

                let value = buf.trim();
                if !value.is_empty() {
                    break value.to_string();
                }
            }
        };

        // Validate token.

        eprintln!("Validating token...");

        let client = Client::new_default(token.clone());

        let viewer = client
            .run(linear_api::schema::viewer::QueryViewer::build(()))
            .await?
            .data
            .context("could not retrieve current user")?
            .viewer;

        let (_, path) = UserConfig::update_token(token)?;

        eprintln!(
            "{} as user {}",
            "Sucessfully logged in".bold(),
            viewer.display_name.bold()
        );

        eprintln!();
        eprintln!("Config saved at: {}", path.display());

        Ok(())
    }
}
