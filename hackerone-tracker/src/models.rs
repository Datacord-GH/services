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
    pub user_id: String,
    pub recognized_report_count: i64,
    pub reputation: i64,
    pub profile_url: String,
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
        format!(
            "https://hackerone-api.discord.workers.dev/user-avatars/{}",
            self.username
        )
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
