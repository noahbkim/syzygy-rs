use std::collections::HashMap;
use crate::Request;
use hyper::Uri;
use std::borrow::BorrowMut;

pub struct Parameters {
    include: Option<Vec<String>>,
    fields: Option<HashMap<String, Vec<String>>>,
    sort: Option<Vec<String>>,
    pagination: Option<HashMap<String, String>>,
    filter: Option<HashMap<String, String>>,
    extra: Option<HashMap<String, String>>,
}

impl Parameters {
    pub fn parse(uri: &Uri) -> Option<Self> {
        let mut result = Self::new();
        let url = url::Url::parse(uri.path_and_query()?.as_str()).ok()?;
        for (key, value) in url.query_pairs() {
            match Self::check_brackets(key.as_ref()) {
                Some(("fields", field)) => result.add_fields(field.to_string(), value.to_string()),
                Some(("pagination", field)) => result.add_pagination(field.to_string(), value.to_string()),
                Some(("filter", field)) => result.add_filter(field.to_string(), value.to_string()),
                _ => match key.as_ref() {
                    "include" => result.add_include(value.to_string()),
                    "sort" => result.add_sort(value.to_string()),
                    _ => result.add_extra(key.to_string(), value.to_string()),
                }
            }
        }
        Some(result)
    }

    fn add_include(&mut self, value: String) {
        match self.include.as_mut() {
            Some(include) => include.push(value),
            None => self.include = Some(vec![value])
        };
    }

    fn add_fields(&mut self, key: String, value: String) {
        let separated = value.split(',')
            .map(|s| s.trim().to_string())
            .collect();
        match self.fields.as_mut() {
            Some(fields) => {
                fields.insert(key, separated);
            },
            None => {
                self.fields = Some({
                    let mut fields = HashMap::new();
                    fields.insert(key, separated);
                    fields
                });
            }
        };
    }

    fn add_sort(&mut self, value: String) {
        match self.sort.as_mut() {
            Some(sort) => sort.push(value),
            None => self.include = Some(vec![value])
        };
    }

    fn add_pagination(&mut self, key: String, value: String) {
        let separated = value.split(',')
            .map(|s| s.trim().to_string())
            .collect();
        match self.pagination.as_mut() {
            Some(pagination) =>{
                pagination.insert(key, separated);
            }
            None => {
                self.pagination = Some({
                    let mut pagination = HashMap::new();
                    pagination.insert(key, separated);
                    pagination
                });
            }
        };
    }

    fn add_filter(&mut self, key: String, value: String) {
        let separated = value.split(',')
            .map(|s| s.trim().to_string())
            .collect();
        match self.filter.as_mut() {
            Some(filter) => {
                filter.insert(key, separated);
            },
            None => {
                self.filter = Some({
                    let mut filter = HashMap::new();
                    filter.insert(key, separated);
                    filter
                });
            }
        };
    }

    fn add_extra(&mut self, key: String, value: String) {
        let separated = value.split(',')
            .map(|s| s.trim().to_string())
            .collect();
        match self.extra.as_mut() {
            Some(extra) => {
                extra.insert(key, separated);
            },
            None => {
                self.extra = Some({
                    let mut fields = HashMap::new();
                    fields.insert(key, separated);
                    fields
                });
            }
        };
    }

    fn check_brackets(key: &str) -> Option<(&str, &str)> {
        let start = key.find('[')?;
        let end = key.find(']')?;
        return Some((&key[..start], &key[start + 1..end]));
    }

    fn new() -> Self {
        Self {
            include: None,
            fields: None,
            sort: None,
            pagination: None,
            filter: None,
            extra: None,
        }
    }
}
