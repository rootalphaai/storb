use std::slice;

use crabtensor::{sign::KeypairSignature, AccountId};
use serde::{Deserialize, Serialize};

use crate::NodeUID;

#[derive(Debug, Deserialize, Serialize)]
#[repr(C)]
pub struct KeyRegistrationInfo {
    pub uid: NodeUID,
    pub account_id: AccountId,
}

#[derive(Debug, Deserialize, Serialize)]
#[repr(C)]
pub struct VerificationMessage {
    pub netuid: NodeUID,
    pub miner: KeyRegistrationInfo,
    pub validator: KeyRegistrationInfo,
}

impl AsRef<[u8]> for VerificationMessage {
    fn as_ref(&self) -> &[u8] {
        // NOTE: This is safe as this is aligned with u8, and is repr(C)
        unsafe { slice::from_raw_parts(self as *const _ as *const u8, size_of::<Self>()) }
    }
}

/// The payload containing the message and its signature that is sent to the miner
#[derive(Debug, Deserialize, Serialize)]
#[repr(C)]
pub struct HandshakePayload {
    pub signature: KeypairSignature,
    pub message: VerificationMessage,
}
