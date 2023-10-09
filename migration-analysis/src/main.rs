mod circleci;
mod cli;
mod config;

use circleci::client::Client;
use circleci::migration::{Analysis, Insight};
use circleci::jobs::Jobs;
use clap::Parser;
use cli::app::{Actions,App};
use config::manager::Manager;


#[tokio::main]
async fn main() {
    let cli = App::parse();
    let config_manager = Manager::new(None);
    match &cli.command {
        Actions::Config(arg) => {
            let authorization = match &arg.auth {
                Some(auth_val) => String::from(auth_val),
                None => String::from(""),
            };
            let project = match &arg.project {
                Some(project_val) => String::from(project_val),
                None => String::from(""),
            };
            let slug = match &arg.slug {
                Some(slug_val) => String::from(slug_val),
                None => String::from(""),
            };
            config_manager.write_config(authorization, project, slug);
        },
        Actions::Analysis(arg) => {
            let config = config_manager.read_config();
            let jobs = retrieve_jobs(&config.slug, &config.project, &arg.workflow, &arg.reporting_window, &config.authorization).await;
            let insights = get_jobs_insights(jobs);
            println!("{}", insights);
        }
    }
}

async fn retrieve_jobs(slug: &String,project:&String,workflow:&String,reporting_window:&String,auth:&String) -> Jobs {
    let client = Client::from(
        slug,
        project,
        workflow,
        reporting_window,
        auth,
    )
    .unwrap();

    return client.get_jobs().await;
}

fn get_jobs_insights(jobs: Jobs) -> Insight {
    let insights = Analysis {}
        .get_insights(
            &String::from("bazel_build"),
            &String::from("cmake_build"),
            &jobs,
        )
        .unwrap();
    return insights;
}