use crate::ZebedeeClient;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceData {
    pub request: String,
    #[serde(rename = "fastRequest")]
    pub fast_request: String,
    pub uri: String,
    #[serde(rename = "fastUri")]
    pub fast_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawalRequestsData {
    pub id: String,
    pub unit: String,
    pub amount: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "expiresAt")]
    pub expires_at: DateTime<Utc>,
    #[serde(rename = "internalId")]
    pub internal_id: String,
    pub description: String,
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    pub status: String,
    pub invoice: InvoiceData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllWithdrawalRequestsRes {
    pub message: Option<String>,
    pub data: Option<Vec<WithdrawalRequestsData>>,
    pub success: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetWithdrawalRequestsRes {
    pub data: Option<WithdrawalRequestsData>,
    pub message: Option<String>,
    pub success: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostWithdrawalRequestsRes {
    pub success: Option<bool>,
    pub data: Option<WithdrawalRequestsData>,
    pub message: Option<String>,
}

/// Use this struct to create a well crafted json body for withdrawal requests
#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawalReqest {
    #[serde(rename = "expiresIn")]
    pub expires_in: u32,
    pub amount: String,
    pub description: String,
    #[serde(rename = "internalId")]
    pub internal_id: String,
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
}

impl Default for WithdrawalReqest {
    fn default() -> WithdrawalReqest {
        WithdrawalReqest {
            expires_in: 300,
            amount: String::from("0"),
            description: String::from("using zebedee rust sdk"),
            internal_id: String::from(""),
            callback_url: String::from(""),
        }
    }
}

pub async fn create_withdrawal_request(
    client: ZebedeeClient,
    withdrawal_request: WithdrawalReqest,
) -> Result<PostWithdrawalRequestsRes, anyhow::Error> {
    let resp = client
        .reqw_cli
        .post(format!("{}/v0/withdrawal-requests", client.domain))
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .json(&withdrawal_request)
        .send()
        .await?;

    let status_code = resp.status();

    let resp_text = resp.text().await?;

    match status_code {
        reqwest::StatusCode::OK => (), //dbg!("OK status:"),
        s => {
            return Err(anyhow::anyhow!(
                "Error: status {}, message: {}",
                s,
                resp_text
            ));
        }
    };

    let resp_serialized = serde_json::from_str(&resp_text);

    let resp_seralized_2: PostWithdrawalRequestsRes = match resp_serialized {
        Ok(c) => c,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Was given a good status, but something failed when parsing to json\nserde parse error: {}, \ntext from API: {}\n status code: {}",
                e,
                resp_text,
                status_code
            ))
        }
    };

    Ok(resp_seralized_2)
}

pub async fn get_withdrawal_requests(
    client: ZebedeeClient,
) -> Result<AllWithdrawalRequestsRes, anyhow::Error> {
    let resp = client
        .reqw_cli
        .get(format!("{}/v0/withdrawal-requests", client.domain))
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .send()
        .await?;

    let status_code = resp.status();
    let resp_text = resp.text().await?;

    match status_code {
        reqwest::StatusCode::OK => (), //dbg!("OK status:"),
        s => {
            return Err(anyhow::anyhow!(
                "Error: status {}, message: {}",
                s,
                resp_text
            ));
        }
    };

    let resp_serialized = serde_json::from_str(&resp_text);

    let resp_seralized_2: AllWithdrawalRequestsRes = match resp_serialized {
        Ok(c) => c,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Was given a good status, but something failed when parsing to json\nserde parse error: {}, \ntext from API: {}\n status code: {}",
                e,
                resp_text,
                status_code
            ))
        }
    };

    Ok(resp_seralized_2)
}

pub async fn get_withdrawal_request(
    client: ZebedeeClient,
    withdrawal_id: String,
) -> Result<GetWithdrawalRequestsRes, anyhow::Error> {
    let url = format!("{}/v0/withdrawal-requests/{}", client.domain, withdrawal_id);
    let resp = client
        .reqw_cli
        .get(&url)
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .send()
        .await?;

    let status_code = resp.status();

    let resp_text = resp.text().await?;

    match status_code {
        reqwest::StatusCode::OK => (), //dbg!("OK status:"),
        s => {
            return Err(anyhow::anyhow!(
                "Error: status {}, message: {}, withdrawal_id: {}, url: {}",
                s,
                resp_text,
                withdrawal_id,
                &url,
            ));
        }
    };

    let resp_serialized = serde_json::from_str(&resp_text);

    let resp_seralized_2: GetWithdrawalRequestsRes = match resp_serialized {
        Ok(c) => c,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Was given a good status, but something failed when parsing to json\nserde parse error: {}, \ntext from API: {}\nstatus code: {}\nwithdrawal_requests_id: {}\n url: {}",
                e,
                resp_text,
                status_code,
                withdrawal_id,
                &url,
            ))
        }
    };

    Ok(resp_seralized_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_create_withdrawal_request() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zbdenv: String =
            env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
        let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

        let withdrawal_request = WithdrawalReqest {
            amount: String::from("10000"),
            ..Default::default()
        };

        let r = create_withdrawal_request(zebedee_client, withdrawal_request)
            .await
            .unwrap();
        assert!(r.success.unwrap());
    }
    #[tokio::test]
    async fn test_get_withdrawal_requests() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zbdenv: String =
            env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
        let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

        let r = get_withdrawal_requests(zebedee_client).await.unwrap();
        assert!(r.success.unwrap());
    }
    #[tokio::test]
    async fn test_get_withdrawal_request() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zbdenv: String =
            env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
        let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

        let withdrawal_request = WithdrawalReqest {
            amount: String::from("10000"),
            ..Default::default()
        };

        let r = create_withdrawal_request(zebedee_client.clone(), withdrawal_request)
            .await
            .unwrap();
        let r2 = get_withdrawal_request(zebedee_client, r.data.unwrap().id)
            .await
            .unwrap();
        assert!(r2.success.unwrap());
    }
}
