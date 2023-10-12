use super::jobs::{Jobs,Item};
use std::fmt;
use std::io::{Error,ErrorKind};

#[derive(Debug)]
pub struct Insight {
    pub base_job: String,
    pub migration_job: String,
    pub min_differential: f64,
    pub mean_differential: f64,
    pub median_differential: f64,
    pub max_differential: f64,
    pub window_start: String,
    pub window_end: String,
}

impl fmt::Display for Insight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = format!(r#"
        Migration analysis:

        Details:

            base job: {}
            migration job: {}
            window start: {}
            window end: {}

        Data:

            minimum duration - differential: {}
            maximum duration - differential: {}
            mean duration - differential: {}
            median duration - differential: {}
        "#, 
        self.base_job,
        self.migration_job,
        self.window_start,
        self.window_end,
        self.min_differential,
        self.max_differential,
        self.mean_differential,
        self.median_differential);
        write!(f, "{}", display)
    }
}

#[derive(Debug)]
pub struct Analysis {
}

impl Analysis {
    pub fn get_insights(&self, migration_job_name: &String, base_job_name: &String, jobs: &Jobs) -> Result<Insight, Error> {
        let base_workflow_option = get_item_with_name(&jobs.items, base_job_name);
        if base_workflow_option.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, format!("failed to find workflow {} in retrieved jobs", &base_job_name))); 
        }
        let base_workflow = base_workflow_option.unwrap();

        let migration_workflow_option = get_item_with_name(&jobs.items, migration_job_name);
        if migration_workflow_option.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, format!("failed to find workflow {} in retrieved jobs", &migration_job_name))); 
        }
        let migration_workflow = migration_workflow_option.unwrap();

        Ok(Insight {
            base_job: String::from(base_job_name),
            migration_job: String::from(migration_job_name), 
            min_differential: (migration_workflow.metrics.duration_metrics.min as f64 - base_workflow.metrics.duration_metrics.min as f64),
            mean_differential: (migration_workflow.metrics.duration_metrics.mean as f64 - base_workflow.metrics.duration_metrics.mean as f64),
            median_differential: (migration_workflow.metrics.duration_metrics.median as f64 - base_workflow.metrics.duration_metrics.median as f64),
            max_differential: (migration_workflow.metrics.duration_metrics.max as f64 - base_workflow.metrics.duration_metrics.max as f64),
            window_start: migration_workflow.window_start,
            window_end: migration_workflow.window_end,
         })
    }
}

fn get_item_with_name(items: &Vec<Item>, name: &String) -> Option<Item> {
    let item_option = items.iter().find(|i| &i.name == name);
    match item_option {
        Some(item) => return Some(item.clone()),
        None => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::jobs::{Metrics,DurationMetrics};
    
    #[test]
    fn get_insights_fails_if_base_workflow_not_found() {
        let base_workflow_name = String::from("base workflow");
        let base_workflow_item = base_workflow_item(&base_workflow_name);
        let migration_workflow_name = String::from("migration workflow");
        let migration_workflow_item = migration_workflow_item(&migration_workflow_name);

        let jobs = Jobs{
            next_page_token: None,
            items: vec![]
        };

        let analysis = Analysis{};
        let actual_err = analysis.get_insights(&migration_workflow_item.name,&base_workflow_item.name, &jobs).unwrap_err();

        assert_eq!(actual_err.kind(), ErrorKind::InvalidInput);
        assert_eq!(actual_err.into_inner().unwrap().to_string(), format!("failed to find workflow {} in retrieved jobs", &base_workflow_item.name))   
    }

    #[test]
    fn get_insights_fails_if_migration_workflow_not_found() {
        let base_workflow_name = String::from("base workflow");
        let base_workflow_item = base_workflow_item(&base_workflow_name);
        let migration_workflow_name = String::from("migration workflow");

        let jobs = Jobs{
            next_page_token: None,
            items: vec![base_workflow_item]
        };

        let analysis = Analysis{};
        let actual_err = analysis.get_insights(&migration_workflow_name,&base_workflow_name, &jobs).unwrap_err();

        assert_eq!(actual_err.kind(), ErrorKind::InvalidInput);
        assert_eq!(actual_err.into_inner().unwrap().to_string(), format!("failed to find workflow {} in retrieved jobs", &migration_workflow_name))   
    }

    #[test]
    fn get_insights() {
        let base_job_name: String = String::from("base workflow");
        let base_workflow_item = base_workflow_item(&base_job_name);
        let base_workflow_min = base_workflow_item.metrics.duration_metrics.min;
        let base_workflow_median = base_workflow_item.metrics.duration_metrics.median;
        let base_workflow_mean = base_workflow_item.metrics.duration_metrics.mean;
        let base_workflow_max = base_workflow_item.metrics.duration_metrics.max;
        
        let migration_workflow_name = String::from("migration workflow");
        let migration_workflow_item = migration_workflow_item(&migration_workflow_name);
        let migration_workflow_min = migration_workflow_item.metrics.duration_metrics.min;
        let migration_workflow_median = migration_workflow_item.metrics.duration_metrics.median;
        let migration_workflow_mean = migration_workflow_item.metrics.duration_metrics.mean;
        let migration_workflow_max = migration_workflow_item.metrics.duration_metrics.max;

        let jobs = Jobs{
            next_page_token: None,
            items: vec![base_workflow_item,migration_workflow_item]
        };

        let analysis = Analysis{};
        let  insights = analysis.get_insights(&migration_workflow_name,&base_job_name, &jobs).unwrap();

        let min_differential  = migration_workflow_min as f64 - base_workflow_min as f64;
        let median_differential  = migration_workflow_median as f64 - base_workflow_median as f64;
        let mean_differential = migration_workflow_mean as f64 - base_workflow_mean as f64;
        let max_differential = migration_workflow_max as f64 - base_workflow_max as f64;

        assert_eq!(base_job_name,insights.base_job);
        assert_eq!(migration_workflow_name, insights.migration_job);
        assert_eq!(min_differential,insights.min_differential);
        assert_eq!(median_differential,insights.median_differential);
        assert_eq!(mean_differential,insights.mean_differential);
        assert_eq!(max_differential,insights.max_differential);
    }

    #[test]
    fn display_insights_returns_formatted_insights() {
        let base_job: String = String::from("base job");
        let migration_job: String = String::from("migration job");
        let min_differential: f64 = 1.0;
        let mean_differential: f64 = 1.0;
        let median_differential: f64 = 1.0;
        let max_differential: f64 = 1.0;
        let window_start: String = String::from("start");
        let window_end: String = String::from("end");

        let expected_display = format!(r#"
        Migration analysis:

        Details:

            base job: {}
            migration job: {}
            window start: {}
            window end: {}

        Data:

            minimum duration - differential: {}
            maximum duration - differential: {}
            mean duration - differential: {}
            median duration - differential: {}
        "#,
        &base_job,
        &migration_job,
        &window_start,
        &window_end,
        &min_differential,
        &max_differential,
        &mean_differential,
        &median_differential);

        let insights = Insight {
            base_job,
            migration_job,
            min_differential,
            mean_differential,
            median_differential,
            max_differential,
            window_start,
            window_end
        };

        let insight_display = format!("{}", insights);

        assert_eq!(insight_display,expected_display);
    }

    fn base_workflow_item(workflow_name: &String) -> Item { 
        let duration_metrics = DurationMetrics {
            min: 1,
            mean: 4,
            median: 5,
            p95: 8,
            max: 12,
            standard_deviation: 1.1,
            total_duration: 23,
        };

        let metrics = Metrics {
            total_runs: 7,
            failed_runs: 2,
            successful_runs: 5,
            median_credits_used: 4,
            duration_metrics: duration_metrics,
            success_rate: 0.9,
            total_credits_used: 4,
            throughput: 5.0,
        };

        return Item {
            name: workflow_name.to_string(),
            metrics: metrics,
            window_start: "start".to_string(),
            window_end: "end".to_string(),
        }; 
    }

    fn migration_workflow_item(workflow_name: &String) -> Item {
        let duration_metrics = DurationMetrics {
            min: 1,
            mean: 3,
            median: 4,
            p95: 7,
            max: 11,
            standard_deviation: 1.0,
            total_duration: 22,
        };

        let metrics = Metrics {
            total_runs: 7,
            failed_runs: 2,
            successful_runs: 5,
            median_credits_used: 4,
            duration_metrics: duration_metrics,
            success_rate: 0.9,
            total_credits_used: 4,
            throughput: 5.0,
        };

        return Item {
            name: workflow_name.to_string(),
            metrics: metrics,
            window_start: "start".to_string(),
            window_end: "end".to_string(),
        };  
    }
}