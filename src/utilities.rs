use crate::ZebedeeClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBtcUsdRes {
    pub success: Option<bool>,
    pub data: Option<BtcUsdData>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BtcUsdData {
    #[serde(rename = "btcUsdPrice")]
    pub btc_usd_price: String,
    #[serde(rename = "btcUsdTimestamp")]
    pub btc_usd_timestamp: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetProdIpsRes {
    pub success: Option<bool>,
    pub data: Option<IpData>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpData {
    pub ips: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetIsSupportedRegionByIpRes {
    pub success: Option<bool>,
    pub data: Option<RegionIpData>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionIpData {
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    #[serde(rename = "isSupported")]
    pub is_supported: bool,
    #[serde(rename = "ipCountry")]
    pub ip_country: String,
    #[serde(rename = "ipRegion")]
    pub ip_region: String,
}

pub async fn get_is_supported_region_by_ip(
    client: ZebedeeClient,
    ip: String,
) -> Result<GetIsSupportedRegionByIpRes, anyhow::Error> {
    let url = format!("{}/v0/is-supported-region/{}", client.domain, ip);
    let resp = client
        .reqw_cli
        .get(&url)
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .send()
        .await?;

    let status_code = resp.status();
    let status_success = resp.status().is_success();
    let resp_text = resp.text().await?;

    if !status_success {
        return Err(anyhow::anyhow!(
            "Error: status {}, message: {}, url: {}",
            status_code,
            resp_text,
            url,
        ));
    }

    let resp_serialized = serde_json::from_str(&resp_text);

    let resp_seralized_2 = match resp_serialized {
        Ok(c) => c,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Was given a good status, but something failed when parsing to json\nserde parse error: {}, \ntext from API: {}\n status code: {}",
                e,
                resp_text,
                status_code,
            ))
        }
    };

    Ok(resp_seralized_2)
}

pub async fn get_prod_ips(client: ZebedeeClient) -> Result<GetProdIpsRes, anyhow::Error> {
    let url = format!("{}/v0/prod-ips", client.domain);
    let resp = client
        .reqw_cli
        .get(&url)
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .send()
        .await?;

    let status_code = resp.status();
    let status_success = resp.status().is_success();
    let resp_text = resp.text().await?;

    if !status_success {
        return Err(anyhow::anyhow!(
            "Error: status {}, message: {}, url: {}",
            status_code,
            resp_text,
            &url,
        ));
    }

    let resp_serialized = serde_json::from_str(&resp_text);

    let resp_seralized_2 = match resp_serialized {
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

pub async fn get_btc_usd(client: ZebedeeClient) -> Result<GetBtcUsdRes, anyhow::Error> {
    let url = format!("{}/v0/btcusd", client.domain);
    let resp = client
        .reqw_cli
        .get(&url)
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .send()
        .await?;

    let status_code = resp.status();
    let status_success = resp.status().is_success();
    let resp_text = resp.text().await?;

    if !status_success {
        return Err(anyhow::anyhow!(
            "Error: status {}, message: {}, url: {}",
            status_code,
            resp_text,
            &url,
        ));
    }

    let resp_serialized = serde_json::from_str(&resp_text);

    let resp_seralized_2 = match resp_serialized {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_get_is_supported_region_by_ip() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zbdenv: String =
            env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
        let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

        let ip = String::from("3.225.112.64");

        let r = get_is_supported_region_by_ip(zebedee_client, ip)
            .await
            .unwrap()
            .success;
        assert!(r.unwrap());
    }

    #[tokio::test]
    async fn test_get_prod_ips() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zbdenv: String =
            env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
        let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

        let r = get_prod_ips(zebedee_client).await.unwrap().success;
        assert!(r.unwrap());
    }

    #[tokio::test]
    async fn test_get_btc_usd() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zbdenv: String =
            env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
        let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();
        let r = get_btc_usd(zebedee_client).await.unwrap().success;
        assert!(r.unwrap());
    }
}
