use super::account::Address;
use super::hash::{Asset, TransactionId};

/// Data specific to a particular transaction type
pub enum TransactionData<'a> {
    Issue {
        name: &'a str,
        description: &'a str,
        quantity: u64,
        decimals: u8,
        reissuable: bool,
        chain_id: u8,
        script: Option<&'a [u8]>,
    },
    Transfer {
        recipient: Address,
        asset: Option<Asset>,
        amount: u64,
        fee_asset: Option<Asset>,
        attachment: Option<&'a str>,
    },
    Reissue {
        asset: Asset,
        quantity: u64,
        reissuable: bool,
        chain_id: u8,
    },
    Burn {
        asset: Asset,
        quantity: u64,
        chain_id: u8,
    },
    Lease {
        recipient: Address,
        amount: u64,
        chain_id: u8,
    },
    CancelLease {
        lease_id: TransactionId,
        chain_id: u8,
    },
    Alias {
        alias: &'a str,
        chain_id: u8,
    },
    MassTransfer,
    Data,
    SetScript {
        script: Option<&'a [u8]>,
        chain_id: u8,
    },
    Sponsor {
        asset: Asset,
        rate: Option<u64>,
    },
    SetAssetScript {
        asset: Asset,
        script: Option<&'a [u8]>,
        chain_id: u8,
    },
}
