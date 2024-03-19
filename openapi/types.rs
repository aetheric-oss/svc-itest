/// Types used for REST communication with the svc-cargo server

use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// Information needed to build a vertipad
#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
#[derive(ToSchema, IntoParams)]
pub struct Vertipad {
    /// The latitude of the pad
    pub centroid_latitude: f64,
    
    /// The longitude of the pad
    pub centroid_longitude: f64,

    /// The radius of the pad in meters
    pub radius_meters: f32,

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

    /// The bounding polygon of this vertiport
    pub vertices: Vec<(f64, f64)>,

    /// The pads at this vertiport
    pub pads: Vec<Vertipad>
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
    pub hangar_id: String
}
