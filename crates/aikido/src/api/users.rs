use crate::client::AikidoClient;
use crate::error::Result;
use crate::models::*;
use serde_json::Value;

impl AikidoClient {
    pub async fn list_users(&self) -> Result<Vec<User>> {
        self.get("/users").await
    }

    pub async fn get_user(&self, user_id: i64) -> Result<User> {
        self.get(&format!("/users/{user_id}")).await
    }

    pub async fn update_user_rights(
        &self,
        user_id: i64,
        body: &UpdateUserRightsRequest,
    ) -> Result<Value> {
        self.put(&format!("/users/{user_id}/rights"), body).await
    }
}
