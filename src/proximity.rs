use serde::{Deserialize, Serialize};

use crate::messages::ProximityPairMessage;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ProximityDeviceModel {
    Unknown = 0xFFFF,
    Airpods = 0x0220,
    AirpodsPro = 0x0E20,
    AirpodsMax = 0x0A20,
    AirpodsGen2 = 0x0F20,
    AirpodsGen3 = 0x1320,
    AirpodsProGen2 = 0x1420,
    PowerBeats = 0x0320,
    PowerBeatsPro = 0x0B20,
    BeatsSoloPro = 0x0C20,
    BeatsStudioBuds = 0x1120,
    BeatsFlex = 0x1020,
    BeatsX = 0x0520,
    BeatsSolo3 = 0x0620,
    BeatsStudio3 = 0x0920,
    BeatsStudioPro = 0x1720,
    BeatsFitPro = 0x1220,
    BeatsStudioBudsPlus = 0x1620,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ProximityDeviceColor {
    Unknown = 0xFF,
    White = 0x00,
    Black = 0x01,
    Red = 0x02,
    Blue = 0x03,
    Pink = 0x04,
    Gray = 0x05,
    Silver = 0x06,
    Gold = 0x07,
    RoseGold = 0x08,
    SpaceGray = 0x09,
    DarkBlue = 0x0A,
    LightBlue = 0x0B,
    Yellow = 0x0C,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Status {
    Off = 0x00,
    InEar = 0x01,
    InCase = 0x02,
    AirplaneMode = 0x03,
    Unknown = 0xFF,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct BatteryState {
    pub case: u32,
    pub left: u32,
    pub right: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ChargingState {
    pub case: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct DeviceState {
    pub case_open: bool,
    pub left: Status,
    pub right: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ProximityDevice {
    pub model: ProximityDeviceModel,
    pub color: ProximityDeviceColor,
    pub battery: BatteryState,
    pub charging_state: ChargingState,
    pub state: DeviceState,
}

impl From<u16> for ProximityDeviceModel {
    fn from(val: u16) -> Self {
        match val {
            0x0220 => ProximityDeviceModel::Airpods,
            0x0E20 => ProximityDeviceModel::AirpodsPro,
            0x0A20 => ProximityDeviceModel::AirpodsMax,
            0x0F20 => ProximityDeviceModel::AirpodsGen2,
            0x1320 => ProximityDeviceModel::AirpodsGen3,
            0x1420 => ProximityDeviceModel::AirpodsProGen2,
            0x0320 => ProximityDeviceModel::PowerBeats,
            0x0B20 => ProximityDeviceModel::PowerBeatsPro,
            0x0C20 => ProximityDeviceModel::BeatsSoloPro,
            0x1120 => ProximityDeviceModel::BeatsStudioBuds,
            0x1020 => ProximityDeviceModel::BeatsFlex,
            0x0520 => ProximityDeviceModel::BeatsX,
            0x0620 => ProximityDeviceModel::BeatsSolo3,
            0x0920 => ProximityDeviceModel::BeatsStudio3,
            0x1720 => ProximityDeviceModel::BeatsStudioPro,
            0x1220 => ProximityDeviceModel::BeatsFitPro,
            0x1620 => ProximityDeviceModel::BeatsStudioBudsPlus,
            _ => ProximityDeviceModel::Unknown,
        }
    }
}

impl From<u8> for ProximityDeviceColor {
    fn from(val: u8) -> Self {
        match val {
            0x00 => ProximityDeviceColor::White,
            0x01 => ProximityDeviceColor::Black,
            0x02 => ProximityDeviceColor::Red,
            0x03 => ProximityDeviceColor::Blue,
            0x04 => ProximityDeviceColor::Pink,
            0x05 => ProximityDeviceColor::Gray,
            0x06 => ProximityDeviceColor::Silver,
            0x07 => ProximityDeviceColor::Gold,
            0x08 => ProximityDeviceColor::RoseGold,
            0x09 => ProximityDeviceColor::SpaceGray,
            0x0A => ProximityDeviceColor::DarkBlue,
            0x0B => ProximityDeviceColor::LightBlue,
            0x0C => ProximityDeviceColor::Yellow,
            _ => ProximityDeviceColor::Unknown,
        }
    }
}

impl From<u8> for Status {
    fn from(val: u8) -> Self {
        match val {
            0x00 => Status::Off,
            0x01 => Status::InEar,
            0x02 => Status::InCase,
            0x03 => Status::AirplaneMode,
            _ => Status::Unknown,
        }
    }
}

impl From<u8> for DeviceState {
    fn from(val: u8) -> Self {
        let flipped = (val >> 4) & 0x02 == 0;
        let case_open = val & 0x01 != 0;

        let left_status = if flipped {
            (val >> 3) & 0x03
        } else {
            (val >> 1) & 0x03
        };
        let right_status = if flipped {
            (val >> 1) & 0x03
        } else {
            (val >> 3) & 0x03
        };

        DeviceState {
            case_open,
            left: Status::from(left_status),
            right: Status::from(right_status),
        }
    }
}

impl From<ProximityPairMessage> for ProximityDevice {
    fn from(val: ProximityPairMessage) -> Self {
        let flipped = (val.device_status >> 4) & 0x02 == 0;

        let case_battery = val.battery2 & 0x0F;
        let left_battery = if flipped {
            val.battery1 >> 4
        } else {
            val.battery1 & 0x0F
        };

        let right_battery = if flipped {
            val.battery1 & 0x0F
        } else {
            val.battery1 >> 4
        };

        let charge_status = val.battery2 >> 4;
        let case_charging = (charge_status & 0x04) != 0;
        let left_charging = if flipped {
            (charge_status & 0x02) != 0
        } else {
            (charge_status & 0x01) != 0
        };

        let right_charging = if flipped {
            (charge_status & 0x01) != 0
        } else {
            (charge_status & 0x02) != 0
        };

        ProximityDevice {
            model: ProximityDeviceModel::from(val.device_model),
            color: ProximityDeviceColor::from(val.device_color),
            state: DeviceState::from(val.device_status),
            battery: BatteryState {
                case: case_battery as u32 * 10,
                left: left_battery as u32 * 10,
                right: right_battery as u32 * 10,
            },
            charging_state: ChargingState {
                case: case_charging,
                left: left_charging,
                right: right_charging,
            },
        }
    }
}
