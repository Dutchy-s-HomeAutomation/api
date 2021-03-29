use serde::Deserialize;

#[derive(Deserialize)]
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
pub struct GenericFulfillmentInput {
    pub intent:             FulfillmentIntent
}

#[derive(Deserialize)]
pub enum FulfillmentIntent {
    #[serde(rename(deserialize = "action.devices.SYNC"))]
    SYNC,
    #[serde(rename(deserialize = "action.devices.QUERY"))]
    QUERY
}

#[derive(Deserialize)]
pub struct QueryFulfillmentInput {
    pub intent:             FulfillmentIntent,
    pub payload:            Vec<QueryPayload>
}

#[derive(Deserialize)]
pub struct Device {
    pub id: String
}

#[derive(Deserialize)]
pub struct QueryPayload {
    pub id: Vec<Device>
}

#[derive(Deserialize)]
pub struct ExecuteFulfillmentInput<T> {
    pub intent:             FulfillmentIntent,
    pub payload:            ExecutePayload<T>
}

#[derive(Deserialize)]
pub struct ExecutePayload<T> {
    pub commands:           Vec<ExecuteCommand<T>>
}

#[derive(Deserialize)]
pub struct ExecuteCommand<T> {
    pub devices:            Vec<Device>,
    pub execution:          Execution<T>
}

#[derive(Deserialize)]
pub struct Execution<T> {
    #[serde(rename(deserialize = "command"))]
    pub command:    CommandAction,
    pub params:     T
}

#[derive(Deserialize)]
pub enum CommandAction {

}