mod circleci;

use circleci::client::Client;
use circleci::migration::Analysis;
use circleci::session::Session;
use clap::Parser;

const DEFAULT_CONFIG: &str = "config.json";
const DEFAULT_SLUG: &str = "github/vignesh-tw";
const DEFAULT_PROJECT: &str = "vsomeip";
const DEFAULT_WORKFLOW: &str = "build-test-deploy";
const DEFAULT_REPORTING_WINDOW: &str = "last-7-days";

/// A CLI to get circleci jobs insights 
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Config file path
    #[arg(short, long, default_value_t = String::from(DEFAULT_CONFIG))]
    config: String,

    /// Pipeline git slug
    #[arg(short, long, default_value_t = String::from(DEFAULT_SLUG))]
    slug: String,

    /// Project's name
    #[arg(short, long, default_value_t = String::from(DEFAULT_PROJECT))]
    project: String,
    
    /// Workflow's name
    #[arg(short, long, default_value_t = String::from(DEFAULT_WORKFLOW))]
    workflow: String,

    /// Reporting period "last-7-days" "last-90-days" "last-24-hours" "last-30-days" "last-60-days"
    #[arg(short, long, default_value_t = String::from(DEFAULT_REPORTING_WINDOW))]
    reporting: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let session = Session::from(&args.config);
    let auth = session.get_auth().unwrap();

    let client = Client::from(
        &args.slug,
        &args.project,
        &args.workflow,
        &args.reporting,
        &String::from(auth),
    )
    .unwrap();

    let jobs = client.get_jobs().await;

    let insights = Analysis {}
        .get_insights(
            &String::from("bazel_build"),
            &String::from("cmake_build"),
            &jobs,
        )
        .unwrap();

    println!("{:?}", &insights)
}
