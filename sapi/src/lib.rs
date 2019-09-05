use slog::Logger;
use std::time::Duration;

use reqwest::{Client, IntoUrl, Response};
// Use old-style Hyper headers until they put them back in.
use reqwest::hyper_011::header::{Accept, ContentType, Headers};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Services {
     metadata: Vec<ServiceData>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ServiceData {
   name: String,
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

    fn default_headers(&self) -> Headers {
        let mut headers = Headers::new();

        headers.set(ContentType::json());
        headers.set(Accept::json());
        headers
    }

    /// Generic get -- results deserialized by caller
    fn get<S>(
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

}


#[test]
fn test_services() {
    use slog::{info, o, Drain, Logger};
    use std::sync::Mutex;

    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let log = Logger::root(
        Mutex::new(slog_term::FullFormat::new(plain).build()).fuse(),
        o!("build-id" => "0.1.0"),
    );

    let client = SAPI::new("http://10.77.77.136", 60, log.clone());

    match client.list_services() {
        Ok(list) => {
            assert_ne!(list.len(), 0);
        },
        Err(e) => {
            info!(log, "Error: {:?}", e);
            assert!(false)
        }
    }
}
