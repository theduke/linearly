use crate::config::UserConfig;

pub mod issue;
pub mod login;
pub mod project;
pub mod team;

#[derive(clap::Parser)]
pub struct Args {
    #[clap(subcommand)]
    cmd: Cmd,
}

impl CliCommand for Args {
    type Output = ();

    async fn run(self) -> Result<(), anyhow::Error> {
        match self.cmd {
            Cmd::Project(c) => c.run().await,
            Cmd::Team(c) => c.run().await,
            Cmd::Issue(c) => c.run().await,
            Cmd::Login(c) => c.run().await,
        }
    }
}

#[derive(clap::Subcommand)]
pub enum Cmd {
    #[clap(subcommand)]
    Project(project::CmdProject),
    #[clap(subcommand)]
    Team(team::CmdTeam),
    #[clap(subcommand, alias = "i")]
    Issue(issue::CmdIssue),

    Login(login::CmdLogin),
}

pub trait CliCommand {
    type Output;
    fn run(self) -> impl std::future::Future<Output = Result<Self::Output, anyhow::Error>> + Send;
}

#[derive(Debug)]
pub struct NoTokenError;

impl std::fmt::Display for NoTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No Linear API token configured. Run 'linear login'!")
    }
}

impl std::error::Error for NoTokenError {}

#[derive(clap::Parser)]
pub struct CommonArgs {
    #[clap(long, env = "LINEAR_TOKEN")]
    pub api_token: Option<String>,
}

impl CommonArgs {
    pub fn client(&self) -> Result<linear_api::Client, anyhow::Error> {
        let token = if let Some(token) = self.api_token.clone() {
            token
        } else if let Some(token) = UserConfig::load()?.and_then(|x| x.token) {
            token
        } else {
            return Err(NoTokenError.into());
        };
        Ok(linear_api::Client::new_default(token))
    }
}

#[derive(clap::ValueEnum, Clone)]
pub enum Format {
    Table,
    Json,
}
