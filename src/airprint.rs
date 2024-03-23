use serde::{Deserialize, Serialize};

use crate::messages::AirPrintMessage;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum AddressType {
    IPv4 = 0x01,
    IPv6 = 0x02,
    Unknown = 0xFF,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum ResourceType {
    ResourcePath = 0x00,
    PrinterId = 0x01,
    Unknown = 0xFF,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum SecurityType {
    None = 0x00,
    TLS = 0x01,
    Unknown = 0xFF,
}

const RESOURCE_PATH_VALUES: [&str; 7] = [
    "/ipp/print",
    "/ipp/printer",
    "/printers/Laserjet",
    "/ipp/port1",
    "/Epson_IPP_Printer",
    "/ipp/ap",
    "/printer",
];

pub struct Printer {
    pub address_type: AddressType,
    pub resource_type: ResourceType,
    pub security_type: SecurityType,
    pub printer_id: Option<u16>,
    pub resource_path: Option<String>,
    pub port: u16,
    pub ipv4_address: Option<u32>,
    pub ipv6_address: Option<u128>,
    pub measured_power: u8,
}

impl From<u8> for AddressType {
    fn from(value: u8) -> Self {
        match value >> 4 {
            0x01 => AddressType::IPv4,
            0x02 => AddressType::IPv6,
            _ => AddressType::Unknown,
        }
    }
}

impl From<u8> for ResourceType {
    fn from(value: u8) -> Self {
        match (value >> 2) & 0x03 {
            0x00 => ResourceType::ResourcePath,
            0x01 => ResourceType::PrinterId,
            _ => ResourceType::Unknown,
        }
    }
}

impl From<u8> for SecurityType {
    fn from(value: u8) -> Self {
        match value & 0x03 {
            0x00 => SecurityType::None,
            0x01 => SecurityType::TLS,
            _ => SecurityType::Unknown,
        }
    }
}

impl From<AirPrintMessage> for Printer {
    fn from(value: AirPrintMessage) -> Self {
        let address_type = AddressType::from(value.connection_info);
        let resource_type = ResourceType::from(value.connection_info);

        Printer {
            address_type,
            resource_type,
            security_type: SecurityType::from(value.connection_info),
            printer_id: match resource_type {
                ResourceType::PrinterId => Some(value.rp_index_value),
                _ => None,
            },
            resource_path: match resource_type {
                ResourceType::ResourcePath => {
                    if (value.rp_index_value as usize) < RESOURCE_PATH_VALUES.len() {
                        Some(RESOURCE_PATH_VALUES[value.rp_index_value as usize].to_string())
                    } else {
                        None
                    }
                }
                _ => None,
            },
            ipv4_address: match address_type {
                AddressType::IPv4 => Some(value.ip4_address),
                _ => None,
            },
            ipv6_address: match address_type {
                AddressType::IPv6 => Some((value.ip4_address as u128) << 96 + value.ip6_address),
                _ => None,
            },
            port: value.port,
            measured_power: value.measured_power,
        }
    }
}
