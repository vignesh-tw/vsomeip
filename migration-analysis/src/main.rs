mod circleci;

use circleci::client::Client;
use circleci::migration::Analysis;
use circleci::session::Session;

const DEFAULT_CONFIG: &str = "config.json";
const DEFAULT_SLUG: &str = "github/vignesh-tw";
const DEFAULT_PROJECT: &str = "vsomeip";
const DEFAULT_WORKFLOW: &str = "build-test-deploy";
const DEFAULT_REPORTING_WINDOW: &str = "last-7-days";

#[tokio::main]
async fn main() {
    let session = Session::from(&DEFAULT_CONFIG.to_string());
    let auth = session.get_auth().unwrap();

    let client = Client::from(
        &String::from(DEFAULT_SLUG),
        &String::from(DEFAULT_PROJECT),
        &String::from(DEFAULT_WORKFLOW),
        &String::from(DEFAULT_REPORTING_WINDOW),
        &String::from(auth),
    )
    .unwrap();

    let jobs = client.get_jobs().await;

    let analysis = Analysis {};
    let insights = analysis
        .get_insights(
            &String::from("bazel_build"),
            &String::from("cmake_build"),
            &jobs,
        )
        .unwrap();

    println!("{:?}", &insights)
}
