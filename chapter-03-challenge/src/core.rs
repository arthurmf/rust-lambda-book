use cuid2::CuidConstructor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;
//use parking_lot::RwLock; // Use this instead of std::sync::RwLock if you want to use a more efficient RwLock implementation

#[derive(Deserialize)]
pub struct ShortenUrlRequest {
    url_to_shorten: String,
}

#[derive(Serialize, Debug)]
pub struct ShortenUrlResponse {
    shortened_url: String,
}

#[derive(Default, Debug)]
pub struct UrlShortener {
    urls: RwLock<HashMap<String, String>>,
}

impl UrlShortener {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn shorten_url(&self, req: ShortenUrlRequest) -> ShortenUrlResponse {
        let short_url = self.generate_short_url();

        let mut map = self.urls.write().unwrap();
        map.insert(short_url.clone(), req.url_to_shorten);

        ShortenUrlResponse {
            shortened_url: short_url,
        }
    }

    pub fn retrieve_url(&self, short_url: &str) -> Option<String> {
        let map = self.urls.read().unwrap();
        map.get(short_url).cloned()
    }

    pub fn get_urls(&self) -> HashMap<String, String> {
        self.urls.read().unwrap().clone()
    }

    fn generate_short_url(&self) -> String {
        let idgen = CuidConstructor::new().with_length(10);
        idgen.create_id()
    }
    
}