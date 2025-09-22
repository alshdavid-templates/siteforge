use lambda_http::Body;
use lambda_http::Error;
use lambda_http::Response;

use super::Context;

pub async fn handler(Context { config, query, .. }: Context) -> Result<Response<Body>, Error> {
  let mut target = format!(
    "{}{}?client_id={}&logout_uri={}/api/auth/logout/callback",
    config.cognito_origin, config.logout_endpoint, config.cognito_client_id, config.local_origin
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
