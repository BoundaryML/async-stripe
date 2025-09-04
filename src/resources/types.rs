use serde::{Deserialize, Serialize};

use crate::params::Timestamp;

/// An enum representing the versions of the Stripe API.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ApiVersion {
    #[serde(rename = "2025-05-28.basil")]
    V2025_05_28_basil,
    #[serde(rename = "2025-08-27.basil")]
    V2025_08_27_basil,
}

impl ApiVersion {
    pub fn as_str(self) -> &'static str {
        match self {
            ApiVersion::V2025_05_28_basil => "2025-05-28.basil",
            ApiVersion::V2025_08_27_basil => "2025-08-27.basil",
        }
    }
}

impl AsRef<str> for ApiVersion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/* Developers note -- DelayDays and DelayDaysOther are not worth the trouble
 * to automate.  Recommend letting the mapping stand*/
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum DelayDays {
    Days(u32),
    Other(DelayDaysOther),
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DelayDaysOther {
    Minimum,
}

impl DelayDays {
    pub fn days(n: u32) -> Self {
        DelayDays::Days(n)
    }
    pub fn minimum() -> Self {
        DelayDays::Other(DelayDaysOther::Minimum)
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Scheduled {
    Timestamp(Timestamp),
    Other(ScheduledOther),
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ScheduledOther {
    Now,
}

impl Scheduled {
    pub fn at(ts: Timestamp) -> Self {
        Scheduled::Timestamp(ts)
    }
    pub fn now() -> Self {
        Scheduled::Other(ScheduledOther::Now)
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum UpTo {
    Max(u64),
    Other(UpToOther),
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpToOther {
    Inf,
}

impl UpTo {
    pub fn max(n: u64) -> Self {
        UpTo::Max(n)
    }
    pub fn now() -> Self {
        UpTo::Other(UpToOther::Inf)
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum PaymentIntentOffSession {
    Exists(bool),
    Other(OffSessionOther),
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OffSessionOther {
    #[serde(rename = "one_off")]
    OneOff,
    #[serde(rename = "recurring")]
    Recurring,
}

impl PaymentIntentOffSession {
    pub fn exists(n: bool) -> Self {
        PaymentIntentOffSession::Exists(n)
    }
    pub fn frequency(n: OffSessionOther) -> Self {
        match n {
            OffSessionOther::OneOff => PaymentIntentOffSession::Other(OffSessionOther::OneOff),
            OffSessionOther::Recurring => {
                PaymentIntentOffSession::Other(OffSessionOther::Recurring)
            }
        }
    }
}
