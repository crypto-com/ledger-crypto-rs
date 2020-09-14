/*******************************************************************************
*   (c) 2020 Zondax GmbH
*
*  Licensed under the Apache License, Version 2.0 (the "License");
*  you may not use this file except in compliance with the License.
*  You may obtain a copy of the License at
*
*      http://www.apache.org/licenses/LICENSE-2.0
*
*  Unless required by applicable law or agreed to in writing, software
*  distributed under the License is distributed on an "AS IS" BASIS,
*  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*  See the License for the specific language governing permissions and
*  limitations under the License.
********************************************************************************/
//! Support library for Crypto Ledger Nano S/X apps

#![deny(warnings, trivial_casts, trivial_numeric_casts)]
#![deny(unused_import_braces, unused_qualifications)]
#![deny(missing_docs)]

use ledger_transport::{APDUCommand, APDUErrorCodes, APDUTransport, Exchange};
use ledger_zondax_generic::{
    map_apdu_error_description, AppInfo, ChunkPayloadType, DeviceInfo, LedgerAppError, Version,
};
use std::str;
use zx_bip44::BIP44Path;

extern crate hex;

const CLA: u8 = 0x08;
const INS_GET_ADDR_SECP256K1: u8 = 0x01;
const INS_SIGN_SECP256K1: u8 = 0x02;

const PK_LEN: usize = 65;

/// Ledger App
pub struct CryptoApp<T: Exchange> {
    apdu_transport: APDUTransport<T>,
}

type PublicKey = [u8; PK_LEN];

/// Kusama address (includes pubkey and the corresponding ss58 address)
#[allow(dead_code)]
pub struct Address {
    /// Public Key
    pub public_key: PublicKey,
    /// Address (exposed as SS58)
    pub address: String,
}

type Signature = [u8; 65];

impl<T: Exchange> CryptoApp<T> {
    /// Connect to the Ledger App
    pub fn new(apdu_transport: APDUTransport<T>) -> Self {
        CryptoApp { apdu_transport }
    }

    fn cla(&self) -> u8 {
        CLA
    }

    /// Retrieve the app version
    pub async fn get_version(&self) -> Result<Version, LedgerAppError> {
        ledger_zondax_generic::get_version(self.cla(), &self.apdu_transport).await
    }

    /// Retrieve the app info
    pub async fn get_app_info(&self) -> Result<AppInfo, LedgerAppError> {
        ledger_zondax_generic::get_app_info(&self.apdu_transport).await
    }

    /// Retrieve the device info
    pub async fn get_device_info(&self) -> Result<DeviceInfo, LedgerAppError> {
        ledger_zondax_generic::get_device_info(&self.apdu_transport).await
    }

    /// Retrieves the public key and address
    pub async fn get_address(
        &self,
        path: &BIP44Path,
        require_confirmation: bool,
    ) -> Result<Address, LedgerAppError> {
        let serialized_path = path.serialize();
        let p1 = if require_confirmation { 1 } else { 0 };

        let command = APDUCommand {
            cla: self.cla(),
            ins: INS_GET_ADDR_SECP256K1,
            p1,
            p2: 0x00,
            data: serialized_path,
        };

        let response = self.apdu_transport.exchange(&command).await?;
        if response.retcode != 0x9000 {
            return Err(LedgerAppError::AppSpecific(
                response.retcode,
                map_apdu_error_description(response.retcode).to_string(),
            ));
        }

        if response.data.len() < PK_LEN {
            return Err(LedgerAppError::InvalidPK);
        }

        log::info!("Received response {}", response.data.len());

        let mut address = Address {
            public_key: [0; PK_LEN],
            address: "".to_string(),
        };

        address.public_key.copy_from_slice(&response.data[..65]);
        address.address = str::from_utf8(&response.data[65..])
            .map_err(|_e| LedgerAppError::Utf8)?
            .to_owned();

        Ok(address)
    }

    /// Sign a transaction
    pub async fn sign(
        &self,
        path: &BIP44Path,
        message: &[u8],
    ) -> Result<Signature, LedgerAppError> {
        let serialized_path = path.serialize();
        let start_command = APDUCommand {
            cla: self.cla(),
            ins: INS_SIGN_SECP256K1,
            p1: ChunkPayloadType::Init as u8,
            p2: 0x00,
            data: serialized_path,
        };

        log::info!("sign ->");
        let response =
            ledger_zondax_generic::send_chunks(&self.apdu_transport, &start_command, message)
                .await?;
        log::info!("sign OK");

        if response.data.is_empty() && response.retcode == APDUErrorCodes::NoError as u16 {
            return Err(LedgerAppError::NoSignature);
        }

        // Last response should contain the answer
        if response.data.len() < 65 {
            return Err(LedgerAppError::InvalidSignature);
        }

        log::info!("{}", hex::encode(&response.data[..]));

        let mut sig: Signature = [0u8; 65];
        sig.copy_from_slice(&response.data[..65]);

        Ok(sig)
    }
}
