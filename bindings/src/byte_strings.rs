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
            "0x6080806040523461001a576103199081610020823930815050f35b600080fdfe6040608081526004908136101561001557600080fd5b600091823560e01c63c93727931461002c57600080fd5b6020928360031936011261023c5781359067ffffffffffffffff821161023c573660238301121561023c57818301359161006d6100688461027b565b61023f565b928084526024913683838301011161023857818492848a9301838801378501015282516002600160ff1b03811015610234576001600160ff1b0381116001166102225760011b9260028401809411610222576100d76100ce6100688661027b565b9480865261027b565b94878501601f1980970136823785511561021057603090538451600110156101fe5760786021860153600292825192855b84811061015d578a8a89898c8351948592818452845191828186015281955b8387106101455750508394508582601f949501015201168101030190f35b86810182015189880189015295810195889550610127565b6101678183610297565b51958660fc1c601097888210156101ec57600f908b6101a7610188866102be565b956f181899199a1a9b1b9c1cb0b131b232b360811b9586901a92610297565b5360f81c16978810156101da57906101cf6101d593926101c6836102be565b991a918b610297565b536102be565b610108565b634e487b7160e01b8952603286528489fd5b634e487b7160e01b8a5260328752858afd5b634e487b7160e01b8452603290528183fd5b634e487b7160e01b8552603282528385fd5b50634e487b7160e01b82526011845290fd5b8280fd5b8380fd5b80fd5b6040519190601f01601f1916820167ffffffffffffffff81118382101761026557604052565b634e487b7160e01b600052604160045260246000fd5b67ffffffffffffffff811161026557601f01601f191660200190565b9081518110156102a8570160200190565b634e487b7160e01b600052603260045260246000fd5b60001981146102cd5760010190565b634e487b7160e01b600052601160045260246000fdfea2646970667358221220b424de7dd3917f30d844215f3e20e7670bc46e1c064182730c69aa91440c65c564736f6c63430008100033" . parse () . expect ("invalid bytecode")
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
