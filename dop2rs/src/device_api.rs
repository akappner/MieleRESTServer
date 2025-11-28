use reqwest::{Client, Request, Response};
use reqwest_middleware::{ClientBuilder, Middleware, Next, Result};
use http::Extensions;

use crate::crypto::MieleRequestSignatureInfo;

#[derive(Clone)]
struct MieleAuthMiddleware
{
    
}

impl MieleRequestSignatureInfo
{
    fn get_header_str<'a> (request: &'a Request, key: &'a str)-> &'a str
    {
         request.headers().get(key).map_or("", |x| x.to_str().unwrap())
    }
    fn from_request (request: &Request) -> crate::crypto::MieleRequestSignatureInfo
    {
        let http_method = request.method().as_str();
        let host = Self::get_header_str(request, "Host");
        let request_uri = request.url().as_str();
        let date = Self::get_header_str(request, "Date");
        let content_type =  Self::get_header_str(request, "Content-Type");
        let accept_header =  Self::get_header_str(request, "Accept");
        crate::crypto::MieleRequestSignatureInfo {accept_header: accept_header.to_string(), content_type: content_type.to_string(), date: date.to_string(), host: host.to_string(), http_method: http_method.to_string(), request_uri: request_uri.to_string(), payload: vec!()}
    }
 /*   fn to_request () -> Request 
    {

    }*/
}

#[async_trait::async_trait]
impl Middleware for MieleAuthMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        println!("Request started {:?}", req);
        let res = next.run(req, extensions).await;
        println!("Result: {:?}", res);
        res
    }
}


