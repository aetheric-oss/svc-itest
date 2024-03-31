//! Rest API implementations
/// openapi generated rest types
pub mod rest_types {
    include!("../../../openapi/types.rs");
}

pub use rest_types::*;

use crate::grpc::client::GrpcClients;
use axum::{extract::Extension, Json};
use chrono::Utc;
use hyper::StatusCode;
use svc_gis_client_grpc::client::{Coordinates, UpdateVertiportsRequest, Vertiport};
use svc_gis_client_grpc::prelude::GisServiceClient;
use svc_storage_client_grpc::prelude::{user::AuthMethod, *};
use uuid::Uuid;

/// Provides a way to tell a caller if the service is healthy.
/// Checks dependencies, making sure all connections can be made.
#[utoipa::path(
    get,
    path = "/health",
    tag = "svc-itest",
    responses(
        (status = 200, description = "Service is healthy, all dependencies running."),
        (status = 503, description = "Service is unhealthy, one or more dependencies unavailable.")
    )
)]
pub async fn health_check(
    Extension(grpc_clients): Extension<GrpcClients>,
) -> Result<(), StatusCode> {
    rest_debug!("(health_check) entry.");

    let mut ok = true;

    if grpc_clients
        .storage
        .vertiport
        .is_ready(ReadyRequest {})
        .await
        .is_err()
    {
        let error_msg = "svc-storage vertiport unavailable.".to_string();
        rest_error!("(health_check) {}.", &error_msg);
        ok = false;
    }

    match ok {
        true => {
            rest_debug!("(health_check) healthy, all dependencies running.");
            Ok(())
        }
        false => {
            rest_error!("(health_check) unhealthy, 1+ dependencies down.");
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

/// Add a vertiport to storage and GIS
#[utoipa::path(
    put,
    path = "/demo/vertiport",
    tag = "svc-itest",
    request_body = AddVertiportRequest,
    responses(
        (status = 200, description = "Request successful.", body = String),
        (status = 500, description = "Request unsuccessful."),
    )
)]
pub async fn add_vertiport(
    Extension(grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<AddVertiportRequest>,
) -> Result<Json<String>, StatusCode> {
    rest_debug!("(add_vertiport) entry.");

    let schedule = "DTSTART:20221020T180000Z;DURATION:PT24H
    RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR,SA,SU"
        .to_string()
        .replace(' ', "");
    let schedule = Some(schedule);
    let points: Vec<GeoPoint> = payload
        .vertices
        .iter()
        .map(|vx| GeoPoint {
            latitude: vx.0,
            longitude: vx.1,
            altitude: payload.altitude,
        })
        .collect();

    let data = vertiport::Data {
        name: payload.label.clone(),
        description: format!("A vertiport named '{}'", payload.label),
        geo_location: Some(GeoPolygon {
            exterior: Some(GeoLineString { points }),
            interiors: vec![],
        }),
        schedule: schedule.clone(),
        created_at: None,
        updated_at: None,
    };

    let vertiport_id = grpc_clients
        .storage
        .vertiport
        .insert(data)
        .await
        .map_err(|e| {
            rest_error!("(add_vertiport) Error: {}.", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_inner()
        .object
        .ok_or_else(|| {
            rest_error!("(add_vertiport) Error: no object returned.");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .id;

    let vertiports = vec![Vertiport {
        identifier: vertiport_id.clone(),
        label: Some(payload.label.clone()),
        vertices: payload
            .vertices
            .iter()
            .map(|vx| Coordinates {
                latitude: vx.0,
                longitude: vx.1,
            })
            .collect(),
        altitude_meters: 0.0,
        timestamp_network: Some(Utc::now().into()),
    }];

    let request = UpdateVertiportsRequest { vertiports };

    grpc_clients
        .gis
        .update_vertiports(request)
        .await
        .map_err(|e| {
            rest_error!("(add_vertiport) Error: {}.", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(vertiport_id))
}

/// Add a vertiport to storage and GIS
#[utoipa::path(
    put,
    path = "/demo/vertipad",
    tag = "svc-itest",
    request_body = AddVertipadRequest,
    responses(
        (status = 200, description = "Request successful.", body = String),
        (status = 500, description = "Request unsuccessful."),
    )
)]
pub async fn add_vertipad(
    Extension(grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<AddVertipadRequest>,
) -> Result<Json<String>, StatusCode> {
    rest_debug!("(add_vertipad) entry.");

    let schedule = "DTSTART:20221020T180000Z;DURATION:PT24H
    RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR,SA,SU"
        .to_string()
        .replace(' ', "");
    let schedule = Some(schedule);
    let data = vertipad::Data {
        vertiport_id: payload.vertiport_id.clone(),
        name: payload.label.clone(),
        geo_location: Some(GeoPoint {
            latitude: payload.latitude,
            longitude: payload.longitude,
            altitude: payload.altitude,
        }),
        enabled: true,
        occupied: false,
        schedule: schedule.clone(),
        created_at: None,
        updated_at: None,
    };

    let vertipad_id = grpc_clients
        .storage
        .vertipad
        .insert(data)
        .await
        .map_err(|e| {
            rest_error!("(add_vertipad) Error: {}.", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_inner()
        .object
        .ok_or_else(|| {
            rest_error!("(add_vertipad) Error: no object returned.");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .id;

    Ok(Json(vertipad_id))
}

/// Add aircraft to storage
#[utoipa::path(
    put,
    path = "/demo/aircraft",
    tag = "svc-itest",
    request_body = AddAircraftRequest,
    responses(
        (status = 200, description = "Request successful.", body = String),
        (status = 500, description = "Request unsuccessful."),
    )
)]
pub async fn add_aircraft(
    Extension(grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<AddAircraftRequest>,
) -> Result<Json<String>, StatusCode> {
    rest_debug!("(add_aircraft) entry.");

    let schedule = "DTSTART:20221020T180000Z;DURATION:PT24H
    RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR,SA,SU"
        .to_string()
        .replace(' ', "");
    let schedule = Some(schedule);

    let aircraft_id = grpc_clients
        .storage
        .vehicle
        .insert(vehicle::Data {
            vehicle_model_id: Uuid::new_v4().to_string(),
            registration_number: payload.registration_number.clone(),
            serial_number: Uuid::new_v4().to_string(),
            description: Some(payload.nickname.clone()),
            hangar_id: Some(payload.hangar_id),
            hangar_bay_id: Some(payload.hangar_bay_id),
            schedule,
            ..Default::default()
        })
        .await
        .map_err(|e| {
            rest_error!("(add_aircraft) Error: {}.", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_inner()
        .object
        .ok_or_else(|| {
            rest_error!("(add_aircraft) Error: no object returned.");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .id;

    Ok(Json(aircraft_id))
}

/// Add user to storage
#[utoipa::path(
    put,
    path = "/demo/user",
    tag = "svc-itest",
    request_body = AddUserRequest,
    responses(
        (status = 200, description = "Request successful.", body = String),
        (status = 500, description = "Request unsuccessful."),
    )
)]
pub async fn add_user(
    Extension(grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<AddUserRequest>,
) -> Result<Json<String>, StatusCode> {
    rest_debug!("(add_user) entry.");

    let auth_method = AuthMethod::Local as i32;
    let user_id = grpc_clients
        .storage
        .user
        .insert(user::Data {
            auth_method,
            display_name: payload.display_name.clone(),
            email: payload.email.clone(),
        })
        .await
        .map_err(|e| {
            rest_error!("(add_user) Error: {}.", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_inner()
        .object
        .ok_or_else(|| {
            rest_error!("(add_user) Error: no object returned.");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .id;

    Ok(Json(user_id))
}

impl TryFrom<AddScannerRequest> for scanner::Data {
    type Error = String;

    fn try_from(value: AddScannerRequest) -> Result<Self, Self::Error> {
        let scanner_type = match value.scanner_type.as_str() {
            "underbelly" => scanner::ScannerType::Underbelly as i32,
            "mobile" => scanner::ScannerType::Mobile as i32,
            "locker" => scanner::ScannerType::Locker as i32,
            "facility" => scanner::ScannerType::Facility as i32,
            _ => return Err("Invalid scanner type.".to_string()),
        };

        Ok(scanner::Data {
            organization_id: value.organization_id,
            scanner_type,
            scanner_status: scanner::ScannerStatus::Active as i32,
        })
    }
}

/// Add user to storage
#[utoipa::path(
    put,
    path = "/demo/scanner",
    tag = "svc-itest",
    request_body = AddScannerRequest,
    responses(
        (status = 200, description = "Request successful.", body = String),
        (status = 500, description = "Request unsuccessful."),
    )
)]
pub async fn add_scanner(
    Extension(grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<AddScannerRequest>,
) -> Result<Json<String>, StatusCode> {
    rest_debug!("(add_scanner) entry.");

    let data: scanner::Data = payload.try_into().map_err(|e| {
        rest_error!("(add_scanner) Error: {}.", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let scanner_id = grpc_clients
        .storage
        .scanner
        .insert(data)
        .await
        .map_err(|e| {
            rest_error!("(add_user) Error: {}.", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_inner()
        .object
        .ok_or_else(|| {
            rest_error!("(add_user) Error: no object returned.");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .id;

    Ok(Json(scanner_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check_success() {
        crate::get_log_handle().await;
        ut_info!("(test_health_check_success) Start.");

        // Mock the GrpcClients extension
        let config = crate::Config::try_from_env().unwrap_or_default();
        let grpc_clients = GrpcClients::default(config); // Replace with your own mock implementation

        // Call the health_check function
        let result = health_check(Extension(grpc_clients)).await;

        // Assert the expected result
        println!("{:?}", result);
        assert!(result.is_ok());

        ut_info!("(test_health_check_success) Success.");
    }
}
