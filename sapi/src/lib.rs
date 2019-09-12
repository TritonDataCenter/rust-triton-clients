// Copyright 2019 Joyent, Inc.
use slog::Logger;
use std::time::Duration;

use reqwest::{Client, IntoUrl, Response};
// Use old-style Hyper headers until they put them back in.
use reqwest::hyper_011::header::{Accept, ContentType, Headers};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Debug)]
struct Services {
    metadata: Vec<ServiceData>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ServiceData {
   name: String,
//   service_name: String,
//   napi_log_level: String,
//   service_domain: String,
}

#[derive(Debug)]
pub struct SAPI {
    sapi_base_url: String,
    request_timeout: u64,
    client: Client,
    log: Logger,
}

impl SAPI {
    /// initialize SAPI client API
    pub fn new(
        sapi_base_url: &str,
        request_timeout: u64,
        log: Logger,
    ) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(request_timeout))
            .build()
            .unwrap();
        let sapi = SAPI {
                sapi_base_url: sapi_base_url.into(),
                request_timeout: request_timeout.into(),
                client,
                log: log.clone(),
        };
        sapi
    }

    pub fn get_zone_config(
        &self,
        uuid: &str
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/configs/{}", self.sapi_base_url.clone(), uuid);
        match self.get(&url)?.error_for_status() {
            Ok(mut resp) => {
                let v: Value = serde_json::from_str(&resp.text().unwrap())?;
                Ok(v)
            },
            Err(e) => Err(Box::new(e))
        }
    }

    /// List all services
    pub fn list_services(
        &self
    ) -> Result<Vec<ServiceData>, Box<dyn std::error::Error>> {

        let url = format!("{}", self.sapi_base_url.clone() + "/services");
        let sdata: Vec<ServiceData> = self.get(&url)?.json()?;
        Ok(sdata)
    }

    /// get service by uuid
    pub fn get_service(
        &self,
        uuid: &str
    ) -> Result<ServiceData, Box<dyn std::error::Error>> {
        let url = format!("{}", self.sapi_base_url.clone()
                            + "/service/{}" + uuid);
        let sdata: ServiceData = self.get(&url)?.json()?;
        Ok(sdata)
    }

    pub fn create_service(
        &self,
        name: &str,
        application_uuid: &str
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let body = json!({
            "name": name,
            "application_uuid": application_uuid
        });
        let url = format!("{}", self.sapi_base_url.clone() + "/services");
        self.post(&url, &body)
    }

    pub fn update_service(
        &self,
        service_uuid: &str,
        body: Value
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!("{}", self.sapi_base_url.clone()
                          + "/services/{}" + service_uuid);
        self.post(&url, &body)
    }

    pub fn delete_service(
        &self,
        service_uuid: &str
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!("{}", self.sapi_base_url.clone()
                          + "/services/{}" + service_uuid);
        self.delete(&url)
    }


    fn default_headers(&self) -> Headers {
        let mut headers = Headers::new();

        headers.set(ContentType::json());
        headers.set(Accept::json());
        headers
    }

    /// Generic get -- results deserialized by caller
    pub fn get<S>(
        &self,
        url: S
    ) -> Result<Response, Box<dyn std::error::Error>>
    where
        S: IntoUrl
    {
        match self.client.get(url).headers_011(self.default_headers()).send() {
            Ok(response) => Ok(response),
            Err(e) => Err(Box::new(e))
        }
    }

    /// Generic post
    fn post<S>(
        &self,
        url: S,
        body: &Value
    ) -> Result<Response, Box<dyn std::error::Error>>
    where
        S: IntoUrl,
    {

        let resp = self.client
            .post(url)
            .headers_011(self.default_headers())
            .json(&body)
            .send()?;
        Ok(resp)
    }

    /// Generic delete
    fn delete<S>(
        &self,
        url: S,
    ) -> Result<Response, Box<dyn std::error::Error>>
    where
        S: IntoUrl,
    {

        let resp = self.client
            .delete(url)
            .headers_011(self.default_headers())
            .send()?;
        Ok(resp)
    }
}

#[test]
fn test_services() {
    use slog::{error, info, o, Drain, Logger};
    use std::sync::Mutex;

    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let log = Logger::root(
        Mutex::new(slog_term::FullFormat::new(plain).build()).fuse(),
        o!("build-id" => "0.1.0"),
    );

    let client = SAPI::new("http://10.77.77.136", 60, log.clone());

    let s_uuid = String::from("e68592d3-5677-44ec-a5e8-cfd3652dd5be");
    let name = String::from("cheddar");
    match client.create_service(&name, &s_uuid.to_string()) {
        Ok(resp) => {
            assert_eq!(resp.status().is_success(), true);
        },
        Err(_e) => {
            assert!(false)
        }
    }

    match client.list_services() {
        Ok(list) => {
            assert_ne!(list.len(), 0);
        },
        Err(e) => {
            info!(log, "Error: {:?}", e);
            assert!(false)
        }
    }

    let zone_uuid = String::from("f8bf03e3-5636-4cc4-a939-bbca6b4547f0");
    match client.get_zone_config(&zone_uuid) {
        Ok(resp) => {
            info!(log, "config: {:?}", resp["manifests"]["BORAY_SERVER_PORT"]);
            assert_eq!(resp["manifests"].as_null(), None);
        },
        Err(e) => error!(log, "error: {:?}",  e)
    }
}

