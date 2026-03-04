use crate::client::AikidoClient;
use crate::error::Result;
use crate::models::*;
use serde_json::Value;

impl AikidoClient {
    pub async fn list_open_issue_groups(&self) -> Result<Vec<IssueGroup>> {
        self.get("/open-issue-groups").await
    }

    pub async fn get_issue_group(&self, group_id: i64) -> Result<IssueGroup> {
        self.get(&format!("/issues/groups/{group_id}")).await
    }

    pub async fn get_issue_group_issues(&self, group_id: i64) -> Result<Vec<Issue>> {
        let all: Vec<Issue> = self.get("/issues/export").await?;
        Ok(all.into_iter().filter(|i| i.group_id == group_id).collect())
    }

    pub async fn ignore_issue_group(&self, group_id: i64, body: &IgnoreRequest) -> Result<Value> {
        self.put(&format!("/issues/groups/{group_id}/ignore"), body)
            .await
    }

    pub async fn unignore_issue_group(&self, group_id: i64) -> Result<Value> {
        self.put_no_content(
            &format!("/issues/groups/{group_id}/unignore"),
            &serde_json::json!({}),
        )
        .await
        .map(|_| Value::Null)
    }

    pub async fn snooze_issue_group(
        &self,
        group_id: i64,
        body: &SnoozeRequest,
    ) -> Result<Value> {
        self.put(&format!("/issues/groups/{group_id}/snooze"), body)
            .await
    }

    pub async fn unsnooze_issue_group(&self, group_id: i64) -> Result<Value> {
        self.put_no_content(
            &format!("/issues/groups/{group_id}/unsnooze"),
            &serde_json::json!({}),
        )
        .await
        .map(|_| Value::Null)
    }

    pub async fn add_note_to_issue_group(
        &self,
        group_id: i64,
        body: &NoteRequest,
    ) -> Result<Value> {
        self.post(&format!("/issues/groups/{group_id}/notes"), body)
            .await
    }

    pub async fn get_issue_group_tasks(&self, group_id: i64) -> Result<Vec<IssueGroupTask>> {
        self.get(&format!("/issues/groups/{group_id}/tasks")).await
    }
}
