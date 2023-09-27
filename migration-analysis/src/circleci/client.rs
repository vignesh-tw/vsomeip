use reqwest::header::HeaderMap;
use std::io::{Error,ErrorKind};

use super::jobs::Jobs;


#[derive(Debug)]
pub struct Client {
    pub git_slug: String,
    pub project: String,
    pub workflow: String,
    pub reporting_window: String,
    pub circleci_token: String,
}

impl Client {
    pub fn from(git_slug: &String, project: &String, workflow: &String, reporting_window: &String, circleci_token: &String) -> Result<Client, Error> {
        if git_slug.is_empty() {
            return Err(Error::new(ErrorKind::InvalidInput, "git_slug is empty"))
        }

        if project.is_empty() {
            return Err(Error::new(ErrorKind::InvalidInput, "project is empty"))
        }

        if workflow.is_empty() {
            return Err(Error::new(ErrorKind::InvalidInput, "workflow is empty"))
        }

        if reporting_window.is_empty() {
            return Err(Error::new(ErrorKind::InvalidInput, "reporting_window is empty"))
        }

        if circleci_token.is_empty() {
            return Err(Error::new(ErrorKind::InvalidInput, "circleci_token is empty"))
        }
        
        Ok(Client {
            git_slug: git_slug.to_string(),
            project: project.to_string(),
            workflow: workflow.to_string(),
            reporting_window: reporting_window.to_string(),
            circleci_token: circleci_token.to_string(),
        })
    }

    fn jobs_url(&self) -> String {
        let url = format!(
            "https://circleci.com/api/v2/insights/{}/{}/workflows/{}/jobs?branch=main&reporting-window={}",
            self.git_slug,
            self.project,
            self.workflow,
            self.reporting_window
        );
        
        return url.to_string();
    }

    fn headers(&self) -> HeaderMap {
        let auth_header = format!("circle-token {}", &self.circleci_token);
        let mut headers = HeaderMap::new();
        headers.insert("authorization", auth_header.parse().unwrap());
        return headers;
    }

    pub async fn get(&self) -> Jobs {
        let client = reqwest::Client::new();
        
        return client.get(self.jobs_url())
            .headers(self.headers()) 
            .send()
            .await
            .expect("failed to retrieve response")
            .json::<Jobs>()
            .await
            .expect("failed to deserialize response"); 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialisation_fails_if_git_slug_is_omitted() {
        let git_slug = String::from("");
        let project = String::from("project"); 
        let workflow = String::from("workflow");
        let reporting_window = String::from("reporting_window"); 
        let circleci_token = String::from("circleci_token");
        
        let circleci_result = Client::from(
            &git_slug,
            &project,
            &workflow,
            &reporting_window,
            &circleci_token,
        );
        
        let expected_error = circleci_result.unwrap_err();

        assert_eq!(expected_error.kind(), ErrorKind::InvalidInput);
        assert_eq!(expected_error.into_inner().unwrap().to_string(), "git_slug is empty")    
    }

    #[test]
    fn initialisation_fails_if_project_is_omitted() {
        let git_slug = String::from("git_slug");
        let project = String::from(""); 
        let workflow = String::from("workflow");
        let reporting_window = String::from("reporting_window"); 
        let circleci_token = String::from("circleci_token");
        
        let circleci_result = Client::from(
            &git_slug,
            &project,
            &workflow,
            &reporting_window,
            &circleci_token,
        );
        
        let expected_error = circleci_result.unwrap_err();

        assert_eq!(expected_error.kind(), ErrorKind::InvalidInput);
        assert_eq!(expected_error.into_inner().unwrap().to_string(), "project is empty")    
    }

    #[test]
    fn initialisation_fails_if_workflow_is_omitted() {
        let git_slug = String::from("git_slug");
        let project = String::from("project"); 
        let workflow = String::from("");
        let reporting_window = String::from("reporting_window"); 
        let circleci_token = String::from("circleci_token");
        
        let circleci_result = Client::from(
            &git_slug,
            &project,
            &workflow,
            &reporting_window,
            &circleci_token,
        );
        
        let expected_error = circleci_result.unwrap_err();

        assert_eq!(expected_error.kind(), ErrorKind::InvalidInput);
        assert_eq!(expected_error.into_inner().unwrap().to_string(), "workflow is empty")    
    }

    #[test]
    fn initialisation_fails_if_reporting_window_is_omitted() {
        let git_slug = String::from("git_slug");
        let project = String::from("project"); 
        let workflow = String::from("workflow");
        let reporting_window = String::from(""); 
        let circleci_token = String::from("circleci_token");
        
        let circleci_result = Client::from(
            &git_slug,
            &project,
            &workflow,
            &reporting_window,
            &circleci_token,
        );
        
        let expected_error = circleci_result.unwrap_err();

        assert_eq!(expected_error.kind(), ErrorKind::InvalidInput);
        assert_eq!(expected_error.into_inner().unwrap().to_string(), "reporting_window is empty")    
    }

    #[test]
    fn initialisation_fails_if_circleci_token_is_omitted() {
        let git_slug = String::from("git_slug");
        let project = String::from("project"); 
        let workflow = String::from("workflow");
        let reporting_window = String::from("reporting_window"); 
        let circleci_token = String::from("");
        
        let circleci_result = Client::from(
            &git_slug,
            &project,
            &workflow,
            &reporting_window,
            &circleci_token,
        );
        
        let expected_error = circleci_result.unwrap_err();

        assert_eq!(expected_error.kind(), ErrorKind::InvalidInput);
        assert_eq!(expected_error.into_inner().unwrap().to_string(), "circleci_token is empty")    
    }

    #[test]
    fn initialisation_returns_circleci_client_instance() {
        let git_slug = String::from("git_slug");
        let project = String::from("project"); 
        let workflow = String::from("workflow");
        let reporting_window = String::from("reporting_window"); 
        let circleci_token = String::from("cicleci_token");
        let circleci_client_result = Client::from(
            &git_slug,
            &project,
            &workflow,
            &reporting_window,
            &circleci_token,
        );

        let circleci = circleci_client_result.unwrap();
        
        assert_eq!(&circleci.git_slug, &git_slug);
        assert_eq!(&circleci.project, &project);
        assert_eq!(&circleci.workflow, &workflow);
        assert_eq!(&circleci.reporting_window, &reporting_window);
        assert_eq!(&circleci.circleci_token, &circleci_token);
    }

    #[test]
    fn jobs_url_returns_concatenated_jobs_url() {
        let git_slug = String::from("git_slug");
        let project = String::from("project"); 
        let workflow = String::from("workflow");
        let reporting_window = String::from("reporting_window"); 
        let circleci_token = String::from("cicleci_token");
        let circleci = Client::from(
            &git_slug,
            &project,
            &workflow,
            &reporting_window,
            &circleci_token,
        ).unwrap();

        let url = circleci.jobs_url();

        assert_eq!(url, format!("https://circleci.com/api/v2/insights/{}/{}/workflows/{}/jobs?branch=main&reporting-window={}",&git_slug,&project,&workflow,&reporting_window)); 
    }

    #[test]
    fn headers_return_headers_containing_authorization() {
        let git_slug = String::from("git_slug");
        let project = String::from("project"); 
        let workflow = String::from("workflow");
        let reporting_window = String::from("reporting_window"); 
        let circleci_token = String::from("cicleci_token");
        let circleci = Client::from(
            &git_slug,
            &project,
            &workflow,
            &reporting_window,
            &circleci_token,
        ).unwrap();

        let headers = circleci.headers();
        let auth_header = headers.get("authorization").unwrap();

        let header_string = auth_header.to_str().unwrap();
        assert_eq!(format!("circle-token {}", circleci_token), header_string);
    }

}
