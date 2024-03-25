use serde::{Deserialize, Serialize};

use crate::{Error, ErrorKind};

// Messages to implement
enum MessageOpcode {
    AirDrop = 0x05,
    HomeKit = 0x06,
    HeySiri = 0x08,
    AirplaySource = 0x0A,
    MagicSwitch = 0x0B,
    TetheringSource = 0x0E,
    TetheringTarget = 0x0D,
    NearbyAction = 0x0F,
    NearbyInfo = 0x10,
    FindMy = 0x12,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    Airprint(AirPrintMessage),
    AirplayTarget(AirplayTargetMessage),
    ProximityPairing(ProximityPairMessage),
    Handoff(HandoffMessage),
    NearbyInfo(NearbyInfoMessage),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirPrintMessage {
    pub header: MessageHeader,
    pub connection_info: u8,
    pub rp_index_value: u16,
    pub port: u16,
    pub ip4_address: u32,
    pub ip6_address: u128,
    pub measured_power: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirplayTargetMessage {
    pub header: MessageHeader,
    pub flags: u8,
    pub seed: u8,
    pub ip4_address: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandoffMessage {
    pub header: MessageHeader,
    pub clipboard_status: u8,
    pub iv: u16,
    pub gcm_auth: u8,
    pub encrypted_payload: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearbyInfoMessage {
    pub header: MessageHeader,
    pub status_flags: u8,
    pub action_code: u8,
    pub data_flags: u8,
    pub auth_tag: u32,
}

impl Message {
    pub fn decode(data: &[u8]) -> Result<Message, Error> {
        match data[0] {
            0x07 => Ok(Message::ProximityPairing(ProximityPairMessage::decode(
                data,
            )?)),
            0x09 => Ok(Message::AirplayTarget(AirplayTargetMessage::decode(data)?)),
            0x03 => Ok(Message::Airprint(AirPrintMessage::decode(data)?)),
            0x0C => Ok(Message::Handoff(HandoffMessage::decode(data)?)),
            0x10 => Ok(Message::NearbyInfo(NearbyInfoMessage::decode(data)?)),
            _ => Err(Error::new(
                ErrorKind::DecodeError,
                format!("Unknown opcode: {:02X?}", data[0]).as_str(),
            )),
        }
    }
}

impl TryFrom<&[u8]> for Message {
    type Error = crate::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::decode(value)
    }
}

impl ProximityPairMessage {
    pub fn decode(data: &[u8]) -> Result<ProximityPairMessage, Error> {
        if data.len() < 2 {
            return Err(Error::new(
                ErrorKind::DecodeError,
                "Length mismatch. Cannot read opcode + length",
            ));
        }

        let opcode = data[0];
        if opcode != 0x07 {
            return Err(Error::new(
                ErrorKind::DecodeError,
                "Tried to decode Proximity Pair message with invalid opcode. Expected 0x07",
            ));
        }

        let length = data[1] as usize;
        if data.len() < length + 2 {
            return Err(Error::new(
                ErrorKind::DecodeError,
                "Packet length != buffer length",
            ));
        }

        let mut encrypted_payload = vec![0; length - 11];
        encrypted_payload.copy_from_slice(&data[11..length]);

        Ok(ProximityPairMessage {
            header: MessageHeader { opcode, length },
            device_model: ((data[3] as u16) << 8) + (data[4] as u16),
            device_status: data[5],
            battery1: data[6],
            battery2: data[7],
            lid_open_count: data[8],
            device_color: data[9],
            encrypted_payload,
        })
    }
}

impl TryFrom<&[u8]> for ProximityPairMessage {
    type Error = crate::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::decode(value)
    }
}

impl AirPrintMessage {
    pub fn decode(data: &[u8]) -> Result<AirPrintMessage, Error> {
        if data.len() < 2 {
            return Err(Error::new(
                ErrorKind::DecodeError,
                "Length mismatch. Cannot read opcode + length",
            ));
        }

        let opcode = data[0];
        if opcode != 0x03 {
            return Err(Error::new(
                ErrorKind::DecodeError,
                "Tried to decode AirPrint message with invalid opcode. Expected 0x03",
            ));
        }

        let length = data[1] as usize;
        if data.len() < length + 2 {
            return Err(Error::new(
                ErrorKind::DecodeError,
                "Packet length != buffer length",
            ));
        }

        Ok(AirPrintMessage {
            header: MessageHeader { opcode, length },
            connection_info: data[2],
            rp_index_value: ((data[3] as u16) << 8) + (data[4] as u16),
            port: ((data[5] as u16) << 8) + (data[6] as u16),
            ip4_address: ((data[7] as u32) << 24)
                + ((data[8] as u32) << 16)
                + ((data[9] as u32) << 8)
                + (data[10] as u32),
            ip6_address: ((data[11] as u128) << 88)
                + ((data[12] as u128) << 80)
                + ((data[13] as u128) << 72)
                + ((data[14] as u128) << 64)
                + ((data[15] as u128) << 56)
                + ((data[16] as u128) << 48)
                + ((data[17] as u128) << 40)
                + ((data[18] as u128) << 32)
                + ((data[19] as u128) << 24)
                + ((data[20] as u128) << 16)
                + ((data[21] as u128) << 8)
                + (data[22] as u128),
            measured_power: data[23],
        })
    }
}

impl TryFrom<&[u8]> for AirPrintMessage {
    type Error = crate::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::decode(value)
    }
}

impl AirplayTargetMessage {
    pub fn decode(data: &[u8]) -> Result<AirplayTargetMessage, Error> {
        if data.len() < 2 {
            return Err(Error::new(
                ErrorKind::DecodeError,
                "Length mismatch. Cannot read opcode + length",
            ));
        }

        let opcode = data[0];
        if opcode != 0x09 {
            return Err(Error::new(
                ErrorKind::DecodeError,
                "Tried to decode Airplay Target message with invalid opcode. Expected 0x09",
            ));
        }

        let length = data[1] as usize;
        if data.len() < length + 2 {
            return Err(Error::new(
                ErrorKind::DecodeError,
                "Packet length != buffer length",
            ));
        }

        Ok(AirplayTargetMessage {
            header: MessageHeader { opcode, length },
            flags: data[2],
            seed: data[3],
            ip4_address: ((data[4] as u32) << 24)
                + ((data[5] as u32) << 16)
                + ((data[6] as u32) << 8)
                + (data[7] as u32),
        })
    }
}

impl TryFrom<&[u8]> for AirplayTargetMessage {
    type Error = crate::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::decode(value)
    }
}

impl HandoffMessage {
    pub fn decode(data: &[u8]) -> Result<HandoffMessage, Error> {
        if data.len() < 2 {
            return Err(Error::new(
                ErrorKind::DecodeError,
                "Length mismatch. Cannot read opcode + length",
            ));
        }

        let opcode = data[0];
        if opcode != 0x0C {
            return Err(Error::new(
                ErrorKind::DecodeError,
                "Tried to decode Handoff message with invalid opcode. Expected 0x0C",
            ));
        }

        let length = data[1] as usize;
        if data.len() < length + 2 {
            return Err(Error::new(
                ErrorKind::DecodeError,
                "Packet length != buffer length",
            ));
        }

        let mut encrypted_payload = vec![0; length - 6];
        encrypted_payload.copy_from_slice(&data[6..length]);

        Ok(HandoffMessage {
            header: MessageHeader { opcode, length },
            clipboard_status: data[2],
            iv: ((data[3] as u16) << 8) + (data[4] as u16),
            gcm_auth: data[5],
            encrypted_payload,
        })
    }
}

impl TryFrom<&[u8]> for HandoffMessage {
    type Error = crate::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::decode(value)
    }
}

impl NearbyInfoMessage {
    pub fn decode(data: &[u8]) -> Result<NearbyInfoMessage, Error> {
        if data.len() < 2 {
            return Err(Error::new(
                ErrorKind::DecodeError,
                "Length mismatch. Cannot read opcode + length",
            ));
        }

        let opcode = data[0];
        if opcode != 0x10 {
            return Err(Error::new(
                ErrorKind::DecodeError,
                "Tried to decode Nearby Info message with invalid opcode. Expected 0x10",
            ));
        }

        let length = data[1] as usize;
        if data.len() < length + 2 {
            return Err(Error::new(
                ErrorKind::DecodeError,
                "Packet length != buffer length",
            ));
        }

        Ok(NearbyInfoMessage {
            header: MessageHeader { opcode, length },
            status_flags: data[2] >> 4,
            action_code: data[2] & 0x0F,
            data_flags: data[3],
            auth_tag: ((data[4] as u32) << 16) + ((data[4] as u32) << 8) + (data[4] as u32),
        })
    }
}

impl TryFrom<&[u8]> for NearbyInfoMessage {
    type Error = crate::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::decode(value)
    }
}
