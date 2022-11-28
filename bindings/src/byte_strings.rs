pub use byte_strings::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod byte_strings {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "ByteStrings was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"input\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"toHexString\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static BYTESTRINGS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static BYTESTRINGS_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080806040523461001a576103aa9081610020823930815050f35b600080fdfe608060405260048036101561001357600080fd5b6000803560e01c63c93727931461002a5750600080fd5b60203660031901126101ac57813567ffffffffffffffff81116101a857366023820112156101a85780830135610067610062826101f9565b6101c6565b9181835236602483830101116101a457818492602460209301838601378301015280516100b7906100b2906100ad906100a86002600160ff1b038210610292565b61029e565b6102c3565b6102df565b9260306100c38561031e565b5360786100cf85610334565b536002918051935b8481106100f057604051806100ec8882610224565b0390f35b8061012161011b6101156101076101859587610345565b516001600160f81b03191690565b60f81c90565b60ff1690565b9461017f86861c600f60109889831015610197575b8b61016261014387610364565b966f181899199a1a9b1b9c1cb0b131b232b360811b9586901a92610345565b53169788101561018a575b61017683610364565b971a9189610345565b53610364565b6100d7565b610192610307565b61016d565b61019f610307565b610136565b8380fd5b5080fd5b80fd5b50634e487b7160e01b600052604160045260246000fd5b6040519190601f01601f1916820167ffffffffffffffff8111838210176101ec57604052565b6101f46101af565b604052565b60209067ffffffffffffffff8111610217575b601f01601f19160190565b61021f6101af565b61020c565b919091602080825283519081818401526000945b828610610265575050806040939411610258575b601f01601f1916010190565b600083828401015261024c565b8581018201518487016040015294810194610238565b50634e487b7160e01b600052601160045260246000fd5b1561029957565b600080fd5b6001600160ff1b0381116001166102b6575b60011b90565b6102be61027b565b6102b0565b60029060021981116102d3570190565b6102db61027b565b0190565b906102ec610062836101f9565b82815280926102fd601f19916101f9565b0190602036910137565b50634e487b7160e01b600052603260045260246000fd5b60209080511561032c570190565b6102db610307565b60219080516001101561032c570190565b90602091805182101561035757010190565b61035f610307565b010190565b60019060001981146102d357019056fea2646970667358221220297ed87e5ca15611c53efce6c6f801796ee30d93994d0d50f40ede05743a03b164736f6c634300080d0033" . parse () . expect ("invalid bytecode")
        });
    pub struct ByteStrings<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ByteStrings<M> {
        fn clone(&self) -> Self {
            ByteStrings(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ByteStrings<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for ByteStrings<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ByteStrings))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ByteStrings<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), BYTESTRINGS_ABI.clone(), client).into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                BYTESTRINGS_ABI.clone(),
                BYTESTRINGS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `toHexString` (0xc9372793) function"]
        pub fn to_hex_string(
            &self,
            input: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([201, 55, 39, 147], input)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ByteStrings<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `toHexString` function with signature `toHexString(bytes)` and selector `[201, 55, 39, 147]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "toHexString", abi = "toHexString(bytes)")]
    pub struct ToHexStringCall {
        pub input: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all return fields from the `toHexString` function with signature `toHexString(bytes)` and selector `[201, 55, 39, 147]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ToHexStringReturn(pub String);
}
