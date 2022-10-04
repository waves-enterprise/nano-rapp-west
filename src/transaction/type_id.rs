/// Transaction type
#[derive(PartialEq)]
pub enum Type {
    Issue = 3,
    Transfer = 4,
    Reissue = 5,
    Burn = 6,
    Lease = 8,
    LeaseCancel = 9,
    Alias = 10,
    MassTransfer = 11,
    Data = 12,
    SetScript = 13,
    Sponsor = 14,
    SetAssetScript = 15,
    CreateContract = 103,
    Undefined,
}

impl Type {
    pub fn from_u8(value: u8) -> Type {
        match value {
            3u8 => Type::Issue,
            4u8 => Type::Transfer,
            5u8 => Type::Reissue,
            6u8 => Type::Burn,
            8u8 => Type::Lease,
            9u8 => Type::LeaseCancel,
            10u8 => Type::Alias,
            11u8 => Type::MassTransfer,
            12u8 => Type::Data,
            13u8 => Type::SetScript,
            14u8 => Type::Sponsor,
            15u8 => Type::SetAssetScript,
            103u8 => Type::CreateContract,
            _ => Type::Undefined,
        }
    }
}
