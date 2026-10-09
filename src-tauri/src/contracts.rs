use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountKey {
    pub environment: String,
    pub account_id: String,
}

impl AccountKey {
    pub fn stable_id(&self) -> String {
        format!("{}:{}", self.environment, self.account_id)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum LoginRegion {
    #[serde(rename = "CN")]
    China,
    #[serde(rename = "KR")]
    Asia,
    #[serde(rename = "EU")]
    Europe,
    #[serde(rename = "US")]
    Americas,
}

impl LoginRegion {
    pub const fn launch_code(self) -> &'static str {
        match self {
            Self::China => "CN",
            Self::Asia => "KR",
            Self::Europe => "EU",
            Self::Americas => "US",
        }
    }

    pub fn from_environment(environment: &str) -> Option<Self> {
        match environment.trim().to_ascii_lowercase().as_str() {
            "cn" | "cn.actual.battlenet.com.cn" => Some(Self::China),
            "kr" | "kr.actual.battle.net" => Some(Self::Asia),
            "eu" | "eu.actual.battle.net" => Some(Self::Europe),
            "us" | "us.actual.battle.net" => Some(Self::Americas),
            _ => None,
        }
    }

    #[cfg_attr(target_os = "macos", allow(dead_code))]
    pub fn matches_environment(self, environment: &str) -> bool {
        Self::from_environment(environment) == Some(self)
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSnapshot {
    pub app_name: String,
    pub version: String,
    pub mode: String,
    pub platform: String,
    pub data_directory: String,
    pub client: ClientSnapshot,
    pub accounts: Vec<AccountSnapshot>,
    pub login_session: Option<LoginSessionSnapshot>,
    pub notice: Option<String>,
    pub updated_at: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum LoginIntent {
    Add,
    #[serde(rename = "reauthenticate")]
    Reauthenticate {
        #[serde(rename = "accountKey")]
        account_key: AccountKey,
    },
}

impl LoginIntent {
    pub fn requested_region(&self) -> Option<LoginRegion> {
        match self {
            Self::Add => None,
            Self::Reauthenticate { account_key } => {
                LoginRegion::from_environment(&account_key.environment)
            }
        }
    }

    pub fn accepts_account(&self, current: &AccountKey) -> bool {
        match self {
            Self::Add => {
                !current.environment.trim().is_empty() && !current.account_id.trim().is_empty()
            }
            Self::Reauthenticate { account_key } => current == account_key,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginSessionSnapshot {
    pub id: String,
    pub intent: LoginIntent,
    pub created_at: u64,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginCompletionResult {
    pub snapshot: AppSnapshot,
    pub cancelled: bool,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LoginCancellationStatus {
    Accepted,
    Starting,
    TooLate,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientSnapshot {
    pub status: ClientStatus,
    pub executable_path: String,
    pub detected_automatically: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSnapshot {
    pub key: AccountKey,
    pub id: String,
    pub battle_tag: String,
    pub region: String,
    pub environment: String,
    pub snapshot_status: SnapshotStatus,
    pub last_saved_at: Option<u64>,
    pub note: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ClientStatus {
    Running,
    Stopped,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SnapshotStatus {
    Ready,
    Expired,
    Missing,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OperationKind {
    Recovery,
    Switch,
    Login,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationEvent {
    pub kind: OperationKind,
    pub phase: String,
    pub title: String,
    pub detail: String,
    pub progress: u8,
}

#[derive(Clone, Debug)]
pub struct DiscoveredAccount {
    pub key: AccountKey,
    pub internal_name: String,
    pub battle_tag: String,
}

#[derive(Clone, Debug, Default)]
pub struct AccountCatalog {
    pub accounts: Vec<DiscoveredAccount>,
    #[cfg_attr(target_os = "macos", allow(dead_code))]
    pub current_account_key: Option<AccountKey>,
}

#[cfg(test)]
mod tests {
    use super::{AccountKey, LoginIntent, LoginRegion};

    #[test]
    fn login_intent_matches_frontend_wire_shape() {
        assert_eq!(
            serde_json::from_str::<LoginIntent>(
                r#"{"kind":"reauthenticate","accountKey":{"environment":"cn.actual.battlenet.com.cn","accountId":"42"}}"#,
            )
            .unwrap(),
            LoginIntent::Reauthenticate {
                account_key: AccountKey {
                    environment: "cn.actual.battlenet.com.cn".into(),
                    account_id: "42".into(),
                },
            }
        );
        let add = serde_json::from_str::<LoginIntent>(r#"{"kind":"add"}"#).unwrap();
        assert_eq!(add, LoginIntent::Add);
        assert_eq!(serde_json::to_string(&add).unwrap(), r#"{"kind":"add"}"#);
        assert_eq!(add.requested_region(), None);
    }

    #[test]
    fn login_regions_match_battle_net_environment_hosts() {
        for (environment, expected) in [
            ("cn.actual.battlenet.com.cn", LoginRegion::China),
            ("kr.actual.battle.net", LoginRegion::Asia),
            ("eu.actual.battle.net", LoginRegion::Europe),
            ("us.actual.battle.net", LoginRegion::Americas),
        ] {
            assert_eq!(LoginRegion::from_environment(environment), Some(expected));
            assert!(expected.matches_environment(environment));
        }
        assert_eq!(LoginRegion::from_environment("global"), None);
        assert_eq!(LoginRegion::from_environment("tw.actual.battle.net"), None);
        assert_eq!(LoginRegion::from_environment("kr.invalid.example"), None);
        assert_eq!(LoginRegion::from_environment("us.anything"), None);
    }

    #[test]
    fn reauthentication_derives_the_login_region_from_the_account_key() {
        let intent = LoginIntent::Reauthenticate {
            account_key: AccountKey {
                environment: "eu.actual.battle.net".into(),
                account_id: "1:2".into(),
            },
        };
        assert_eq!(intent.requested_region(), Some(LoginRegion::Europe));

        let legacy = LoginIntent::Reauthenticate {
            account_key: AccountKey {
                environment: "global".into(),
                account_id: "1:2".into(),
            },
        };
        assert_eq!(legacy.requested_region(), None);
    }

    #[test]
    fn adding_accepts_a_discovered_identity_but_reauthentication_keeps_the_exact_target() {
        let target = AccountKey {
            environment: "cn".into(),
            account_id: "1:2".into(),
        };
        let other = AccountKey {
            environment: "us".into(),
            account_id: "1:3".into(),
        };
        assert!(LoginIntent::Add.accepts_account(&target));
        assert!(LoginIntent::Add.accepts_account(&other));
        assert!(!LoginIntent::Add.accepts_account(&AccountKey {
            environment: "".into(),
            account_id: "".into()
        }));
        let intent = LoginIntent::Reauthenticate {
            account_key: target.clone(),
        };
        assert!(intent.accepts_account(&target));
        assert!(!intent.accepts_account(&other));
    }
}
