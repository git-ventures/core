use anyhow::Error;
use git2::Commit;
pub use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct Contributor {
    pub name: Option<String>,
    pub email: Option<String>,
    pub score: f64,
    pub public_key: Option<String>,
}

impl TryInto<Contributor> for Commit<'_> {
    type Error = Error;
    fn try_into(self) -> Result<Contributor, Error> {
        let author = self.author();
        Ok(Contributor {
            name: author.name().map(|name| name.to_string()),
            email: author.email().map(|email| email.to_string()),
            // TODO: Parse Public key from raw signed header
            public_key: None,
            score: 0.0,
        })
    }
}

impl Contributor {
    pub fn increase_score(&mut self, points: f64) -> f64 {
        self.score += points;
        self.score
    }

    pub fn decrease_score(&mut self, points: f64) -> f64 {
        self.score -= points;
        self.score
    }
}
