mod circleci;
mod cli;
mod config;

use circleci::client::Client;
use circleci::migration::{Analysis, Insight};
use circleci::jobs::Jobs;
use clap::{Parser};
use cli::app::{Actions,App, Config};
use config::manager::Manager;


const DEFAULT_CONFIG: &str = "config.json";
const DEFAULT_SLUG: &str = "github/vignesh-tw";
const DEFAULT_PROJECT: &str = "vsomeip";
const DEFAULT_WORKFLOW: &str = "build-test-deploy";
const DEFAULT_REPORTING_WINDOW: &str = "last-7-days";



#[tokio::main]
async fn main() {
    let cli = App::parse();
    match &cli.command {
        Actions::Config(arg) => {
            let mut authorization = String::from("");
            let mut project = String::from("");
            let mut slug = String::from("");
            match &arg.auth {
                Some(auth_arg) => {
                    _ = &authorization.replace_range(.., &auth_arg);
                },
                None => (),
            }
            match &arg.project {
                Some(project_arg) => {
                    _ = &project.replace_range(.., &project_arg);
                },
                None => (),
            }
            match &arg.slug {
                Some(slug_arg) => {
                    _ = &slug.replace_range(.., &slug_arg);
                },
                None => (),                
            }
            let config_manager = Manager::new(None);
            config_manager.write_config(authorization, project, slug);
        },
        Actions::Analysis(arg) => {
            println!("not implemented yet!")
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