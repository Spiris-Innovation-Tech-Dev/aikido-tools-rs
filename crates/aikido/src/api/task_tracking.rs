use crate::client::AikidoClient;
use crate::error::Result;
use crate::models::*;
use serde_json::Value;

impl AikidoClient {
    pub async fn list_task_tracking_projects(&self) -> Result<Vec<TaskTrackingProject>> {
        self.get("/task_tracking/projects").await
    }

    pub async fn list_tasks_from_project(&self, project_id: &str) -> Result<Vec<TaskTrackingTask>> {
        let project_id = crate::client::encode_path_segment(project_id, "project_id")?;
        self.get(&format!("/task_tracking/projects/{project_id}/tasks"))
            .await
    }

    pub async fn link_task_to_issue_group(
        &self,
        body: &LinkTaskToIssueGroupRequest,
    ) -> Result<Value> {
        self.post("/task_tracking/linkTaskToIssueGroup", body).await
    }

    pub async fn map_repos_to_project(&self, body: &MapReposToProjectRequest) -> Result<Value> {
        self.post("/task_tracking/mapCodeReposToProjects", body)
            .await
    }
}
