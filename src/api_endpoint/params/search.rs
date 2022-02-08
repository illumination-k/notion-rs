use reqwest::Body;
use serde::{ser::SerializeStruct, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Order {
    Ascending,
    Descending,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortChoises {
    LastEditedTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SortParams {
    timestamp: SortChoises,
    direction: Order,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FilterValue {
    Page,
    Database,
}

#[derive(Debug)]
pub struct Filter {
    value: FilterValue,
}

impl Filter {
    pub fn new(value: FilterValue) -> Self {
        Self { value }
    }
}

impl Serialize for Filter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("FilterParams", 2)?;
        state.serialize_field("property", "object")?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

#[must_use]
#[derive(Debug, Serialize)]
pub struct Params {
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<SortParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Filter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_cursor: Option<String>,
}

impl Params {
    pub fn new() -> Self {
        Self {
            sort: None,
            filter: None,
            query: None,
            page_size: None,
            start_cursor: None,
        }
    }

    pub fn with_query<S>(mut self, query: S) -> Self
    where
        S: ToString,
    {
        self.query = Some(query.to_string());
        self
    }

    pub fn with_filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }
}

impl Into<Body> for Params {
    fn into(self) -> Body {
        Body::from(serde_json::to_string(&self).unwrap())
    }
}
