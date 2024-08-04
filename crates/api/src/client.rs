use std::time::Duration;

use cynic::{http::CynicReqwestError, GraphQlResponse};

const ENDPOINT: &str = "https://api.linear.app/graphql";

pub struct Client {
    client: reqwest::Client,
    endpoint: url::Url,
    token: String,
}

impl Client {
    pub fn new(client: reqwest::Client, token: String) -> Self {
        Self {
            client,
            endpoint: ENDPOINT.parse().unwrap(),
            token,
        }
    }

    pub fn new_default(token: String) -> Self {
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .build()
            .expect("could not construct http client");
        Self::new(client, token)
    }

    pub async fn run<ResponseData, Vars>(
        &self,
        operation: cynic::Operation<ResponseData, Vars>,
    ) -> Result<GraphQlResponse<ResponseData>, CynicReqwestError>
    where
        Vars: serde::Serialize + std::fmt::Debug,
        ResponseData: serde::de::DeserializeOwned + 'static,
    {
        let response = self
            .client
            .post(self.endpoint.clone())
            .header(reqwest::header::AUTHORIZATION, &self.token)
            .json(&operation)
            .send()
            .await
            .map_err(CynicReqwestError::ReqwestError)?;

        // if let Some(value) = response
        //     .headers()
        //     .get("X-RateLimit-Requests-Remaining")
        //     .and_then(|x| x.to_str().ok())
        // {
        // }

        let status = response.status();
        if !status.is_success() {
            let text = response.text().await;
            let text = match text {
                Ok(text) => text,
                Err(e) => return Err(CynicReqwestError::ReqwestError(e)),
            };

            let Ok(deserred) = serde_json::from_str(&text) else {
                let response = CynicReqwestError::ErrorResponse(status, text);
                return Err(response);
            };

            Ok(deserred)
        } else {
            let json = response.json().await;
            json.map_err(CynicReqwestError::ReqwestError)
        }
    }
}
