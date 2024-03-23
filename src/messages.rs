use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Airprint = 0x03,
    AirDrop = 0x05,
    HomeKit = 0x06,
    ProximityPairing = 0x07,
    HeySiri = 0x08,
    AirplayTarget = 0x09,
    AirplaySource = 0x0A,
    MagicSwitch = 0x0B,
    Handoff = 0x0C,
    TetheringSource = 0x0E,
    TetheringTarget = 0x0D,
    NearbyAction = 0x0F,
    NearbyInfo = 0x10,
    FindMy = 0x12,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHeader {
    pub opcode: u8,
    pub length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProximityPairMessage {
    pub header: MessageHeader,
    pub device_model: u16,
    pub device_status: u8,
    pub battery1: u8,
    pub battery2: u8,
    pub lid_open_count: u8,
    pub device_color: u8,
    pub encrypted_payload: Vec<u8>,
}
