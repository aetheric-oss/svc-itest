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
use svc_storage_client_grpc::prelude::*;
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
    rest_debug!("(query_vertiports) entry.");

    let schedule: Option<String> = Some(
        "\
        DTSTART:20221020T180000Z;DURATION:PT14H
        RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR
        DTSTART:20221022T000000Z;DURATION:PT24H
        RRULE:FREQ=WEEKLY;BYDAY=SA,SU"
            .to_string(),
    );

    let points: Vec<GeoPoint> = payload
        .vertices
        .iter()
        .map(|vx| GeoPoint {
            latitude: vx.0,
            longitude: vx.1,
        })
        .collect();

    let data = vertiport::Data {
        name: payload.label.clone(),
        description: "".to_string(),
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

    for pad in &payload.pads {
        let data = vertipad::Data {
            vertiport_id: vertiport_id.clone(),
            name: pad.label.clone(),
            geo_location: Some(GeoPoint {
                latitude: pad.centroid_latitude,
                longitude: pad.centroid_longitude,
            }),
            enabled: true,
            occupied: false,
            schedule: schedule.clone(),
            created_at: None,
            updated_at: None,
        };

        grpc_clients
            .storage
            .vertipad
            .insert(data)
            .await
            .map_err(|e| {
                rest_error!("(add_vertiport) Error: {}.", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    }

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

// /// Example REST API function
// #[utoipa::path(
//     delete,
//     path = "/demo/vertiport",
//     tag = "svc-itest",
//     request_body = String,
//     responses(
//         (status = 200, description = "Request successful.", body = String),
//         (status = 500, description = "Request unsuccessful."),
//     )
// )]
// pub async fn delete_vertiport(
//     Extension(mut grpc_clients): Extension<GrpcClients>,
//     Json(payload): Json<String>,
// ) -> Result<Json<String>, StatusCode> {
//     rest_debug!("(query_vertiports) entry.");

//     const schedule = Some("\
//         DTSTART:20221020T180000Z;DURATION:PT14H
//         RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR
//         DTSTART:20221022T000000Z;DURATION:PT24H
//         RRULE:FREQ=WEEKLY;BYDAY=SA,SU".to_string()
//     );

//     let data = vertiport::Data {
//         name: payload.label.clone(),
//         description: "".to_string(),
//         geo_location: None,
//         schedule,
//         created_at: None,
//         updated_at: None,
//     }

//     let vertiport_id = grpc_clients
//         .storage
//         .vertiport
//         .insert(data)
//         .await
//         .map_err(|e| {
//             rest_error!("(add_vertiport) Error: {}.", e);
//             StatusCode::INTERNAL_SERVER_ERROR
//         })?
//         .into_inner()
//         .id;

//     for pad in &payload.pads {
//         let data = vertipad::Data {
//             vertiport_id: vertiport_id.clone(),
//             name: pad.label.clone(),
//             geo_location: Some(GeoPoint {
//                 latitude: pad.centroid_latitude,
//                 longitude: pad.centroid_longitude,
//             }),
//             enabled: true,
//             occupied: false,
//             schedule: schedule.clone(),
//             created_at: None,
//             updated_at: None,
//         };

//         grpc_clients
//             .storage
//             .vertipad
//             .insert(data)
//             .await
//             .map_err(|e| {
//                 rest_error!("(add_vertiport) Error: {}.", e);
//                 StatusCode::INTERNAL_SERVER_ERROR
//             })?;
//     }

//     Ok(Json(format!("{}!", vertiport_id)))
// }

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

    let schedule: Option<String> = Some(
        "\
        DTSTART:20221020T180000Z;DURATION:PT14H
        RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR
        DTSTART:20221022T000000Z;DURATION:PT24H
        RRULE:FREQ=WEEKLY;BYDAY=SA,SU"
            .to_string(),
    );

    let aircraft_id = grpc_clients
        .storage
        .vehicle
        .insert(vehicle::Data {
            vehicle_model_id: Uuid::new_v4().to_string(),
            registration_number: payload.registration_number.clone(),
            serial_number: Uuid::new_v4().to_string(),
            description: Some(payload.nickname.clone()),
            hangar_id: Some(payload.hangar_id),
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
