use roxmltree::Node;

use crate::{SdkResult, error::SdkError};

use super::EventMessage;

pub struct TextMessage {
    pub msg_id: u64,
    pub content: String,
}

impl EventMessage for TextMessage {
    type ReceivedMessage = TextMessage;

    fn from_xml(node: &Node) -> SdkResult<Self::ReceivedMessage> {
        let content = node.descendants().find(|n| n.has_tag_name("Content")).map(|n| n.text()).flatten().ok_or_else(|| SdkError::InvalidParams("parse xml need Content params".to_owned()))?;
        let msg_id = node.descendants().find(|n| n.has_tag_name("MsgId")).map(|n| n.text()).flatten().ok_or_else(|| SdkError::InvalidParams("parse xml need MsgId params".to_owned()))?;
        let msg_id = msg_id.parse::<u64>().map_err(|_e| SdkError::InvalidParams(
                                "Parse XML msg from wechat error: tag `MsgId` should be number"
                                    .to_string()))?;
        Ok(TextMessage {
            content: content.to_owned(),
            msg_id
        })
    }
}