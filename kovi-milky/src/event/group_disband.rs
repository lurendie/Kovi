use crate::MilkyEvent;
use kovi::bot::BotInformation;
use kovi::error::EventBuildError;
use kovi::event::{Event, InternalEvent};
use kovi::types::ApiAndOptOneshot;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use tokio::sync::mpsc;

/// 群解散事件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDisbandReceiveEventData {
    /// 群号
    pub group_id: i64,
    /// 操作者 QQ 号
    pub operator_id: i64,
}

pub type GroupDisbandEvent = MilkyEvent<GroupDisbandReceiveEventData>;

impl Event for GroupDisbandEvent {
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

impl GroupDisbandEvent {
    pub(crate) fn new(temp: &Value) -> Result<GroupDisbandEvent, EventBuildError> {
        crate::event::expect_event_type(temp, "group_disband")?;

        let event: GroupDisbandEvent = serde_json::from_value(temp.clone())
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
    fn parses_group_disband() {
        let event = GroupDisbandEvent::new(&json!({
            "event_type": "group_disband",
            "time": 1,
            "self_id": 2,
            "data": {
                "group_id": 100,
                "operator_id": 200
            }
        }))
        .expect("parse group_disband");
        assert_eq!(event.data.group_id, 100);
        assert_eq!(event.data.operator_id, 200);
    }

    #[test]
    fn rejects_other_event_types() {
        let err = GroupDisbandEvent::new(&json!({
            "event_type": "group_name_change",
            "time": 1,
            "self_id": 2,
            "data": {
                "group_id": 100,
                "new_group_name": "x",
                "operator_id": 200
            }
        }));
        assert!(err.is_err());
    }
}
