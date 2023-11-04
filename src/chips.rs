//! The chip DB.
//! This numbers are from `GetCHIPID` fn in EVT code.

pub fn chip_id_to_chip_name(chip_id: u32) -> Option<&'static str> {
    match chip_id & 0xFFF00000 {
        0x650_00000 => Some("CH565"),
        0x690_00000 => Some("CH569"),
        0x710_00000 => Some("CH571"),
        0x730_00000 => Some("CH573"),
        0x810_00000 => Some("CH581"),
        0x820_00000 => Some("CH582"),
        0x830_00000 => Some("CH583"),
        0x920_00000 => Some("CH592"),
        0x003_00000 => match chip_id & 0xFFFFFF0F {
            0x003_00500 => Some("CH32V003F4P6"),
            0x003_10500 => Some("CH32V003F4U6"),
            0x003_20500 => Some("CH32V003A4M6"),
            0x003_30500 => Some("CH32V003J4M6"),
            _ => None,
        },
        0x035_00000 => match chip_id & 0xFFFFFF0F {
            0x035_00601 => Some("CH32X035R8T6"),
            0x035_10601 => Some("CH32X035C8T6"),
            0x035_E0601 => Some("CH32X035F8U6"),
            0x035_60601 => Some("CH32X035G8U6"),
            0x035_B0601 => Some("CH32X035G8R6"),
            0x035_70601 => Some("CH32X035F7P6"),
            0x035_A0601 => Some("CH32X033F8P6"),
            _ => None,
        },
        0x103_00000 => match chip_id & 0xFFFFFF0F {
            0x103_00700 => Some("CH32L103C8U6"),
            0x103_10700 => Some("CH32L103C8T6"),
            0x103_A0700 => Some("CH32L103F8P6"),
            0x103_B0700 => Some("CH32L103G8R6"),
            0x103_20700 => Some("CH32L103K8U6"),
            0x103_D0700 => Some("CH32L103F8U6"),
            0x103_70700 => Some("CH32L103F7P6"),
            _ => None,
        },
        0x200_00000 => match chip_id {
            0x200_04102 => Some("CH32F103C8T6"),
            0x200_0410F => Some("CH32F103R8T6"),
            _ => None,
        },
        0x203_00000 => match chip_id & 0xFFFFFF0F {
            0x203_00500 => Some("CH32V203C8U6"),
            0x203_10500 => Some("CH32V203C8T6"),
            0x203_20500 => Some("CH32V203K8T6"),
            0x203_30500 => Some("CH32V203C6T6"),
            0x203_50500 => Some("CH32V203K6T6"),
            0x203_60500 => Some("CH32V203G6U6"),
            0x203_B0500 => Some("CH32V203G8R6"),
            0x203_E0500 => Some("CH32V203F8U6"),
            0x203_70500 => Some("CH32V203F6P6"),
            0x203_90500 => Some("CH32V203F6P6"),
            0x203_A0500 => Some("CH32V203F8P6"),
            0x203_4050C => Some("CH32V203RBT6"),
            _ => None,
        },
        0x208_00000 => match chip_id & 0xFFFFFF0F {
            0x208_0050C => Some("CH32V208WBU6"),
            0x208_1050C => Some("CH32V208RBT6"),
            0x208_2050C => Some("CH32V208CBU6"),
            0x208_3050C => Some("CH32V208GBU6"),
            _ => None,
        },
        0x303_00000 | 0x305_00000 | 0x307_00000 => match chip_id & 0xFFFFFF0F {
            0x303_30504 => Some("CH32V303CBT6"),
            0x303_20504 => Some("CH32V303RBT6"),
            0x303_10504 => Some("CH32V303RCT6"),
            0x303_00504 => Some("CH32V303VCT6"),
            0x305_20508 => Some("CH32V305FBP6"),
            0x305_00508 => Some("CH32V305RBT6"),
            0x305_B0508 => Some("CH32V305GBU6"),
            0x307_30508 => Some("CH32V307WCU6"),
            0x307_20508 => Some("CH32V307FBP6"),
            0x307_10508 => Some("CH32V307RCT6"),
            0x307_00508 => Some("CH32V307VCT6"),
            _ => None,
        },
        0x641 => match chip_id & 0xFFFFFF0F {
            0x641_00500 => Some("CH641F"),
            0x641_10500 => Some("CH641D"),
            0x641_50500 => Some("CH641U"),
            0x641_60500 => Some("CH641P"),
            _ => None,
        },
        0x643_00000 => match chip_id {
            0x643_00601 => Some("CH643W"),
            0x643_10601 => Some("CH643Q"),
            0x643_30601 => Some("CH643L"),
            0x643_40601 => Some("CH643U"),
            _ => None,
        },
        _ => None,
    }
}
