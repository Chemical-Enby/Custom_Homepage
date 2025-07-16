use chrono::DateTime;
use chrono::offset::Utc;
use crate::image_response::profile_image::ProfileImage;
use crate::image_response::social::Social;
use crate::image_response::user_links::UserLinks;

pub struct User {
    id: String,
    updated_at: DateTime<Utc>,
    username: String,
    name: String,
    first_name: String,
    last_name: String,
    twitter_username: String,
    portfolio_url: String,
    bio: String,
    location: String,
    links: UserLinks,
    profile_image: ProfileImage,
    instagram_username: String,
    total_collections: u32,
    total_likes: u32,
    total_photos: u32,
    total_promoted_photos: u32,
    total_illustrations: u32,
    total_promoted_illustrations: u32,
    accepted_tos: bool,
    for_hire: bool,
    social: Social
}