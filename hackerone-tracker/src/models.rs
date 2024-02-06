use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Deserialize, FromRow)]
pub struct HackerOneThanksDB {
    pub username: String,
    pub user_id: String,
    pub reputation: i64,
    pub profile_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HackerOneThanks {
    pub username: String,
    pub id: String,
    pub reputation: i64,
    pub avatar_url: String,
    pub position: usize,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ReputationEffect {
    TriagedOrResolved, // +7 (will be removed if closed with state other than resolved)
    RemovedTriageOrResolved, // -7
    DuplicateOfResolvedReportBeforePublic, // +2
    Informative,       // 0
    NotApplicable,     // -5
    Spam,              // -10,
    BountySevere,      // +50
    BountyHigh,        // +25
    BountyMedium,      // +15
    BountyLow,         // +10,
    Unknown,
}

impl HackerOneThanks {
    pub fn get_hackerone_url(&self) -> String {
        format!("https://hackerone.com/{}", self.username)
    }

    pub fn get_avatar_url(&self) -> String {
        let default_avatar = String::from("https://hackerone.com/assets/avatars/default-25f7248a18bdf9e2dc8310319b148d66cff430fa0fade6c5f25fee1b8d7f27ed.png");

        if self.avatar_url.len() > 2048 {
            return default_avatar;
        } else if self.avatar_url.starts_with("https") {
            return self.avatar_url.clone();
        } else {
            return default_avatar;
        }
    }

    // https://docs.hackerone.com/hackers/reputation.html#effects-of-report-state-on-reputation
    pub fn reputation_type(&self, reputation: i64) -> ReputationEffect {
        match reputation {
            -10 => ReputationEffect::Spam,
            -7 => ReputationEffect::RemovedTriageOrResolved,
            -5 => ReputationEffect::NotApplicable,
            0 => ReputationEffect::Informative,
            2 => ReputationEffect::DuplicateOfResolvedReportBeforePublic,
            7 => ReputationEffect::TriagedOrResolved,
            10 => ReputationEffect::BountyLow,
            15 => ReputationEffect::BountyMedium,
            25 => ReputationEffect::BountyHigh,
            50 => ReputationEffect::BountySevere,
            _ => ReputationEffect::Unknown,
        }
    }
}
