use megalodon::{generator, SNS};
use anyhow::{Context, Result};

pub struct OAuthResult {
    pub access_token: String,
    pub refresh_token: Option<String>,
}

pub async fn run_oauth(instance_url: &str) -> Result<OAuthResult> {
    let client = generator(SNS::Gotosocial, instance_url.to_string(), None, None).context("error")?;
    let options = megalodon::megalodon::AppInputOptions {
        scopes: Some(vec![
            "read".to_string(),
            "write".to_string(),
            "follow".to_string(),
        ]),
        ..Default::default()
    };

    let app_data = client
        .register_app("loxodocli".to_string(), &options)
        .await
        .context("failed to auth")?;

    let client_id = app_data.client_id;
    let client_secret = app_data.client_secret;

    let auth_url = app_data
        .url
        .as_ref()
        .context("No auth URL")?;
    
    // temp ui
    println!("Copy/paste this url in a browser:");
    println!("{}", auth_url);
    println!("Copy/paste the auth code:");

    let mut code = String::new();
    std::io::stdin()
        .read_line(&mut code)
        .context("Invalid auth code")?;
    let token_data = client
        .fetch_access_token(
            client_id,
            client_secret,
            code.trim().to_string(),
            megalodon::default::NO_REDIRECT.to_string(),
        )
        .await
        .context("Failed to generaet access token")?;

    Ok(OAuthResult {
        access_token: token_data.access_token,
        refresh_token: token_data.refresh_token,
    })
}
