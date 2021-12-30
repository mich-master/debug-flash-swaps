use std::fmt;
use std::str::FromStr;
// use std::fs::File;
// use std::io::Read;
// use std::time::{ SystemTime, Duration };

// use serde_json::{ to_vec, Value };

use secp256k1::{ SecretKey };

use web3::types::{ Address, U256 };
// use web3::contract::{ Contract, Options };
use web3::contract::{ Options };
use ethers_tools::EthersUtils;

// use web3_tools::{ AsEip55 };

use neonevm_sdk::{
    // network::Network,
    types::{ EthAddress, ExpandToDecimals, Erc20Specs, Erc20Deploy, Erc20DeploySpecs },
};

// use crate::ContractPaths;


// pub fn get_contract_from_abi_file(web3: &web3::Web3<web3::transports::Http>, abi_file_path: &str, eth_contract_address: EthAddress) -> Result<Contract<web3::transports::Http>,()> {

//     // open the abi file
//     let abi = File::open(abi_file_path);
//     if abi.is_err() {
//         println!("Failed to open {}\n", abi_file_path);
//         return Err(());
//     }

//     // read the abi file
//     let mut abi_data = String::new();
//     let bytes_read = abi.unwrap().read_to_string(&mut abi_data);
//     if bytes_read.is_err() {
//         println!("Failed to read from {}\n", abi_file_path);
//         return Err(());
//     }

//     let lib: Value = serde_json::from_str(&abi_data).unwrap();
//     let lib_abi: Vec<u8> = to_vec(&lib["abi"]).unwrap();

//     Contract::from_json(web3.eth(), eth_contract_address.into(), &lib_abi).map_err(|_|())
// }

pub enum Erc20Means {
    Origin,
    Bridge(Erc20Deploy),
}

pub struct Erc20Token {
    pub specs: Erc20Specs,
    pub eth_address: EthAddress,
    pub means: Erc20Means,
}

impl<'a> Erc20Token {
    // pub fn get_abi_path(&self, paths: &'a ContractPaths) -> &'a str {
    //     match self.means {
    //         Erc20Means::Origin    => &paths.erc20,
    //         Erc20Means::Bridge(_) => &paths.erc20wrapper,
    //     }
    // }
    pub fn get_name(&self) -> &str {
        &self.specs.name
    }
    pub fn get_symbol(&self) -> &str {
        &self.specs.symbol
    }
    // pub fn get_address(&self) -> Address {
    //     (&self.eth_address).as_ref().clone()
    // }
}

impl From<Erc20DeploySpecs> for Erc20Token {
    fn from(f: Erc20DeploySpecs) -> Erc20Token {
        Erc20Token {
            specs: f.specs,
            eth_address: EthAddress::from_str(&f.deploy.neonevm_erc20token_address).unwrap(),
            means: Erc20Means::Bridge(f.deploy),
        }
    }
}

impl fmt::Display for Erc20Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{} [ {} ]", self.get_name(), self.get_symbol())
    }
}

impl fmt::Debug for Erc20Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variant =
            match self.means {
                Erc20Means::Origin    => "Origin",
                Erc20Means::Bridge(_) => "Bridge",
            };
        write!(f,"ERC20 {}: {} [ {} ] ; Contract Address: {}", variant, self.get_name(), self.get_symbol(), self.eth_address)
    }
}

pub struct WethToken {
    pub eth_address: EthAddress,
}

impl WethToken {
    pub fn new(a: &str) -> WethToken {
        WethToken {
            eth_address: EthAddress::from_str(a).unwrap(),
        }
    }
    // pub fn get_address(&self) -> Address {
    //     (&self.eth_address).as_ref().clone()
    // }
}

impl fmt::Display for WethToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Wrapped Ether [ WETH ]")
    }
}

impl fmt::Debug for WethToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"ERC20 Wrapped Ether [ WETH ] ; Contract Address: {}", self.eth_address)
    }
}


pub enum SwapToken {
    Weth(WethToken),
    Erc20(Erc20Token),
}

impl<'a> SwapToken {
    // pub fn get_abi_path(&self, paths: &'a ContractPaths) -> &'a str {
    //     match self {
    //         SwapToken::Weth(_)      => &paths.weth9,
    //         SwapToken::Erc20(erc20) => erc20.get_abi_path(paths),
    //     }
    // }
    // pub fn get_name(&self) ->  &str {
    //     match self {
    //         SwapToken::Weth(_)      => "Neon Token",
    //         SwapToken::Erc20(erc20) => erc20.get_name(),
    //     }
    // }
    // pub fn get_symbol(&self) ->  &str {
    //     match self {
    //         SwapToken::Weth(_)      => "NEON",
    //         SwapToken::Erc20(erc20) => erc20.get_symbol(),
    //     }
    // }
    // pub fn get_address(&self) ->  Address {
    //     match self {
    //         SwapToken::Weth(t)  => t.get_address(),
    //         SwapToken::Erc20(t) => t.get_address(),
    //     }
    // }
    pub fn expand_from_uint(&self, amount: u32) -> u128 {
        match self {
            SwapToken::Weth(_)  => amount.expand_to_decimals(18).unwrap(),
            SwapToken::Erc20(t) => amount.expand_to_decimals(t.specs.decimals).unwrap(),
        }
    }
}

impl From<Erc20DeploySpecs> for SwapToken {
    fn from(f: Erc20DeploySpecs) -> SwapToken {
        SwapToken::Erc20(Erc20Token::from(f))
    }
}

impl fmt::Debug for SwapToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SwapToken::Weth(t)  => t.fmt(f),
            SwapToken::Erc20(t) => t.fmt(f),
        }
    }
}


pub struct NeonswapEnvironment {
    pub web3: web3::Web3<web3::transports::Http>,
    // contract_paths: ContractPaths,
    pub signing_key: SecretKey,
    pub signing_address: Address,
    // eth_weth9_address: EthAddress,
    // eth_uniswap_v2factory_address: EthAddress,
    // pub uniswap_v1exchange: Contract<web3::transports::Http>,
    // uniswap_v1factory: Contract<web3::transports::Http>,
    // uniswap_v2factory: Contract<web3::transports::Http>,
    // eth_uniswap_v2router02_address: EthAddress,
    // uniswap_v2router02: Contract<web3::transports::Http>,
}

impl NeonswapEnvironment {
    pub fn default_web3_options(&self) -> Options {
        Options::with(|o|{
            o.gas = Some(U256::from(3000000));
            o.gas_price = Some(U256::from(1000000000));
        })
    }
    pub fn new(web3: web3::Web3<web3::transports::Http>,
        // contract_paths: ContractPaths,
        signing_key_string: &str,
        signing_key_utils: EthersUtils,
        // uniswap_v1_exchange_address: Address,
        // uniswap_v1_factory_address: Address,
        // uniswap_v2_factory_address: Address,
        // uniswap_v2_router02_address: Address,
    ) -> Self {

        // let eth_uniswap_v1exchange_address: EthAddress = EthAddress::from_str(&uniswap_v1_exchange_address.as_eip55()).unwrap();
        // let eth_uniswap_v1factory_address: EthAddress = EthAddress::from_str(&uniswap_v1_factory_address.as_eip55()).unwrap();
        // let eth_uniswap_v2factory_address: EthAddress = EthAddress::from_str(&uniswap_v2_factory_address.as_eip55()).unwrap();
        // let eth_uniswap_v2router02_address: EthAddress = EthAddress::from_str(&uniswap_v2_router02_address.as_eip55()).unwrap();

        // let uniswap_v1exchange = get_contract_from_abi_file(&web3, &contract_paths.uniswap_v1exchange, eth_uniswap_v1exchange_address.clone()).unwrap();
        // let uniswap_v1factory = get_contract_from_abi_file(&web3, &contract_paths.uniswap_v1factory, eth_uniswap_v1factory_address.clone()).unwrap();
        // let uniswap_v2factory = get_contract_from_abi_file(&web3, &contract_paths.uniswap_v2factory, eth_uniswap_v2factory_address.clone()).unwrap();
        // let uniswap_v2router02 = get_contract_from_abi_file(&web3, &contract_paths.uniswap_v2router02, eth_uniswap_v2router02_address.clone()).unwrap();
        NeonswapEnvironment {
            web3,
            // contract_paths,
            signing_key: SecretKey::from_str(signing_key_string).unwrap(),
            signing_address: signing_key_utils.address(),
            // eth_uniswap_v2factory_address,
            // uniswap_v1exchange,
            // uniswap_v1factory,
            // uniswap_v2factory,
            // eth_uniswap_v2router02_address,
            // uniswap_v2router02,
        }
    }
//     async fn get_pair(&self, token_a: &SwapToken, token_b: &SwapToken) -> EthAddress {

//         let token_a_address: Address = token_a.get_address();
//         let token_b_address: Address = token_b.get_address();

//         let uniswap_v2_get_pair_ab_address: Address = 
//             self.uniswap_v2factory.query(
//                     "getPair",
//                     (token_a_address,token_b_address),
//                     self.signing_address,
//                     self.default_web3_options(),
//                     None,
//                 )
//                 .await
//                 .unwrap();
        
//         if !uniswap_v2_get_pair_ab_address.is_zero() {
//             println!("{} [ {} ] <-> {} [ {} ] Pair Address: {:?}", token_a.get_name(), token_a.get_symbol(), token_b.get_name(), token_b.get_symbol(), uniswap_v2_get_pair_ab_address);
//         };

//         uniswap_v2_get_pair_ab_address.into()
//     }
//     async fn pair_exists(&self, token_a: &SwapToken, token_b: &SwapToken) -> bool {
//         self.get_pair(token_a, token_b)
//             .await
//             .is_zero()
//     }
//     async fn create_pair(&self, token_a: &SwapToken, token_b: &SwapToken) {

//         let token_a_address: Address = token_a.get_address();
//         let token_b_address: Address = token_b.get_address();

//         let create_pair = 
//             self.uniswap_v2factory.signed_call_with_confirmations(
//                     "createPair",
//                     (token_a_address,token_b_address),
//                     self.default_web3_options(),
//                     0,
//                     &self.signing_key
//                 )
//                 .await;
        
//         match create_pair {
//             Ok(receipt) => {
//                 let status: u64 = receipt.status.map(|s| s.as_u64()).unwrap_or(0u64);
//                 if status > 0 {
//                     println!("{} [ {} ] <-> {} [ {} ] Pair Created!", token_a.get_name(), token_a.get_symbol(), token_b.get_name(), token_b.get_symbol());
//                 } else {
//                     println!("Create Pair {} [ {} ] <-> {} [ {} ] Declined with Receipt:\n{:?}", token_a.get_name(), token_a.get_symbol(), token_b.get_name(), token_b.get_symbol(), receipt);
//                 }
//             },
//             Err(error) => {
//                 println!("Create Pair {} [ {} ] <-> {} [ {} ] failed with Error:\n{:?}", token_a.get_name(), token_a.get_symbol(), token_b.get_name(), token_b.get_symbol(), error);
//             },
//         }

//     }
//     async fn approve_liquidity(&self, erc20_token: &Erc20Token, amount: u128) {

//         let contract = get_contract_from_abi_file(&self.web3, erc20_token.get_abi_path(&self.contract_paths), erc20_token.get_address().into()).unwrap();

//         let uniswap_v2router02_address: Address = (&self.eth_uniswap_v2router02_address).as_ref().clone();
//         let amount_u256: U256 = U256::from(amount);

//         let approve = 
//             contract.signed_call_with_confirmations(
//                     "approve",
//                     (uniswap_v2router02_address, amount_u256),
//                     self.default_web3_options(),
//                     0,
//                     &self.signing_key
//                 )
//                 .await;

//         match approve {
//             Ok(receipt) => {
//                 let status: u64 = receipt.status.map(|s| s.as_u64()).unwrap_or(0u64);
//                 if status > 0 {
//                     println!("{} [ {} ] : {}  {} -> {} Approved!", erc20_token.get_name(), erc20_token.get_symbol(), amount, EthAddress::from(self.signing_address), self.eth_uniswap_v2router02_address);
//                 } else {
//                     println!("Approve {} [ {} ] : {}  {} -> {} Declined with Receipt:\n{:?}", erc20_token.get_name(), erc20_token.get_symbol(), amount, EthAddress::from(self.signing_address), self.eth_uniswap_v2router02_address, receipt);
//                 }
//             },
//             Err(error) => {
//                 println!("Approve {} [ {} ] : {}  {} -> {} failed with Error:\n{:?}", erc20_token.get_name(), erc20_token.get_symbol(), amount, EthAddress::from(self.signing_address), self.eth_uniswap_v2router02_address, error);
//             },
//         }
//     }
//     async fn add_liquidity(&self, token_a: &Erc20Token, token_b: &Erc20Token, amount_a: u128, amount_b: u128) {

//         let token_a_address: Address = token_a.get_address();
//         let token_b_address: Address = token_b.get_address();

//         let amount_a_u256: U256 = U256::from(amount_a);
//         let amount_b_u256: U256 = U256::from(amount_b);

//         let deadline = SystemTime::now()
//             .checked_add(Duration::from_secs(3600))
//             .unwrap()
//             .duration_since(SystemTime::UNIX_EPOCH)
//             .unwrap()
//             .as_secs()
//             * 1000;
//         let deadline_u256: U256 = U256::from(deadline);

//         let add_liquidity = 
//             self.uniswap_v2router02.signed_call_with_confirmations(
//                     "addLiquidity",
//                     (token_a_address, token_b_address, amount_a_u256, amount_b_u256, amount_a_u256, amount_b_u256, self.signing_address, deadline_u256),
//                     self.default_web3_options(),
//                     0,
//                     &self.signing_key,
//                 )
//                 .await;

//         match add_liquidity {
//             Ok(receipt) => {
//                 let status: u64 = receipt.status.map(|s| s.as_u64()).unwrap_or(0u64);
//                 if status > 0 {
//                     println!("{} [ {} ] : {} <-> {} [ {} ] : {} Liquidity Added", token_a.get_name(), token_a.get_symbol(), amount_a, token_b.get_name(), token_b.get_symbol(), amount_b);
//                 } else {
//                     println!("Add Liquidity {} [ {} ] : {} <-> {} [ {} ] : {} Declined with Receipt:\n{:?}", token_a.get_name(), token_a.get_symbol(), amount_a, token_b.get_name(), token_b.get_symbol(), amount_b, receipt);
//                 }
//             },
//             Err(error) => {
//                 println!("Add Liquidity {} [ {} ] : {} <-> {} [ {} ] : {} failed with Error:\n{:?}", token_a.get_name(), token_a.get_symbol(), amount_a, token_b.get_name(), token_b.get_symbol(), amount_b, error);
//             },
//         }
//     }
//     async fn add_liquidity_eth(&self, token_b: &Erc20Token, amount_weth: u128, amount_b: u128) {

//         let token_b_address: Address = token_b.get_address();

//         let amount_weth_u256: U256 = U256::from(amount_weth);
//         let amount_b_u256: U256 = U256::from(amount_b);

//         let deadline = SystemTime::now()
//             .checked_add(Duration::from_secs(3600))
//             .unwrap()
//             .duration_since(SystemTime::UNIX_EPOCH)
//             .unwrap()
//             .as_secs()
//             * 1000;
//         let deadline_u256: U256 = U256::from(deadline);

//         let mut options = self.default_web3_options();
//         options.value = Some(amount_weth_u256);

//         let add_liquidity_eth = 
//             self.uniswap_v2router02.signed_call_with_confirmations(
//                     "addLiquidityETH",
//                     (token_b_address, amount_b_u256, amount_b_u256, amount_weth, self.signing_address, deadline_u256),
//                     options,
//                     0,
//                     &self.signing_key,
//                 )
//                 .await;

//         match add_liquidity_eth {
//             Ok(receipt) => {
//                 let status: u64 = receipt.status.map(|s| s.as_u64()).unwrap_or(0u64);
//                 if status > 0 {
//                     println!("NEON [ ETH ] : {} <-> {} [ {} ] : {} Liquidity Added!", amount_weth, token_b.get_name(), token_b.get_symbol(), amount_b);
//                 } else {
//                     println!("Add Liquidity NEON [ ETH ] : {} <-> {} [ {} ] : {} Declined with Receipt:\n{:?}", amount_weth, token_b.get_name(), token_b.get_symbol(), amount_b, receipt);
//                 }
//             },
//             Err(error) => {
//                 println!("Add Liquidity NEON [ ETH ] : {} <-> {} [ {} ] : {} failed with Error:\n{:?}", amount_weth, token_b.get_name(), token_b.get_symbol(), amount_b, error);
//             },
//         }
//     }
//     pub async fn create_pair_add_liquidity(&self, token_a: &SwapToken, token_b: &SwapToken, amount_a: u128, amount_b: u128) -> Result<(),()> {

//         if self.pair_exists(token_a, token_b).await {
//             self.create_pair(token_a, token_b)
//                 .await;
//             self.get_pair(token_a, token_b)
//                 .await;
//         }

//         if let SwapToken::Erc20(token_a_params) = token_a {
//             self.approve_liquidity(token_a_params, amount_a)
//                 .await;
//         };
//         if let SwapToken::Erc20(token_b_params) = token_b {
//             self.approve_liquidity(token_b_params, amount_b)
//                 .await;
//         };

//         match token_a {
//             SwapToken::Weth(_) => {
//                 match token_b {
//                     SwapToken::Weth(_) => {
//                         unimplemented!()
//                     },
//                     SwapToken::Erc20(token_b_params) => {
//                         self.add_liquidity_eth(token_b_params, amount_a, amount_b)
//                             .await;
//                     }
//                 }
//             },
//             SwapToken::Erc20(token_a_params) => {
//                 match token_b {
//                     SwapToken::Weth(_) => {
//                         self.add_liquidity_eth(token_a_params, amount_b, amount_a)
//                             .await;
//                     },
//                     SwapToken::Erc20(token_b_params) => {
//                         self.add_liquidity(token_a_params, token_b_params, amount_a, amount_b)
//                             .await;
//                     }
//                 }
//             },
//         }

//         Ok(())

//     }
}