use crate::MilkyEvent;
use kovi::bot::BotInformation;
use kovi::error::EventBuildError;
use kovi::event::{Event, InternalEvent};
use kovi::types::ApiAndOptOneshot;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use tokio::sync::mpsc;

/// 群成员减少事件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMemberDecreaseReceiveEventData {
    /// 群号
    pub group_id: i64,
    /// 发生变更的用户 QQ 号
    pub user_id: i64,
    /// 管理员 QQ 号，如果是管理员踢出
    #[serde(default)]
    pub operator_id: Option<i64>,
}

pub type GroupMemberDecreaseEvent = MilkyEvent<GroupMemberDecreaseReceiveEventData>;

impl Event for GroupMemberDecreaseEvent {
    fn de(
        event: &InternalEvent,
        _: &BotInformation,
        _: &mpsc::Sender<ApiAndOptOneshot>,
    ) -> Option<Self> {
        let InternalEvent::DriverEvent(json) = event else {
            return None;
        };

        Self::new(json).ok()
    }
}

impl GroupMemberDecreaseEvent {
    pub(crate) fn new(temp: &Value) -> Result<GroupMemberDecreaseEvent, EventBuildError> {
        crate::event::expect_event_type(temp, "group_member_decrease")?;
        let event: GroupMemberDecreaseEvent = serde_json::from_value(temp.clone())
            .map_err(|e| EventBuildError::ParseError(e.to_string()))?;
        debug!("{event:?}");

        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn rejects_other_event_types() {
        let err = GroupMemberDecreaseEvent::new(&json!({
            "event_type": "group_member_increase",
            "time": 1,
            "self_id": 2,
            "data": {
                "group_id": 100,
                "user_id": 200
            }
        }));
        assert!(err.is_err());
    }
}
