use std::str::FromStr;
use std::time::{ SystemTime, Duration };
use serde::{ Deserialize };

use secp256k1::SecretKey;
use web3::types::{ Address, U256 };
use web3::signing::{ Key, SecretKeyRef };
use web3::contract::{ Contract, Options };

use web3_tools::{ AsEip55, deploy_contract, get_contract_from_abi_file };
use ethers_tools::EthersUtils;

use neonevm_sdk::{
    // types::{ EthAddress, Erc20Specs },
    types::{ Erc20Specs },
};

mod liquidity;

use liquidity::{ NeonswapEnvironment, Erc20Means, Erc20Token, SwapToken, WethToken };

#[derive(Clone)]
#[derive(Deserialize)]
pub struct ContractPaths {
    uni: String,
    timelock: String,
    governor_alpha: String,
    weth9: String,
    uniswap_v1factory: String,
    uniswap_v1exchange: String,
    uniswap_v2factory: String,
    uniswap_v2router01: String,
    uniswap_v2router02: String,
    router_event_emitter: String,
    uniswap_v2migrator: String,
    multicall: String,
    erc20: String,
    erc20wrapper: String,
    example: String,
}

#[derive(Deserialize)]
struct DeployConfig {
    abi_paths: ContractPaths,
    key_path: String,
    url: String,
}

fn read_deploy_config(path: &str) -> Result<DeployConfig,()> {

    let f = std::fs::File::open(path);
    if f.is_err() {
        println!("Config file not found!");
    }
    let file = f.map_err(|_|())?;

    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).map_err(|_|())
}

const CONFIG_FILE_PATH: &'static str = "./debug_config.json";

#[tokio::main(flavor = "current_thread")]
async fn main() {

    let deploy_config: DeployConfig = read_deploy_config(CONFIG_FILE_PATH).unwrap();
    let paths = deploy_config.abi_paths;
    let eth_private_key: String = std::fs::read_to_string(deploy_config.key_path).unwrap();

    let ethers_utils = EthersUtils::new(&eth_private_key);
    
    let transport = web3::transports::Http::new(&deploy_config.url).unwrap();
    
    let web3 = web3::Web3::new(transport);

    println!("----- Deployment of Neonswap Contracts -----\n");

    let chain_id = web3.eth().chain_id().await.unwrap();
    println!("chain_id :  {}", chain_id);
    
    let key: SecretKey = SecretKey::from_str(&eth_private_key).unwrap();
    let key_ref: SecretKeyRef = SecretKeyRef::new(&key);
    let address: Address = key_ref.address();
    println!("Deployer Address: {}", address.as_eip55());

    let balance = web3.eth().balance(address, None).await.unwrap();
    println!("Balance of {}: {}", address.as_eip55(), balance);

    let nonce = web3.eth().transaction_count(address, None).await.unwrap();
    println!("Current Nonce of {}: {}", address.as_eip55(), nonce);
    let transaction_count = nonce.as_u32();

    println!("\n--------------------------------\n");

    let presumed_uni_address                    = ethers_utils.get_contract_address(0.into());
    let presumed_timelock_address               = ethers_utils.get_contract_address(1.into());
    let presumed_governor_alpha_address         = ethers_utils.get_contract_address(2.into());
    let presumed_weth9_address                  = ethers_utils.get_contract_address(3.into());
    let presumed_uniswap_v1factory_address      = ethers_utils.get_contract_address(4.into());
    let presumed_uniswap_v1exchange_address     = ethers_utils.get_contract_address(5.into());
    let presumed_uniswap_v2factory_address      = ethers_utils.get_contract_address(7.into());
    let presumed_uniswap_v2router01_address     = ethers_utils.get_contract_address(8.into());
    let presumed_uniswap_v2router02_address     = ethers_utils.get_contract_address(9.into());
    let presumed_router_event_emitter_address   = ethers_utils.get_contract_address(10.into());
    let presumed_uniswap_v2migrator_address     = ethers_utils.get_contract_address(11.into());
    let presumed_multicall_address              = ethers_utils.get_contract_address(12.into());

    if transaction_count == 0 {
        let minting_allowed_after: U256 = U256::from(
            SystemTime::now()
                .checked_add(Duration::from_secs(60*60))
                .unwrap()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                * 1000
            );
        let uni: Contract<web3::transports::Http> = 
            deploy_contract(&web3, &key, &paths.uni, (address,presumed_timelock_address,minting_allowed_after), None)
                .await
                .unwrap();
        println!("Deployed Uni Address: {}", uni.address().as_eip55());
        assert_eq!(presumed_uni_address, uni.address());
    } else {
        println!("Uni Exists at Address: {}", presumed_uni_address.as_eip55());
    } 
    
    if transaction_count < 1 {
        let delay: U256 = U256::from(60*60*24*3);
        let timelock: Contract<web3::transports::Http> = 
            deploy_contract(&web3, &key, &paths.timelock, (presumed_governor_alpha_address, delay), None)
                .await
                .unwrap();
        println!("Deployed Timelock Address: {}", timelock.address().as_eip55());
        assert_eq!(presumed_timelock_address, timelock.address());
    } else {
        println!("Timelock Exists at Address: {}", presumed_timelock_address.as_eip55());
    }
    
    if transaction_count < 2 {
        let governor_alpha: Contract<web3::transports::Http> = 
            deploy_contract(&web3, &key, &paths.governor_alpha, (presumed_timelock_address, presumed_uni_address), None)
                .await
                .unwrap();
        println!("Deployed Governor Alpha Address: {}", governor_alpha.address().as_eip55());
        assert_eq!(presumed_governor_alpha_address, governor_alpha.address());
    } else {
        println!("Governor Alpha Exists at Address: {}", presumed_governor_alpha_address.as_eip55());
    }

    if transaction_count < 3 {
        let weth9: Contract<web3::transports::Http> = 
            deploy_contract(&web3, &key, &paths.weth9, (), None)
                .await
                .unwrap();
        println!("Deployed WETH Address: {}", weth9.address().as_eip55());
        assert_eq!(presumed_weth9_address, weth9.address());
    } else {
        println!("WETH Exists at Address: {}", presumed_weth9_address.as_eip55());
    }

    let uniswap_v1factory: Contract<web3::transports::Http> = 
        if transaction_count < 4 {
            let uniswap_v1factory: Contract<web3::transports::Http> = 
                deploy_contract(&web3, &key, &paths.uniswap_v1factory, (), None)
                    .await
                    .unwrap();
            println!("Deployed Uniswap V1 Factory Address: {}", uniswap_v1factory.address().as_eip55());
            assert_eq!(presumed_uniswap_v1factory_address, uniswap_v1factory.address());
            uniswap_v1factory
        } else {
            println!("Uniswap V1 Factory Exists at Address: {}", presumed_uniswap_v1factory_address.as_eip55());
            get_contract_from_abi_file(&web3, &paths.uniswap_v1factory, presumed_uniswap_v1factory_address).unwrap()
        };

    if transaction_count < 5 {
        let uniswap_v1exchange: Contract<web3::transports::Http> = 
            deploy_contract(&web3, &key, &paths.uniswap_v1exchange, (), None)
                .await
                .unwrap();
        println!("Deployed Uniswap V1 Exchange Address: {}", uniswap_v1exchange.address().as_eip55());
        assert_eq!(presumed_uniswap_v1exchange_address, uniswap_v1exchange.address());
    } else {
        println!("Uniswap V1 Exchange Exists at Address: {}", presumed_uniswap_v1exchange_address.as_eip55());
    }

    if transaction_count < 6 {
        let _uniswap_v1factory_initialize = 
            uniswap_v1factory.signed_call_with_confirmations("initializeFactory", presumed_uniswap_v1exchange_address, Options::default(), 0, &key)
                .await
                .unwrap();
    }

    if transaction_count < 7 {
        let uniswap_v2factory: Contract<web3::transports::Http> = 
            deploy_contract(&web3, &key, &paths.uniswap_v2factory, address, None)
                .await
                .unwrap();
        println!("Deployed Uniswap V2 Factory Address: {}", uniswap_v2factory.address().as_eip55());
        assert_eq!(presumed_uniswap_v2factory_address, uniswap_v2factory.address());
    } else {
        println!("Uniswap V2 Factory Exists at Address: {}", presumed_uniswap_v2factory_address.as_eip55());
    }

    if transaction_count < 8 {
        let uniswap_v2router01: Contract<web3::transports::Http> = 
            deploy_contract(&web3, &key, &paths.uniswap_v2router01, (presumed_uniswap_v2factory_address, presumed_weth9_address), None)
                .await
                .unwrap();
        println!("Deployed Uniswap V2 Router01 Address: {}", uniswap_v2router01.address().as_eip55());
        assert_eq!(presumed_uniswap_v2router01_address, uniswap_v2router01.address());
    } else {
        println!("Uniswap V2 Router01 Exists at Address: {}", presumed_uniswap_v2router01_address.as_eip55());
    }

    if transaction_count < 9 {
        let uniswap_v2router02: Contract<web3::transports::Http> = 
            deploy_contract(&web3, &key, &paths.uniswap_v2router02, (presumed_uniswap_v2factory_address, presumed_weth9_address), None)
                .await
                .unwrap();
        println!("Deployed Uniswap V2 Router02 Address: {}", uniswap_v2router02.address().as_eip55());
        assert_eq!(presumed_uniswap_v2router02_address, uniswap_v2router02.address());
    } else {
        println!("Uniswap V2 Router02 Exists at Address: {}", presumed_uniswap_v2router02_address.as_eip55());
    }

    if transaction_count < 10 {
        let router_event_emitter: Contract<web3::transports::Http> = 
            deploy_contract(&web3, &key, &paths.router_event_emitter, (), None)
                .await
                .unwrap();
        println!("Deployed Router Event Emitter Address: {}", router_event_emitter.address().as_eip55());
        assert_eq!(presumed_router_event_emitter_address, router_event_emitter.address());
    } else {
        println!("Router Event Emitter Exists at Address: {}", presumed_router_event_emitter_address.as_eip55());
    }

    if transaction_count < 11 {
        let uniswap_v2migrator: Contract<web3::transports::Http> = 
            deploy_contract(&web3, &key, &paths.uniswap_v2migrator, (presumed_uniswap_v1factory_address, presumed_uniswap_v2router01_address), None)
                .await
                .unwrap();
        println!("Deployed Uniswap V2 Migrator Address: {}", uniswap_v2migrator.address().as_eip55());
        assert_eq!(presumed_uniswap_v2migrator_address, uniswap_v2migrator.address());
    } else {
        println!("Uniswap V2 Migrator Exists at Address: {}", presumed_uniswap_v2migrator_address.as_eip55());
    }

    if transaction_count < 12 {
        let multicall: Contract<web3::transports::Http> = 
            deploy_contract(&web3, &key, &paths.multicall, (), None)
                .await
                .unwrap();
        println!("Deployed Multicall Address: {}", multicall.address().as_eip55());
        assert_eq!(presumed_multicall_address, multicall.address());
    } else {
        println!("Multicall Exists at Address: {}", presumed_multicall_address.as_eip55());
    }

    println!("\n--------------------------------\n");

    let presumed_weth_partner_address = ethers_utils.get_contract_address(13.into());
    let presumed_token_a_address = ethers_utils.get_contract_address(14.into());
    let presumed_token_b_address = ethers_utils.get_contract_address(15.into());
    let presumed_example_address = ethers_utils.get_contract_address(16.into());

    let token_weth_partner: Contract<web3::transports::Http> = 
        if transaction_count < 13 {
            let supply_weth_partner: U256 = U256::from(500000*1000000000000000000u128);
            let token_weth_partner: Contract<web3::transports::Http> = 
                deploy_contract(&web3, &key, &paths.erc20, supply_weth_partner, None)
                    .await
                    .unwrap();
            println!("Deployed WETH Partner Address: {}", token_weth_partner.address().as_eip55());
            assert_eq!(presumed_weth_partner_address, token_weth_partner.address());
            token_weth_partner
        } else {
            let token_weth_partner: Contract<web3::transports::Http> =
                get_contract_from_abi_file(&web3, &paths.erc20, presumed_weth_partner_address).unwrap();
            println!("WETH Partner at Address: {}", presumed_weth_partner_address.as_eip55());
            token_weth_partner
        };

    if transaction_count < 14 {
        let supply_a: U256 = U256::from(500000*1000000000000000000u128);
        let token_a: Contract<web3::transports::Http> = 
            deploy_contract(&web3, &key, &paths.erc20, supply_a, None)
                .await
                .unwrap();
        println!("Deployed Token 'A' Address: {}", token_a.address().as_eip55());
        assert_eq!(presumed_token_a_address, token_a.address());
    } else {
        println!("Token 'A' at Address: {}", presumed_token_a_address.as_eip55());
    }

    if transaction_count < 15 {
        let supply_b: U256 = U256::from(300000*1000000000000000000u128);
        let token_b: Contract<web3::transports::Http> = 
            deploy_contract(&web3, &key, &paths.erc20, supply_b, None)
                .await
                .unwrap();
        println!("Deployed Token 'B' Address: {}", token_b.address().as_eip55());
        assert_eq!(presumed_token_b_address, token_b.address());
    } else {
        println!("Token 'B' at Address: {}", presumed_token_b_address.as_eip55());
    }

    if transaction_count < 16 {
        let example: Contract<web3::transports::Http> = 
            deploy_contract(&web3, &key, &paths.example, (presumed_uniswap_v2factory_address, presumed_uniswap_v1factory_address, presumed_uniswap_v2router02_address), None)
                .await
                .unwrap();
        println!("Deployed Example Flash Swap Address: {}", example.address().as_eip55());
        assert_eq!(presumed_example_address, example.address());
    } else {
        println!("Example Flash Swap Exists at Address: {}", presumed_example_address.as_eip55());
    }

    let neon_token: SwapToken = SwapToken::Weth(WethToken::new(&presumed_weth9_address.as_eip55()));
    println!("{:?}", neon_token);

    let swap_token_weth_partner: SwapToken =
        SwapToken::Erc20(
            Erc20Token {
                specs: Erc20Specs {
                        name: "WETH Partner".to_string(),
                        symbol: "WETHP".to_string(),
                        decimals: 18,
                    },
                // eth_address: EthAddress::from_str("0xC59dEC342962109CB5F3bCF14e088347DFDC5e72").unwrap(),
                eth_address: presumed_weth_partner_address.into(),
                means: Erc20Means::Origin,
            }
        );
    // println!("WETH Partner: {:?}", token_weth_partner);

    let neonswap: NeonswapEnvironment =
        NeonswapEnvironment::new(
            web3,
            &eth_private_key,
            ethers_utils,
            // paths.clone(),
            // presumed_uniswap_v1factory_address,
            // presumed_uniswap_v2factory_address,
            // presumed_uniswap_v2router02_address,
        );

    if transaction_count < 17 {
        let uniswap_v1factory_create_exchange = 
            uniswap_v1factory.signed_call_with_confirmations("createExchange", presumed_weth_partner_address, neonswap.default_web3_options(), 0, &key)
                .await
                .unwrap();
        println!("createExchange: {:?}", uniswap_v1factory_create_exchange);
    } else {
        println!("createExchange Exists");
    }
    
    let uniswap_v2_get_exchange_address: Address = 
        uniswap_v1factory.query("getExchange", presumed_weth_partner_address, neonswap.signing_address, neonswap.default_web3_options(), None)
            .await
            .unwrap();
    println!("{:?}", uniswap_v2_get_exchange_address);

    // let eth_weth_exchange_address: EthAddress = EthAddress::from_str(&uniswap_v2_get_exchange_address.as_eip55()).unwrap();
    let weth_exchange = get_contract_from_abi_file(&neonswap.web3, &paths.uniswap_v1exchange, uniswap_v2_get_exchange_address).unwrap();
    println!("{:?}", weth_exchange.address());

    let approve = 
        token_weth_partner.signed_call_with_confirmations("approve",(weth_exchange.address(), swap_token_weth_partner.expand_from_uint(5)), neonswap.default_web3_options(), 0, &neonswap.signing_key)
            .await;
    
    match approve {
        Ok(receipt) => {
            let status: u64 = receipt.status.map(|s| s.as_u64()).unwrap_or(0u64);
            if status > 0 {
                println!("Approved!");
            } else {
                println!("Approve Declined with Receipt:\n{:?}", receipt);
            }
        },
        Err(error) => {
            println!("Approve failed with Error:\n{:?}", error);
        },
    }
    // println!("{:?}", approve);

    let min_liquidity: U256 = U256::from(5*1000000000000000000u128);
    let max_tokens: U256 = U256::from(5*1000000000000000000u128);
    let deadline = SystemTime::now()
        .checked_add(Duration::from_secs(3600))
        .unwrap()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        * 1000;
    let deadline_u256: U256 = U256::from(deadline);
    let add_liquidity_v1 = 
        weth_exchange.signed_call_with_confirmations("addLiquidity", (min_liquidity, max_tokens, deadline_u256), neonswap.default_web3_options(), 0, &neonswap.signing_key)
            .await;

    match add_liquidity_v1 {
        Ok(receipt) => {
            let status: u64 = receipt.status.map(|s| s.as_u64()).unwrap_or(0u64);
            if status > 0 {
                println!("Added!");
            } else {
                println!("Add Liquidity Declined with Receipt:\n{:?}", receipt);
            }
        },
        Err(error) => {
            println!("Add Liquidity failed with Error:\n{:?}", error);
        },
    }
    // println!("{:?}", add_liquidity_v1);

    // neonswap.create_pair_add_liquidity(&neon_token, &token_weth_partner, neon_token.expand_from_uint(1000), token_weth_partner.expand_from_uint(4000))
    //     .await
    //     .unwrap();

}