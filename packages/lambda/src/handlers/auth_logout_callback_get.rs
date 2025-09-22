use lambda_http::Body;
use lambda_http::Error;
use lambda_http::Response;

use super::Context;

pub async fn handler(Context { query, .. }: Context) -> Result<Response<Body>, Error> {
  let mut location = String::from("/");

  if let Some(state) = query.get("state") {
    location += &format!("&state={}", state)
  }

  let cookie = now_plus_days(0)?;

  Ok(
    Response::builder()
      .status(307)
      .header("Location", location)
      .header(
        "Set-Cookie",
        format!(
          "auth_refresh_token=null; SameSite=Strict; Path=/api/auth/refresh; HttpOnly; Expires={}",
          cookie
        ),
      )
      .header(
        "Set-Cookie",
        format!(
          "auth_id_token=null; SameSite=Strict; Path=/api; HttpOnly; Expires={}",
          cookie
        ),
      )
      .body(vec![].into())
      .map_err(Box::new)?,
  )
}

fn now_plus_days(days: u64) -> anyhow::Result<String> {
  let now = chrono::offset::Utc::now();
  let Some(now) = now.checked_add_days(chrono::Days::new(days)) else {
    anyhow::bail!("date error")
  };
  Ok(format!("{}", now.format("%+")))
}
