//! Enum types that prevent passing illegal parameters to the
//! Guardian's content API.

use serde::Deserialize;
use strum_macros::Display;

#[derive(Clone, Display, Debug, Deserialize, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum OrderBy {
    Newest,
    Oldest,
    Relevance,
}

#[derive(Clone, Display, Debug, Deserialize, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum OrderDate {
    Published,
    NewspaperEdition,
    LastModified,
}

#[derive(Clone, Display, Debug, Deserialize, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum UseDate {
    Published,
    FirstPublication,
    NewspaperEdition,
    LastModified,
}

#[derive(Clone, Display, Debug, Deserialize, Eq, PartialEq)]
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

#[derive(Clone, Display, Debug, Deserialize, Eq, PartialEq)]
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

#[derive(Clone, Display, Debug, Deserialize, Eq, PartialEq)]
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

#[derive(Clone, Display, Default, Debug, Deserialize, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum Endpoint {
    #[default]
    Content,
    Tags,
    Sections,
    Editions,
    SingleItem,
}

/// Whether a trait matches the `All` variant
pub(crate) trait IsAll {
    fn is_all(&self) -> bool;
}

macro_rules! impl_is_all {
    ($enum_type:ident) => {
        impl IsAll for $enum_type {
            fn is_all(&self) -> bool {
                matches!(self, $enum_type::All)
            }
        }
    };
    ($enum_type:ident <$lifetime:lifetime>) => {
        impl<$lifetime> IsAll for $enum_type<$lifetime> {
            fn is_all(&self) -> bool {
                matches!(self, $enum_type::All)
            }
        }
    };
}

impl_is_all!(Block<'a>);
impl_is_all!(Field);
impl_is_all!(Tag);
