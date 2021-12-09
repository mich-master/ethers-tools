use ethers_core::abi::ethereum_types::Address as AddressEth;
use ethers_core::types::U256;
use ethers_core::utils;
use ethers_signers::{ Signer, LocalWallet };

fn ethers_to_web3_address(address_ethers: AddressEth) -> web3::types::Address {
    web3::types::H160(address_ethers.0)
}

pub struct EthersUtils {
    address: AddressEth,
}

impl EthersUtils {
    pub fn new(private_key: &str) -> Self {
        let wallet: LocalWallet = private_key.parse::<LocalWallet>().unwrap();
        let address: AddressEth = wallet.address();
        EthersUtils {
            address: address,
        }
    }
    pub fn get_contract_address(&self, nonce: web3::types::U256) -> web3::types::Address {
        let nonce_ethers: U256 = U256(nonce.0);
        let address_ethers: AddressEth = utils::get_contract_address(self.address,nonce_ethers);
        ethers_to_web3_address(address_ethers)
    }
}
