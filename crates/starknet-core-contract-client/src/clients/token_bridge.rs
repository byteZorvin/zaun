use std::sync::Arc;

use crate::{interfaces::{
    DaiERC20Token, StarkgateManager, StarkgateRegistry, StarknetTokenBridge
}, LocalWalletSignerMiddleware, StarknetBridgeContractClient};

use ethers::types::Address;

/// Client to interact with a Token Bridge (ERC20)
pub struct StarknetTokenBridgeContractClient {
    manager: StarkgateManager<LocalWalletSignerMiddleware>,
    registry: StarkgateRegistry<LocalWalletSignerMiddleware>,
    token_bridge: StarknetTokenBridge<LocalWalletSignerMiddleware>,
    erc20_token: DaiERC20Token<LocalWalletSignerMiddleware>,
}

impl StarknetTokenBridgeContractClient {
    pub fn new(manager: Address, registry: Address, token_bridge: Address, dai: Address, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self {
            manager: StarkgateManager::new(manager, client.clone()),
            registry: StarkgateRegistry::new(registry, client.clone()),
            token_bridge: StarknetTokenBridge::new(token_bridge, client.clone()),
            erc20_token: DaiERC20Token::new(dai, client.clone()),
        }
    }
}

impl AsRef<StarkgateManager<LocalWalletSignerMiddleware>> for StarknetTokenBridgeContractClient {
    fn as_ref(&self) -> &StarkgateManager<LocalWalletSignerMiddleware> {
        &self.manager
    }
}

impl AsRef<StarkgateRegistry<LocalWalletSignerMiddleware>> for StarknetTokenBridgeContractClient {
    fn as_ref(&self) -> &StarkgateRegistry<LocalWalletSignerMiddleware> {
        &self.registry
    }
}

impl AsRef<StarknetTokenBridge<LocalWalletSignerMiddleware>> for StarknetTokenBridgeContractClient {
    fn as_ref(&self) -> &StarknetTokenBridge<LocalWalletSignerMiddleware> {
        &self.token_bridge
    }
}

impl AsRef<DaiERC20Token<LocalWalletSignerMiddleware>> for StarknetTokenBridgeContractClient {
    fn as_ref(&self) -> &DaiERC20Token<LocalWalletSignerMiddleware> {
        &self.erc20_token
    }
}

impl StarknetBridgeContractClient for StarknetTokenBridgeContractClient {
    fn address(&self) -> Address {
        self.token_bridge.address()
    }

    fn manager(&self) -> Address {
        self.manager.address()
    }

    fn registry(&self) -> Address {
        self.registry.address()
    }

    fn token(&self) -> Address {
        self.erc20_token.address()
    }

    fn manager_client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.manager.client()
    }
    fn registry_client(&self) -> Arc<LocalWalletSignerMiddleware> { self.registry.client() }
    fn bridge_client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.token_bridge.client()
    }

    fn token_client(&self) -> Arc<LocalWalletSignerMiddleware> { self.erc20_token.client() }
}