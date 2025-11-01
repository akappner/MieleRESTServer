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
        let httpMethod = request.method().as_str();
        let host = Self::get_header_str(request, "Host");
        let request_uri = request.url().as_str();
        let date = Self::get_header_str(request, "Date");
        let contentType =  Self::get_header_str(request, "Content-Type");
        let acceptHeader =  Self::get_header_str(request, "Accept");
        crate::crypto::MieleRequestSignatureInfo {acceptHeader: acceptHeader.to_string(), contentType: contentType.to_string(), date: date.to_string(), host: host.to_string(), httpMethod: httpMethod.to_string(), request_uri: request_uri.to_string(), payload: vec!()}
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


