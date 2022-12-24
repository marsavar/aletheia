//! Enum types that prevent passing illegal parameters to the
//! Guardian's content API.

use serde::Deserialize;
use strum_macros::Display;

#[derive(Display, Debug, Deserialize, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum OrderBy {
    Newest,
    Oldest,
    Relevance,
}

#[derive(Display, Debug, Deserialize, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum OrderDate {
    Published,
    NewspaperEdition,
    LastModified,
}

#[derive(Display, Debug, Deserialize, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum UseDate {
    Published,
    FirstPublication,
    NewspaperEdition,
    LastModified,
}

#[derive(Display, Debug, Deserialize, Eq, PartialEq)]
#[strum(serialize_all = "camelCase")]
pub enum Field {
    TrailText,
    Headline,
    ShowInRelatedContent,
    Body,
    BodyText,
    LastModified,
    HasStoryPackage,
    Score,
    Standfirst,
    ShortUrl,
    Byline,
    Thumbnail,
    Wordcount,
    Commentable,
    IsPremoderated,
    AllowUgc,
    Publication,
    InternalPageCode,
    ProductionOffice,
    ShouldHideAdverts,
    LiveBloggingNow,
    CommentCloseDate,
    StarRating,
    /// Override all fields
    All,
}

#[derive(Display, Debug, Deserialize, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum Tag {
    Blog,
    Contributor,
    Keyword,
    NewspaperBook,
    NewspaperBookSection,
    Publication,
    Series,
    Tone,
    Type,
    /// Override all tags
    All,
}

#[derive(Display, Debug, Deserialize, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum Block<'a> {
    Main,
    Body,
    /// Override all block types
    All,
    BodyLatest,
    BodyLatestWith(i32),
    BodyOldest,
    BodyOldestWith(i32),
    BodyBlockId(&'a str),
    BodyAroundBlockId(&'a str),
    BodyAroundBlockIdWith(&'a str, i32),
    BodyKeyEvents,
    BodyPublishedSince(i64),
}

#[derive(Display, Debug, Deserialize, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum Endpoint {
    Content,
    Tags,
    Sections,
    Editions,
    SingleItem,
}
