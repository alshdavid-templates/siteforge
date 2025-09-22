use cookie::Cookie;
use lambda_http::Body;
use lambda_http::Error;
use lambda_http::Response;

use super::Context;

pub async fn handler(Context { req, .. }: Context) -> Result<Response<Body>, Error> {
  let Some(cookie_header) = req.headers().get("Cookie") else {
    return Ok(
      Response::builder()
        .status(403)
        .body("".into())
        .map_err(Box::new)?,
    );
  };

  for cookie in cookie_header.to_str()?.split(";") {
    let cookie = Cookie::parse_encoded(cookie)?;
    match cookie.name() {
      "auth_id_token" => {
        let id_token = cookie.value();
        let payload_b64 = id_token.split_terminator(".").collect::<Vec<&str>>()[1];
        let payload = decode_b64(payload_b64)?;
        return Ok(
          Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(payload.into())
            .map_err(Box::new)?,
        );
      }
      _ => continue,
    }
  }

  Ok(
    Response::builder()
      .status(403)
      .body("".into())
      .map_err(Box::new)?,
  )
}

fn decode_b64(input: &str) -> anyhow::Result<String> {
  use base64::{Engine as _, engine::general_purpose::STANDARD};

  match STANDARD.decode(input) {
    Ok(decoded_bytes) => Ok(String::from_utf8(decoded_bytes)?),
    Err(_e) => Err(anyhow::anyhow!("Failed to parse")),
  }
}
