// Copyright 2020 Joyent, Inc.

use slog::{error, info, o, Drain, Logger};
use std::sync::Mutex;

fn main() {
    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let log = Logger::root(
        Mutex::new(slog_term::FullFormat::new(plain).build()).fuse(),
        o!("build-id" => "0.1.0"),
    );

    let client = sapi::SAPI::new("http://sapi.ruidc0.joyent.us", 60, log.clone());

    let sdc_app = client.get_application_by_name("sdc").expect("sdc_app");

    assert_eq!(sdc_app.len(), 1);

    let sdc_app_data = sdc_app[0].clone();
    let sdc_app_metadata = sdc_app_data.metadata.expect("app metadata");
    let app_admin_login = sdc_app_metadata["ufds_admin_login"].clone();

    assert_eq!(app_admin_login.as_str(), Some("admin"));

    let mut sapi_svc = client
        .get_service_by_name("sapi")
        .expect("get sapi service");

    assert_eq!(sapi_svc.len(), 1);

    let sapi_svc_data = sapi_svc.pop().expect("first sapi service");
    let sapi_metadata = sapi_svc_data.metadata.expect("metadata");
    let service_name = sapi_metadata["SERVICE_NAME"].clone();

    assert_eq!(service_name.as_str(), Some("sapi"));

    let services = client.list_services().expect("list services");
    dbg!(&services);

    let svc = &services[0];
    let svc_uuid = &svc.uuid;

    let instances = client
        .list_service_instances(svc_uuid)
        .expect("list service instances");
    let zone_uuid = &instances[0].uuid;

    match client.get_zone_config(&zone_uuid) {
        Ok(resp) => {
            info!(log, "config: {:#?}", resp);
        }
        Err(e) => error!(log, "error: {:?}", e),
    }
}
