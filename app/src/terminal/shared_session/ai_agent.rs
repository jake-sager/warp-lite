use anyhow::Result;

#[derive(Clone, Debug, Default)]
pub struct ResponseEvent;

pub fn decode_agent_response_event(_encoded: &str) -> Result<ResponseEvent> {
    Ok(ResponseEvent)
}

pub fn encode_agent_response_event(_event: &ResponseEvent) -> String {
    String::new()
}
