use std::env::VarError;

use jsonwebtoken::{
    decode, decode_header, jwk, jwk::AlgorithmParameters, Algorithm, DecodingKey, Header,
    Validation,
};
use rand::Rng;
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::response;
use rocket::serde::{json, Deserialize, Serialize};
use rocket::State;

use crate::errors;
use crate::services::users;

pub fn random_state_string() -> String {
    use rand::{distributions::Alphanumeric, thread_rng};
    let mut rng = thread_rng();
    let chars: String = (0..7).map(|_| rng.sample(Alphanumeric) as char).collect();
    chars
}

#[get("/login")]
pub fn auth0_redirect(
    cookies: &CookieJar,
    settings: &State<Auth0>,
) -> Result<response::Redirect, Status> {
    // TODO make an actual random string
    let state = random_state_string();
    let my_cookie = Cookie::build(("state", state.clone())).same_site(SameSite::Lax); // might be able to get rid of samesite lax if hosted properly
    cookies.add(my_cookie);
    let uri = format!("https://{}/authorize?response_type=code&client_id={}&redirect_uri={}&scope=openid%20profile&state={}",
        settings.auth0_tenant_domain,
        settings.client_id,
        &settings.redirect_uri,
        state);
    Ok(response::Redirect::to(uri))
}

// The callback we get from Auth0
// contains code and state.
// state should be same as we sent
#[get("/callback?<code>&<state>")]
pub async fn auth0_callback(
    code: String,
    state: String,
    cookies: &CookieJar<'_>,
    // db: State<DB>,
    settings: &State<Auth0>,
) -> Result<response::Redirect, Status> {
    let cook = cookies.get_pending("state");
    info!("{:?}", cook);
    if let Some(cookie) = cook {
        if state != cookie.value() {
            return Err(rocket::http::Status::Forbidden);
        }
    } else {
        println!("cookie state bad");
        return Err(rocket::http::Status::BadRequest);
    }
    cookies.remove("state");

    let token_request = settings.create_token_request(&code);
    let endpoint = format!("https://{}/oauth/token", settings.auth0_tenant_domain);
    let client = reqwest::Client::new();

    let Ok(resp_json) = client
        .post(endpoint)
        .header("Content-Type", "application/json")
        .json(&token_request)
        .send()
        .await
    else {
        error!("failed to send token request");
        return Err(rocket::http::Status::BadRequest);
    };

    info!("resp_json {:?}", resp_json);

    let resp = resp_json.json::<TokenResponse>().await;

    match resp {
        Ok(r) => {
            info!("it worked {:?}", r);
            let token = decode_jwt(r, settings).await;
            info!("decoded {:?}", token)
        }
        Err(e) => {
            error!("failed to deserialize token response {:?}", e);
            return Err(rocket::http::Status::BadRequest);
        }
    }

    // we might want to decode if we use a certificate

    // TODO decode the JWT

    // let user = users::get_or_create_user(new_user)

    // let user = get_or_create_user(&db, &payload).map_err(|e| match e.downcast_ref() {
    //     Some(AuthError::MalformedJWT { .. }) => Status::BadRequest,
    //     _ => Status::InternalServerError,
    // })?;

    // let jwt = &resp.id_token.clone();
    // let hashed_jwt = hex_digest(HashAlgorithm::SHA256, jwt.as_bytes());
    // let new_session = Session {
    //     user_id: user.user_id,
    //     expires: payload.exp,
    //     raw_jwt: jwt.as_bytes().to_vec(),
    // };
    // let encoded_session = serialize(&new_session).map_err(|_| Status::Unauthorized)?;
    // let session_key = make_key!("sessions/", hashed_jwt.clone());
    // db.set(session_key.0, encoded_session).unwrap();
    // let cookie = Cookie::build("session", hashed_jwt)
    //     .path("/")
    //     .secure(true)
    //     .http_only(true)
    //     .finish();
    // cookies.add(cookie);

    Ok(response::Redirect::to("/loggedin"))
}

#[get("/loggedin")]
pub fn logged_in() -> response::Redirect {
    info!("logged in!");
    response::Redirect::to("/")
}

pub struct Auth0 {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    auth0_tenant_domain: String,
}

impl Auth0 {
    // TODO replace VarError with real error
    pub fn from_env() -> Result<Auth0, VarError> {
        let app_settings = Auth0 {
            client_id: std::env::var("AUTH0_CLIENT_ID")?,
            client_secret: std::env::var("AUTH0_CLIENT_SECRET")?,
            redirect_uri: std::env::var("AUTH0_REDIRECT_URI")?,
            auth0_tenant_domain: std::env::var("AUTH0_DOMAIN")?,
        };
        Ok(app_settings)
    }

    // make mame a token request with our secret and the code recieved from auth0
    fn create_token_request(&self, code: &str) -> TokenRequest {
        TokenRequest {
            grant_type: String::from("authorization_code"),
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            code: code.to_string(),
            redirect_uri: self.redirect_uri.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct TokenRequest {
    grant_type: String,
    client_id: String,
    client_secret: String,
    code: String,
    redirect_uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct TokenResponse {
    access_token: String,
    expires_in: u32,
    id_token: String,
    token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct IdTokenClaims {
    sub: String,
    nickname: String,
    exp: usize,
}

async fn get_jwks(settings: &State<Auth0>) -> Result<jwk::JwkSet, errors::Error> {
    // https://{yourDomain}/.well-known/jwks.json

    let endpoint = format!(
        "https://{}/.well-known/jwks.json",
        settings.auth0_tenant_domain
    );
    let client = reqwest::Client::new();

    let Ok(resp_json) = client
        .get(endpoint)
        .header("Content-Type", "application/json")
        .send()
        .await
    else {
        error!("failed to send token request");
        return Err(errors::Error::Placeholder(
            "failed to send token request".into(),
        ));
    };

    info!("resp_json {:?}", resp_json);

    let jwks: jwk::JwkSet = json::from_str(
        r#"
    {"keys":[{"alg":"RS256","kty":"RSA","use":"sig","n":"2V31IZF-EY2GxXQPI5OaEE--sezizPamNZDW9AjBE2cCErfufM312nT2jUsCnfjsXnh6Z_b-ncOMr97zIZkq1ofU7avemv8nX7NpKmoPBpVrMPprOax2-e3wt-bSfFLIHyghjFLKpkT0LOL_Fimi7xY-J86R06WHojLo3yGzAgQCswZmD4CFf6NcBWDcb6l6kx5vk_AdzHIkVEZH4aikUL_fn3zq5qbE25oOg6pT7F7Pp4zdHOAEKnIRS8tvP8tvvVRkUCrjBxz_Kx6Ne1YOD-fkIMRk_MgIWeKZZzZOYx4VrC0vqYiM-PcKWbNdt1kNoTHOeL06XZeSE6WPZ3VB1Q","e":"AQAB","kid":"1Z57d_i7TE6KTY57pKzDy","x5t":"1gA-aTE9VglLXZnrqvzwWhHsFdk","x5c":["MIIDDTCCAfWgAwIBAgIJHwhLfcIbNvmkMA0GCSqGSIb3DQEBCwUAMCQxIjAgBgNVBAMTGWRldi1kdXp5YXlrNC5ldS5hdXRoMC5jb20wHhcNMjEwNjEzMDcxMTQ1WhcNMzUwMjIwMDcxMTQ1WjAkMSIwIAYDVQQDExlkZXYtZHV6eWF5azQuZXUuYXV0aDAuY29tMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA2V31IZF+EY2GxXQPI5OaEE++sezizPamNZDW9AjBE2cCErfufM312nT2jUsCnfjsXnh6Z/b+ncOMr97zIZkq1ofU7avemv8nX7NpKmoPBpVrMPprOax2+e3wt+bSfFLIHyghjFLKpkT0LOL/Fimi7xY+J86R06WHojLo3yGzAgQCswZmD4CFf6NcBWDcb6l6kx5vk/AdzHIkVEZH4aikUL/fn3zq5qbE25oOg6pT7F7Pp4zdHOAEKnIRS8tvP8tvvVRkUCrjBxz/Kx6Ne1YOD+fkIMRk/MgIWeKZZzZOYx4VrC0vqYiM+PcKWbNdt1kNoTHOeL06XZeSE6WPZ3VB1QIDAQABo0IwQDAPBgNVHRMBAf8EBTADAQH/MB0GA1UdDgQWBBRPX3shmtgajnR4ly5t9VYB66ufGDAOBgNVHQ8BAf8EBAMCAoQwDQYJKoZIhvcNAQELBQADggEBAHtKpX70WU4uXOMjbFKj0e9HMXyCrdcX6TuYiMFqqlOGWM4yghSM8Bd0HkKcirm4DUoC+1dDMzXMZ+tbntavPt1xG0eRFjeocP+kIYTMQEG2LDM5HQ+Z7bdcwlxnuYOZQfpgKAfYbQ8Cxu38sB6q82I+5NJ0w0VXuG7nUZ1RD+rkXaeMYHNoibAtKBoTWrCaFWGV0E55OM+H0ckcHKUUnNXJOyZ+zEOzPFY5iuYIUmn1LfR1P0SLgIMfiooNC5ZuR/wLdbtyKtor2vzz7niEiewz+aPvfuPnWe/vMtQrfS37/yEhCozFnbIps/+S2Ay78mNBDuOAA9fg5yrnOmjABCU="]},{"alg":"RS256","kty":"RSA","use":"sig","n":"0KDpAuJZyDwPg9CfKi0R3QwDROyH0rvd39lmAoqQNqtYPghDToxFMDLpul0QHttbofHPJMKrPfeEFEOvw7KJgelCHZmckVKaz0e4tfu_2Uvw2kFljCmJGfspUU3mXxLyEea9Ef9JqUru6L8f_0_JIDMT3dceqU5ZqbG8u6-HRgRQ5Jqc_fF29Xyw3gxNP_Q46nsp_0yE68UZE1iPy1om0mpu8mpsY1-Nbvm51C8i4_tFQHdUXbhF4cjAoR0gZFNkzr7FCrL4On0hKeLcvxIHD17SxaBsTuCBGd35g7TmXsA4hSimD9taRHA-SkXh558JG5dr-YV9x80qjeSAvTyjcQ","e":"AQAB","kid":"v2HFn4VqJB-U4vtQRJ3Ql","x5t":"AhUBZjtsFdx7C1PFtWAJ756bo5k","x5c":["MIIDDTCCAfWgAwIBAgIJSSFLkuG8uAM8MA0GCSqGSIb3DQEBCwUAMCQxIjAgBgNVBAMTGWRldi1kdXp5YXlrNC5ldS5hdXRoMC5jb20wHhcNMjEwNjEzMDcxMTQ2WhcNMzUwMjIwMDcxMTQ2WjAkMSIwIAYDVQQDExlkZXYtZHV6eWF5azQuZXUuYXV0aDAuY29tMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA0KDpAuJZyDwPg9CfKi0R3QwDROyH0rvd39lmAoqQNqtYPghDToxFMDLpul0QHttbofHPJMKrPfeEFEOvw7KJgelCHZmckVKaz0e4tfu/2Uvw2kFljCmJGfspUU3mXxLyEea9Ef9JqUru6L8f/0/JIDMT3dceqU5ZqbG8u6+HRgRQ5Jqc/fF29Xyw3gxNP/Q46nsp/0yE68UZE1iPy1om0mpu8mpsY1+Nbvm51C8i4/tFQHdUXbhF4cjAoR0gZFNkzr7FCrL4On0hKeLcvxIHD17SxaBsTuCBGd35g7TmXsA4hSimD9taRHA+SkXh558JG5dr+YV9x80qjeSAvTyjcQIDAQABo0IwQDAPBgNVHRMBAf8EBTADAQH/MB0GA1UdDgQWBBSEkRwvkyYzzzY/jPd1n7/1VRQNdzAOBgNVHQ8BAf8EBAMCAoQwDQYJKoZIhvcNAQELBQADggEBAGtdl7QwzpaWZjbmd6UINAIlpuWIo2v4EJD9kGan/tUZTiUdBaJVwFHOkLRsbZHc5PmBB5IryjOcrqsmKvFdo6wUZA92qTuQVZrOTea07msOKSWE6yRUh1/VCXH2+vAiB9A4DFZ23WpZikBR+DmiD8NGwVgAwWw9jM6pe7ODY+qxFXGjQdTCHcDdbqG2160nKEHCBvjR1Sc/F0pzHPv8CBJCyGAPTCXX42sKZI92pPzdKSmNNijCuIEYLsjzKVxaUuwEqIshk3mYeu6im4VmXXFj+MlyMsusVWi2py7fGFadamzyiV/bxZe+4xzzrRG1Kow/WnVEizfTdEzFXO6YikE="]}]}
    "#,
    )?;

    Ok(jwks)
}

async fn decode_jwt(
    token_response: TokenResponse,
    settings: &State<Auth0>,
) -> Result<IdTokenClaims, errors::Error> {
    let header = decode_header(&token_response.id_token)?;
    let kid = match header.kid {
        Some(k) => k,
        None => return Err(errors::Error::Placeholder("no kid in jwt".into())),
    };

    let jwks = get_jwks(settings).await?;

    if let Some(j) = jwks.find(&kid) {
        match &j.algorithm {
            AlgorithmParameters::RSA(rsa) => {
                let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap();

                let mut validation = Validation::new(Algorithm::RS256);
                validation
                    .set_audience(&[format!("https://{}/api/v2/", settings.auth0_tenant_domain)]);
                validation.validate_exp = false;
                let decoded_token =
                    decode::<IdTokenClaims>(&token_response.id_token, &decoding_key, &validation);
                //         .unwrap();
                // println!("{:?}", decoded_token);
            }
            _ => unreachable!("this should be a RSA"),
        }
    } else {
        return Err(errors::Error::Placeholder(
            "No matching JWK found for the given kid".into(),
        ));
    }

    Ok(IdTokenClaims {
        sub: "not real".into(),
        nickname: "not real".into(),
        exp: 9,
    })

    // let token = jsonwebtoken::decode::<IdTokenClaims>(
    //     &r.id_token,
    //     &DecodingKey::from_rsa_raw_components(modulus, exponent),
    //     &Validation::new(Algorithm::RS256),
    // );
}

// TODO Use my errors throughout this file. Maybe not so necessary for the routes that return statuses
