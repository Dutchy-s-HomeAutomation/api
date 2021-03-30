use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct FulfillmentRequest<T> {
    #[serde(rename(deserialize = "requestId"))]
    pub request_id:         String,
    pub inputs:             Vec<T>
}

/**
Used for generic deserialization, e.g. to get the intent
Also used for the SYNC intent
*/
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct GenericFulfillmentInput {
    pub intent:             FulfillmentIntent
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub enum FulfillmentIntent {
    #[serde(rename(deserialize = "action.devices.SYNC"))]
    SYNC,
    #[serde(rename(deserialize = "action.devices.QUERY"))]
    QUERY
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct QueryFulfillmentInput {
    pub intent:             FulfillmentIntent,
    pub payload:            Vec<QueryPayload>
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Device {
    pub id: String
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct QueryPayload {
    pub id: Vec<Device>
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ExecuteFulfillmentInput<T> {
    pub intent:             FulfillmentIntent,
    pub payload:            ExecutePayload<T>
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ExecutePayload<T> {
    pub commands:           Vec<ExecuteCommand<T>>
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ExecuteCommand<T> {
    pub devices:            Vec<Device>,
    pub execution:          Execution<T>
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Execution<T> {
    #[serde(rename(deserialize = "command"))]
    pub command:    CommandAction,
    pub params:     T
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub enum CommandAction {

}