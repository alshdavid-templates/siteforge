use lambda_http::Body;
use lambda_http::Error;
use lambda_http::Response;

use super::Context;

pub async fn handler(Context { config, query, req, .. }: Context) -> Result<Response<Body>, Error> {
  let return_origin = match req.headers().get("host") {
    Some(host) => host.to_str()?,
    None => config.local_origin.as_str(),
  };
  
  let mut target = format!(
    "{}{}?client_id={}&logout_uri={}/api/auth/logout/callback",
    config.cognito_origin, config.logout_endpoint, config.cognito_client_id, return_origin
  );

  if let Some(state) = query.get("state") {
    target += &format!("&state={}", state)
  }

  Ok(
    Response::builder()
      .status(307)
      .header("Location", target)
      .body(vec![].into())
      .map_err(Box::new)?,
  )
}
