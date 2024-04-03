use clap::Parser;

#[derive(Parser, Debug)]
pub struct Auth {
    #[clap(long)]
    pub key: String,

    #[clap(long)]
    pub secret: String,
}

impl Auth {
    pub fn as_header(&self) -> String {
        let header = format!("sso-key {}:{}", self.key, self.secret);
        return header;
    }
}
