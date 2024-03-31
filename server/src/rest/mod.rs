//! REST
//! provides server implementations for REST API

#[macro_use]
pub mod macros;
pub mod api;
pub mod server;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::add_vertiport,
        api::add_vertipad,
        api::add_aircraft,
        api::add_user,
        api::add_scanner
    ),
    components(
        schemas(
            api::rest_types::AddVertiportRequest,
            api::rest_types::AddVertipadRequest,
            api::rest_types::AddAircraftRequest,
            api::rest_types::AddUserRequest,
            api::rest_types::AddScannerRequest,
        )
    ),
    tags(
        (name = "svc-itest", description = "svc-itest REST API")
    )
)]
struct ApiDoc;

/// Create OpenAPI3 Specification File
pub fn generate_openapi_spec(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = ApiDoc::openapi()
        .to_pretty_json()
        .expect("(ERROR) unable to write openapi specification to json.");

    std::fs::write(target, output).expect("(ERROR) unable to write json string to file.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_openapi_spec_generation() {
        crate::get_log_handle().await;
        ut_info!("(test_openapi_spec_generation) Start.");

        assert!(generate_openapi_spec("/tmp/generate_openapi_spec.out").is_ok());

        ut_info!("(test_openapi_spec_generation) Success.");
    }
}
