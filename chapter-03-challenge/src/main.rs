use crate::core::{ShortenUrlRequest, UrlShortener};
use lambda_http::{
    http::{Method, StatusCode},
    run, service_fn, tracing, Error, IntoResponse, Request, RequestExt, RequestPayloadExt,
};
use utils::{empty_response, json_response, redirect_response};
mod core;
mod utils;

async fn function_handler(
    url_shortener: &UrlShortener,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    // Manually writing a router in Lambda is not a best practice, in practice you would either use seperate Lambda functions per endpoint or use a web framework like Actix or Axum inside Lambda.
    // This is purely for demonstration purposes to allow us to build a functioning URL shortener and share memory between GET and POST requests.
    match *event.method() {
        Method::POST => {
            if let Some(shorten_url_request) = event.payload::<ShortenUrlRequest>()? {
                let shortened_url_response = url_shortener.shorten_url(shorten_url_request);
                println!("Shortened URL: {:?}", shortened_url_response);
                json_response(&StatusCode::OK, &shortened_url_response)
            } else {
                empty_response(&StatusCode::BAD_REQUEST)
            }
        }

        Method::GET => {
            
            let path = event.uri().path();  // Extract only the path part of the URI       

            // If the request is for the /links endpoint, return a JSON response with all the shortened URLs
            if path == "/links" {
                let urls = url_shortener.get_urls();              
                json_response(&StatusCode::OK, &urls)
            } else {

                let link_id = event
                    .path_parameters_ref()
                    .and_then(|params| params.first("linkId"))
                    .unwrap_or("");

                if link_id.is_empty() {
                    empty_response(&StatusCode::NOT_FOUND)
                } else if let Some(url) = url_shortener.retrieve_url(link_id) {
                    redirect_response(&url)
                } else {
                    Ok(empty_response(&StatusCode::NOT_FOUND)?)
                }
            }
        }

        _ => empty_response(&StatusCode::METHOD_NOT_ALLOWED),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let shortener = UrlShortener::new();

    run(service_fn(|event| function_handler(&shortener, event))).await
}