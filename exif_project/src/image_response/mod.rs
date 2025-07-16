mod alternative_slug;
mod url;
mod photo_links;
mod user;
mod user_links;
mod profile_image;
mod social;
mod exif;
mod location;
mod coordinates;
mod meta;
mod tag;

use crate::image_response::{alternative_slug::AlternativeSlug,
                            url::Url,
                            photo_links::PhotoLinks,
                            user::User,
                            exif::Exif,
                            location::Location,
                            meta::Meta,
                            tag::Tag,
};

use chrono::{DateTime, Utc};


pub struct ImageResponse {
    id: String,
    slug: String,
    alternative_slug: AlternativeSlug,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    promoted_at: DateTime<Utc>, // TODO
    width: u32,
    height: u32,
    color: String,
    blur_hash: String,
    description: String,
    alt_description: String,
    breadcrumbs: Vec<String>, // TODO
    urls: Url,
    links: PhotoLinks,
    likes: u32,
    liked_by_user: bool,
    current_user_collections: Vec<String>, // TODO
    sponsorship: String, // TODO
    topic_submissions: String, // TODO
    asset_type: String,
    user: User,
    exif: Exif,
    location: Location,
    meta: Meta,
    public_domain: bool,
    tags: Vec<Tag>,
    views: u64,
    downloads: u32,
    topics: Vec<String>
}