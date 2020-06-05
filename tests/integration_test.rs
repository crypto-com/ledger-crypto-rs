/*******************************************************************************
*   (c) 2018-2020 Zondax GmbH
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
#![deny(warnings, trivial_casts, trivial_numeric_casts)]
#![deny(unused_import_braces, unused_qualifications)]
#![deny(missing_docs)]

extern crate hex;
#[macro_use]
extern crate matches;
#[macro_use]
extern crate serial_test;
extern crate ledger_crypto;

#[cfg(test)]
mod integration_tests {
    use futures_await_test::async_test;
    use ledger_crypto::{APDUTransport, CryptoApp};
    use zx_bip44::BIP44Path;

    fn init_logging() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[ignore = "not yet implemented"]
    #[async_test]
    #[serial]
    async fn version() {
        init_logging();

        log::info!("Test");

        let transport = APDUTransport {
            transport_wrapper: ledger::TransportNativeHID::new().unwrap(),
        };
        let app = CryptoApp::new(transport);

        let resp = app.get_version().await.unwrap();

        println!("mode  {}", resp.mode);
        println!("major {}", resp.major);
        println!("minor {}", resp.minor);
        println!("patch {}", resp.patch);
        println!("locked {}", resp.locked);

        assert!(resp.major == 0);
    }

    // #[async_test]
    // #[serial]
    // async fn address_transfer() {
    //     init_logging();
    //
    //     let transport = APDUTransport {
    //         transport_wrapper: ledger::TransportNativeHID::new().unwrap(),
    //     };
    //     let app = CryptoApp::new(transport);
    //
    //     let path = BIP44Path::from_string("m/44'/394'/0'/0/0").unwrap();
    //     let resp = app.get_address(&path, false).await.unwrap();
    //
    //     assert_eq!(resp.public_key.len(), 65);
    //
    //     let pkhex = hex::encode(&resp.public_key[..]);
    //     println!("Public Key   {:?}", pkhex);
    //     println!("Address address {:?}", resp.address);
    //
    //     assert_eq!(pkhex, "");
    //     assert_eq!(resp.address, "");
    // }

    #[async_test]
    #[serial]
    async fn address_staking() {
        init_logging();

        let transport = APDUTransport {
            transport_wrapper: ledger::TransportNativeHID::new().unwrap(),
        };
        let app = CryptoApp::new(transport);

        let path = BIP44Path::from_string("m/44'/394'/1'/0/0").unwrap();
        let resp = app.get_address(&path, false).await.unwrap();

        assert_eq!(resp.public_key.len(), 65);

        let pkhex = hex::encode(&resp.public_key[..]);
        println!("Public Key   {:?}", pkhex);
        println!("Address address {:?}", resp.address);

        assert_eq!(
            pkhex,
            "04eda422888bff3b3fa957ab9a509b6ae70c1249c9b9b35f1832aeb2e9a4f94b86076054d2641464e55f85e0c6d27d7dcebd60386f6178dec5e77a2a03330952aa"
        );
        assert_eq!(resp.address, "D218B4DE1EF21BE0894BB008F3C7D0D3439C174B");
    }

    // ASYNC TEST DOES NOT ALLOW ignore ATTRIBUTE
    // #[async_test]
    // #[serial]
    // async fn show_address_transfer() {
    //     init_logging();
    //
    //     let transport = APDUTransport {
    //         transport_wrapper: ledger::TransportNativeHID::new().unwrap(),
    //     };
    //     let app = CryptoApp::new(transport);
    //
    //     let path = BIP44Path::from_string("m/44'/394'/0/0/0").unwrap();
    //     let resp = app.get_address(&path, true).await.unwrap();
    //
    //     assert_eq!(resp.public_key.len(), 65);
    //
    //     let pkhex = hex::encode(&resp.public_key[..]);
    //     println!("Public Key   {:?}", pkhex);
    //     println!("Address address {:?}", resp.address);
    //
    //     assert_eq!(pkhex, "");
    //     assert_eq!(resp.address, "");
    // }

    #[async_test]
    #[serial]
    async fn show_address_staking() {
        init_logging();

        let transport = APDUTransport {
            transport_wrapper: ledger::TransportNativeHID::new().unwrap(),
        };
        let app = CryptoApp::new(transport);

        let path = BIP44Path::from_string("m/44'/394'/1'/0/0").unwrap();
        let resp = app.get_address(&path, true).await.unwrap();

        assert_eq!(resp.public_key.len(), 65);

        let pkhex = hex::encode(&resp.public_key[..]);
        println!("Public Key   {:?}", pkhex);
        println!("Address address {:?}", resp.address);

        assert_eq!(
            pkhex,
            "04eda422888bff3b3fa957ab9a509b6ae70c1249c9b9b35f1832aeb2e9a4f94b86076054d2641464e55f85e0c6d27d7dcebd60386f6178dec5e77a2a03330952aa"
        );
        assert_eq!(resp.address, "D218B4DE1EF21BE0894BB008F3C7D0D3439C174B");
    }

    #[async_test]
    #[serial]
    async fn sign_empty() {
        init_logging();

        let transport = APDUTransport {
            transport_wrapper: ledger::TransportNativeHID::new().unwrap(),
        };
        let app = CryptoApp::new(transport);

        let path = BIP44Path::from_string("m/44'/394'/0/0/5").unwrap();
        let some_message0 = b"";

        let response = app.sign(&path, some_message0).await;
        assert!(response.is_err());
        assert!(matches!(
            response.err().unwrap(),
            ledger_crypto::LedgerError::InvalidEmptyMessage
        ));
    }

    // #[async_test]
    // #[serial]
    // async fn sign_verify() {
    //     init_logging();
    //
    //     let transport = APDUTransport {
    //         transport_wrapper: ledger::TransportNativeHID::new().unwrap(),
    //     };
    //     let app = CryptoApp::new(transport);
    //
    //     let path = BIP44Path::from_string("m/44'/394'/0'/0/5").unwrap();
    //
    //     // TODO: Add a test transaction
    //     let txstr = "1234";
    //     let blob = hex::decode(txstr).unwrap();
    //
    //     // First, get public key
    //     let _addr = app.get_address(&path, false).await.unwrap();
    //     let _response = app.sign(&path, &blob).await.unwrap();
    //
    //     // TODO: verify signature is valid
    // }
}
