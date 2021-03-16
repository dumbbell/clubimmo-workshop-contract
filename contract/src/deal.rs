use std::collections::HashMap;
use serde_json;
use serde::{Serialize, Deserialize};
use casper_types::{
    account::AccountHash,
    bytesrepr::{self, ToBytes, FromBytes},
    CLTyped, CLType,
};


#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Deal {
    pub address: String,
    pub asked_price: u64,
    pub offers: HashMap<AccountHash, u64>,
}

impl CLTyped for Deal {
    fn cl_type() -> CLType {
        CLType::Any
    }
}

impl ToBytes for Deal {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut result = bytesrepr::allocate_buffer(self)?;
        let json = serde_json::to_string(self).unwrap();
        result.extend(json.to_bytes()?);
        Ok(result)
    }

    fn serialized_length(&self) -> usize {
        let json = serde_json::to_string(self).unwrap();
        json.as_bytes().len()
    }
}

impl FromBytes for Deal {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (json, bytes): (String, &[u8]) = FromBytes::from_bytes(bytes)?;
        let deal: Deal = serde_json::from_str(json.as_str()).unwrap();
        Ok((deal, bytes))
    }
}
