use crate::{Permissions, SignedPermit};
use cosmwasm_std::{Api, Binary, StdError, StdResult};
use serde::Serialize;
use sha3::{Digest, Keccak256};

pub fn validate_metamask<Permission: Permissions>(
    api: &dyn Api,
    signature: &Binary,
    signed_permit: &SignedPermit<Permission>,
    pubkey: &Binary,
) -> StdResult<bool> {
    let mut signed_bytes = vec![];
    signed_bytes.extend_from_slice(b"\x19Ethereum Signed Message:\n");

    let signed_tx_pretty_amino_json = to_binary_pretty(signed_permit)?;

    signed_bytes.extend_from_slice(signed_tx_pretty_amino_json.len().to_string().as_bytes());
    signed_bytes.extend_from_slice(signed_tx_pretty_amino_json.as_slice());

    let mut hasher = Keccak256::new();

    hasher.update(&signed_bytes);

    let signed_bytes_hash = hasher.finalize();

    let verified = api
        .secp256k1_verify(&signed_bytes_hash, &signature.0, &pubkey.0)
        .map_err(|err| StdError::generic_err(err.to_string()))?;
    Ok(verified)
}

fn to_binary_pretty<T>(data: &T) -> StdResult<Binary>
where
    T: Serialize + ?Sized,
{
    const INDENT: &[u8; 4] = b"    ";
    super::pretty::to_vec_pretty(data, INDENT)
        .map_err(|e| StdError::serialize_err(std::any::type_name::<T>(), e))
        .map(Binary)
}
