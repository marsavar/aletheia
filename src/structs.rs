use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::BTreeMap;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: Option<String>,
    pub response: Option<SearchResponse>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub status: Option<String>,
    pub user_tier: Option<String>,
    pub total: Option<u32>,
    pub start_index: Option<u32>,
    pub page_size: Option<u32>,
    pub current_page: Option<u32>,
    pub pages: Option<u32>,
    pub order_by: Option<String>,
    pub results: Option<Vec<SearchResult>>,
    pub message: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    pub byline: Option<String>,
    pub short_url: Option<String>,
    pub trail_text: Option<String>,
    pub headline: Option<String>,
    pub body: Option<String>,
    pub last_modified: Option<chrono::DateTime<Utc>>,
    pub has_story_package: Option<String>,
    pub score: Option<String>,
    pub standfirst: Option<String>,
    pub show_in_related_content: Option<String>,
    pub thumbnail: Option<String>,
    pub wordcount: Option<String>,
    pub commentable: Option<String>,
    pub is_premoderated: Option<String>,
    pub allow_ugc: Option<String>,
    pub publication: Option<String>,
    pub internal_page_code: Option<String>,
    pub production_office: Option<String>,
    pub should_hide_adverts: Option<String>,
    pub live_blogging_now: Option<String>,
    pub comment_close_date: Option<chrono::DateTime<Utc>>,
    pub star_rating: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags {
    pub id: String,
    pub r#type: String,
    pub web_title: String,
    pub web_url: String,
    pub api_url: String,
    pub references: Vec<Reference>,
    pub bio: Option<String>,
    pub byline_image_url: Option<String>,
    pub byline_large_image_url: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email_address: Option<String>,
    pub twitter_handle: Option<String>,
    pub section_id: Option<String>,
    pub section_name: Option<String>,
    pub description: Option<String>,
    pub paid_content_type: Option<String>,
    pub paid_content_type_campaign_colour: Option<String>,
    pub rcs_id: Option<String>,
    pub r2_contributor_id: Option<String>,
    pub tag_categories: Option<Vec<String>>,
    pub entity_ids: Option<Vec<String>>,
    pub campaign_information_type: Option<String>,
    pub internal_name: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub id: String,
    pub r#type: String,
    pub section_id: String,
    pub section_name: String,
    pub web_publication_date: chrono::DateTime<Utc>,
    pub web_title: String,
    pub web_url: String,
    pub api_url: String,
    pub is_hosted: bool,
    pub pillar_id: Option<String>,
    pub pillar_name: Option<String>,
    pub fields: Option<Fields>,
    pub tags: Option<Vec<Tags>>,
    pub section: Option<Section>,
    pub blocks: Option<Blocks>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    pub id: String,
    pub r#type: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    pub id: String,
    pub web_title: String,
    pub web_url: String,
    pub api_url: String,
    pub editions: Vec<Edition>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edition {
    pub id: String,
    pub web_title: String,
    pub web_url: String,
    pub api_url: String,
    pub code: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blocks {
    pub main: Option<Block>,
    pub body: Option<Vec<Block>>,
    pub total_body_blocks: Option<i32>,
    pub requested_body_blocks: Option<BTreeMap<String, Vec<Block>>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub id: String,
    pub body_html: String,
    pub body_text_summary: String,
    pub title: Option<String>,
    pub attributes: BlockAttributes,
    pub published: bool,
    pub created_date: Option<serde_json::Value>,
    pub first_published_date: Option<serde_json::Value>,
    pub published_date: Option<serde_json::Value>,
    pub last_modified_date: Option<serde_json::Value>,
    pub contributors: Vec<String>,
    pub created_by: Option<User>,
    pub last_modified_by: Option<User>,
    pub elements: Vec<BlockElement>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockElement {
    pub r#type: String,
    pub assets: Vec<Asset>,
    pub text_type_data: Option<TextElementFields>,
    pub video_type_data: Option<VideoElementFields>,
    pub tweet_type_data: Option<TweetElementFields>,
    pub image_type_data: Option<ImageElementFields>,
    pub audio_type_data: Option<AudioElementFields>,
    pub pullquote_type_data: Option<PullquoteElementFields>,
    pub interactive_type_data: Option<InteractiveElementFields>,
    pub map_type_data: Option<StandardElementFields>,
    pub document_type_data: Option<StandardElementFields>,
    pub table_type_data: Option<StandardElementFields>,
    pub witness_type_data: Option<WitnessElementFields>,
    pub rich_link_type_data: Option<RichLinkElementFields>,
    pub membership_type_data: Option<MembershipElementFields>,
    pub embed_type_data: Option<EmbedElementFields>,
    pub instagram_type_data: Option<InstagramElementFields>,
    pub comment_type_data: Option<CommentElementFields>,
    pub vine_type_data: Option<VineElementFields>,
    pub content_atom_type_data: Option<ContentAtomElementFields>,
    pub tracking: Option<Box<EmbedTracking>>,
    pub code_type_data: Option<CodeElementFields>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextElementFields {
    pub html: Option<String>,
    pub role: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoElementFields {
    pub url: Option<String>,
    pub description: Option<String>,
    pub title: Option<String>,
    pub html: Option<String>,
    pub source: Option<String>,
    pub credit: Option<String>,
    pub caption: Option<String>,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub duration: Option<i32>,
    pub content_auth_system: Option<String>,
    pub embeddable: Option<String>,
    pub is_inappropriate_for_adverts: Option<bool>,
    pub media_id: Option<String>,
    pub still_image_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub short_url: Option<String>,
    pub role: Option<String>,
    pub original_url: Option<String>,
    pub source_domain: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TweetElementFields {
    pub source: Option<String>,
    pub url: Option<String>,
    pub id: Option<String>,
    pub html: Option<String>,
    pub original_url: Option<String>,
    pub role: Option<String>,
    pub source_domain: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageElementFields {
    pub caption: Option<String>,
    pub copyright: Option<String>,
    pub display_credit: Option<bool>,
    pub credit: Option<String>,
    pub source: Option<String>,
    pub photographer: Option<String>,
    pub alt: Option<String>,
    pub media_id: Option<String>,
    pub media_api_uri: Option<String>,
    pub picdar_urn: Option<String>,
    pub suppliers_reference: Option<String>,
    pub image_type: Option<String>,
    pub comment: Option<String>,
    pub role: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioElementFields {
    pub html: Option<String>,
    pub source: Option<String>,
    pub description: Option<String>,
    pub title: Option<String>,
    pub credit: Option<String>,
    pub caption: Option<String>,
    pub duration_minutes: Option<i32>,
    pub duration_seconds: Option<i32>,
    pub clean: Option<bool>,
    pub explicit: Option<bool>,
    pub role: Option<String>,
    pub source_domain: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullquoteElementFields {
    pub html: Option<String>,
    pub attribution: Option<String>,
    pub role: Option<String>,
    pub source: Option<String>,
    pub source_domain: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveElementFields {
    pub url: Option<String>,
    pub original_url: Option<String>,
    pub source: Option<String>,
    pub caption: Option<String>,
    pub alt: Option<String>,
    pub script_url: Option<String>,
    pub html: Option<String>,
    pub script_name: Option<String>,
    pub iframe_url: Option<String>,
    pub role: Option<String>,
    pub is_mandatory: Option<bool>,
    pub source_domain: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandardElementFields {
    pub url: Option<String>,
    pub original_url: Option<String>,
    pub source: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub credit: Option<String>,
    pub caption: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub html: Option<String>,
    pub role: Option<String>,
    pub is_mandatory: Option<bool>,
    pub source_domain: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WitnessElementFields {
    pub url: Option<String>,
    pub original_url: Option<String>,
    pub witness_embed_type: Option<String>,
    pub media_id: Option<String>,
    pub source: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub author_name: Option<String>,
    pub author_username: Option<String>,
    pub author_witness_profile_url: Option<String>,
    pub author_guardian_profile_url: Option<String>,
    pub caption: Option<String>,
    pub alt: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub html: Option<String>,
    pub api_url: Option<String>,
    pub photographer: Option<String>,
    pub date_created: Option<chrono::DateTime<Utc>>,
    pub youtube_url: Option<String>,
    pub youtube_source: Option<String>,
    pub youtube_title: Option<String>,
    pub youtube_description: Option<String>,
    pub youtube_author_name: Option<String>,
    pub youtube_html: Option<String>,
    pub role: Option<String>,
    pub source_domain: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichLinkElementFields {
    pub url: Option<String>,
    pub original_url: Option<String>,
    pub link_text: Option<String>,
    pub link_prefix: Option<String>,
    pub role: Option<String>,
    pub sponsorship: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MembershipElementFields {
    pub original_url: Option<String>,
    pub link_text: Option<String>,
    pub link_prefix: Option<String>,
    pub title: Option<String>,
    pub venue: Option<String>,
    pub location: Option<String>,
    pub identifier: Option<String>,
    pub image: Option<String>,
    pub price: Option<String>,
    pub start: Option<chrono::DateTime<Utc>>,
    pub end: Option<chrono::DateTime<Utc>>,
    pub role: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbedElementFields {
    pub html: Option<String>,
    pub safe_embed_code: Option<bool>,
    pub alt: Option<String>,
    pub is_mandatory: Option<bool>,
    pub role: Option<String>,
    pub source: Option<String>,
    pub source_domain: Option<String>,
    pub caption: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstagramElementFields {
    pub original_url: String,
    pub title: String,
    pub source: String,
    pub author_url: String,
    pub author_username: String,
    pub html: Option<String>,
    pub width: Option<i32>,
    pub alt: Option<String>,
    pub caption: Option<String>,
    pub role: Option<String>,
    pub source_domain: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentElementFields {
    pub source: Option<String>,
    pub discussion_key: Option<String>,
    pub comment_url: Option<String>,
    pub original_url: Option<String>,
    pub source_url: Option<String>,
    pub discussion_url: Option<String>,
    pub author_url: Option<String>,
    pub html: Option<String>,
    pub author_name: Option<String>,
    pub comment_id: Option<i32>,
    pub role: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VineElementFields {
    pub original_url: String,
    pub title: String,
    pub source: String,
    pub author_url: String,
    pub author_username: String,
    pub html: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub alt: Option<String>,
    pub caption: Option<String>,
    pub role: Option<String>,
    pub source_domain: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentAtomElementFields {
    pub atom_id: String,
    pub atom_type: String,
    pub role: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbedTracking {
    pub tracks: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeElementFields {
    pub html: String,
    pub language: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub r#type: String,
    pub mime_type: Option<String>,
    pub file: Option<String>,
    pub type_data: Option<AssetFields>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetFields {
    pub aspect_ratio: Option<String>,
    pub alt_text: Option<String>,
    pub is_inappropriate_for_adverts: Option<bool>,
    pub caption: Option<String>,
    pub credit: Option<String>,
    pub embeddable: Option<bool>,
    pub photographer: Option<String>,
    pub source: Option<String>,
    pub still_image_url: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub name: Option<String>,
    pub secure_file: Option<String>,
    pub is_master: Option<bool>,
    pub size_in_bytes: Option<i64>,
    pub duration_minutes: Option<i32>,
    pub duration_seconds: Option<i32>,
    pub display_credit: Option<bool>,
    pub thumbnail_url: Option<String>,
    pub role: Option<String>,
    pub media_id: Option<String>,
    pub iframe_url: Option<String>,
    pub script_name: Option<String>,
    pub script_url: Option<String>,
    pub block_ads: Option<bool>,
    pub html: Option<String>,
    pub embed_type: Option<String>,
    pub explicit: Option<bool>,
    pub clean: Option<bool>,
    pub thumbnail_image_url: Option<String>,
    pub link_text: Option<String>,
    pub link_prefix: Option<String>,
    pub short_url: Option<String>,
    pub image_type: Option<String>,
    pub suppliers_reference: Option<String>,
    pub media_api_uri: Option<String>,
    pub copyright: Option<String>,
    pub mime_type: Option<String>,
    pub url: Option<String>,
    pub original_url: Option<String>,
    pub id: Option<String>,
    pub attribution: Option<String>,
    pub description: Option<String>,
    pub title: Option<String>,
    pub content_auth_system: Option<String>,
    pub alt: Option<String>,
    pub picdar_urn: Option<String>,
    pub comment: Option<String>,
    pub witness_embed_type: Option<String>,
    pub author_name: Option<String>,
    pub author_username: Option<String>,
    pub author_witness_profile_url: Option<String>,
    pub author_guardian_profile_url: Option<String>,
    pub api_url: Option<String>,
    pub date_created: Option<serde_json::Value>,
    pub youtube_url: Option<String>,
    pub youtube_source: Option<String>,
    pub youtube_title: Option<String>,
    pub youtube_description: Option<String>,
    pub youtube_author_name: Option<String>,
    pub youtube_html: Option<String>,
    pub venue: Option<String>,
    pub location: Option<String>,
    pub identifier: Option<String>,
    pub price: Option<String>,
    pub start: Option<chrono::DateTime<Utc>>,
    pub end: Option<chrono::DateTime<Utc>>,
    pub safe_embed_code: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapiDateTime {
    pub date_time: i64,
    pub iso8601: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockAttributes {
    key_event: Option<bool>,
    summary: Option<bool>,
    title: Option<String>,
    pinned: Option<bool>,
    membership_placeholder: Option<MembershipPlaceholder>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MembershipPlaceholder {
    pub campaign_code: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}
