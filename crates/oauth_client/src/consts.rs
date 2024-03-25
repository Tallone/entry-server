use std::str::FromStr;

pub const ENV_REDIRECT_URL: &str = "ENTRY_OAUTH_REDIRECT_URL";
pub const ENV_GITHUB_CLIENT_ID: &str = "ENTRY_OAUTH_GITHUB_CLIENT_ID";
pub const ENV_GITHUB_CLIENT_SECRET: &str = "ENTRY_OAUTH_GITHUB_CLIENT_SECRET";

pub enum OAuthProvider {
  Github,
}

impl FromStr for OAuthProvider {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
