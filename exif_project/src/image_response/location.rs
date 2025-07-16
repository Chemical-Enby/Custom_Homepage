use crate::image_response::coordinates::Coordinates;

pub struct Location {
    name: String,
    city: String,
    country: String,
    position: Coordinates
}