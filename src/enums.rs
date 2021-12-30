use strum_macros::Display;
use serde::{Deserialize, Serialize};

#[derive(Display, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[strum(serialize_all = "camelCase")]
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
