/// Types used for REST communication with the svc-cargo server

use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// Information needed to build a vertipad
#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
#[derive(ToSchema, IntoParams)]
pub struct AddVertipadRequest {
    /// The ID of the vertiport
    pub vertiport_id: String,

    /// The latitude of the pad
    pub latitude: f64,
    
    /// The longitude of the pad
    pub longitude: f64,

    /// The altitude of the pad in meters
    pub altitude: f64,

    /// The informal label for this pad
    pub label: String
}

/// Information needed to add a vertiport
#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
#[derive(ToSchema, IntoParams)]
pub struct AddVertiportRequest {
    /// The label of the vertiport
    pub label: String,

    /// The address of the vertiport
    pub address: String,

    /// The bounding polygon of this vertiport
    pub vertices: Vec<(f64, f64)>,

    /// The starting altitude of the vertiport in meters
    pub altitude: f64
}

/// Information needed to add an aircraft
#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
#[derive(ToSchema, IntoParams)]
pub struct AddAircraftRequest {
    /// The nickname of the aircraft
    pub nickname: String,

    /// The registration number of the aircraft
    pub registration_number: String,

    /// The hangar ID
    pub hangar_id: String,

    /// The hangar bay ID
    pub hangar_bay_id: String
}

/// Information needed to build a vertipad
#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
#[derive(ToSchema, IntoParams)]
pub struct AddUserRequest {
    /// The display name of the user
    pub display_name: String,
    /// The email of the user
    pub email: String,
}

/// Information needed to build a vertipad
#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
#[derive(ToSchema, IntoParams)]
pub struct AddScannerRequest {
    /// The display name of the user
    pub organization_id: String,
    /// The email of the user
    pub scanner_type: String,
}
