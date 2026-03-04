use crate::client::AikidoClient;
use crate::error::Result;
use crate::models::*;
use serde_json::Value;

impl AikidoClient {
    pub async fn list_teams(&self) -> Result<Vec<Team>> {
        self.get("/teams").await
    }

    pub async fn create_team(&self, body: &CreateTeamRequest) -> Result<Value> {
        self.post("/teams", body).await
    }

    pub async fn update_team(&self, team_id: i64, body: &UpdateTeamRequest) -> Result<Value> {
        self.put(&format!("/teams/{team_id}"), body).await
    }

    pub async fn delete_team(&self, team_id: i64) -> Result<()> {
        self.delete(&format!("/teams/{team_id}")).await
    }

    pub async fn add_user_to_team(
        &self,
        team_id: i64,
        body: &AddUserToTeamRequest,
    ) -> Result<Value> {
        self.post(&format!("/teams/{team_id}/addUser"), body).await
    }

    pub async fn remove_user_from_team(
        &self,
        team_id: i64,
        body: &AddUserToTeamRequest,
    ) -> Result<Value> {
        self.post(&format!("/teams/{team_id}/removeUser"), body)
            .await
    }

    pub async fn link_resource_to_team(
        &self,
        team_id: i64,
        body: &LinkResourceToTeamRequest,
    ) -> Result<Value> {
        self.post(&format!("/teams/{team_id}/linkResource"), body)
            .await
    }

    pub async fn unlink_resource_from_team(
        &self,
        team_id: i64,
        body: &LinkResourceToTeamRequest,
    ) -> Result<Value> {
        self.post(&format!("/teams/{team_id}/unlinkResource"), body)
            .await
    }
}
