use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Display, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum OrderBy {
    Newest,
    Oldest,
    Relevance,
}

#[derive(Display, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum OrderDate {
    Published,
    NewspaperEdition,
    LastModified,
}

#[derive(Display, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum UseDate {
    Published,
    FirstPublication,
    NewspaperEdition,
    LastModified,
}

#[derive(Display, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[strum(serialize_all = "camelCase")]
pub enum Field {
    TrailText,
    Headline,
    ShowInRelatedContent,
    Body,
    LastModified,
    HasStoryPackage,
    Score,
    StandFirst,
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
    All,
}

#[derive(Display, Debug, Serialize, Deserialize, Eq, PartialEq)]
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
    All,
}

#[derive(Display, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum Block<'a> {
    Main,
    Body,
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
