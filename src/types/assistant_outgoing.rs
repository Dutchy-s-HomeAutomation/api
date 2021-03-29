use serde::Serialize;

#[derive(Serialize)]
pub struct FulfillmentResponse<T> {
    #[serde(rename(serialize = "requestId"))]
    request_id:         String,
    payload:            T
}

#[derive(Serialize)]
pub enum SyncDeviceStatus {
    SUCCESS,
    OFFLINE,
    EXCEPTIONS,
    ERROR
}

#[derive(Serialize)]
pub enum ExecuteDeviceStatus {

}

#[derive(Serialize)]
pub struct SyncFulfillmentPayload {
    #[serde(rename(serialize = "agentUserId"))]
    agent_user_id:      String,
    devices:            Vec<SyncDevice>
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncDevice {
    id:                 String,

    #[serde(rename(serialize = "type"))]
    deviceType:         DeviceType,
    traits:             Vec<DeviceTrait>,
    name:               DeviceName,
    will_report_state:  bool,
    device_info:        Option<DeviceInfo>
}

#[derive(Serialize)]
pub struct DeviceName {
    #[serde(rename(serialize = "defaultNames"))]
    default_names:      Option<Vec<String>>,
    name:               String,
    nicknames:          Option<Vec<String>>
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    manufacturer:       String,
    model:              String,
    hw_version:         String,
    sw_version:         String
}

#[derive(Serialize)]
pub enum DeviceTrait {

}

#[derive(Serialize)]
pub enum DeviceType {

}