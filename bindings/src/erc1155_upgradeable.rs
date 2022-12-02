pub use erc1155_upgradeable::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod erc1155_upgradeable {
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
    #[doc = "ERC1155Upgradeable was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ApprovalForAll\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TransferBatch\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TransferSingle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"value\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"URI\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"accounts\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOfBatch\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isApprovedForAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeBatchTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setApprovalForAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"uri\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static ERC1155UPGRADEABLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ERC1155UPGRADEABLE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080806040523461001657610fac908161001c8239f35b600080fdfe6040608081526004908136101561001557600080fd5b600091823560e01c8062fdd58e14610abd57806301ffc9a714610a505780630e89341c146109565780632eb2c2d61461064a5780634e1273f4146104aa578063a22cb465146103cd578063e985e9c51461037b5763f242432a1461007857600080fd5b346103775760a036600319011261037757610091610aed565b8361009a610b08565b91604435906064356084356001600160401b038111610373576100c09036908801610c0a565b926001600160a01b03928316923384148015610354575b6100e090610d68565b8616906100ee821515610dcc565b6100f781610f41565b5061010183610f41565b50808652602096606588528987208588528852838a88205461012582821015610e26565b83895260658a528b8920878a528a52038a882055818752606588528987208388528852898720610156858254610e85565b905582858b51848152868b8201527fc3d58168c5ae7397731d063d5bbf3d657854427343f4c083240f7aacaa2d0f628d3392a43b61019357858951f35b879587946101d48b519788968795869463f23a6e6160e01b9c8d8752339087015260248601526044850152606484015260a0608484015260a4830190610b1e565b03925af1869181610325575b506102b15750506001906101f2610eb2565b6308c379a01461027e575b5061021157505b3880838180808080858951f35b905162461bcd60e51b815290819061027a90820160809060208152603460208201527f455243313135353a207472616e7366657220746f206e6f6e20455243313135356040820152732932b1b2b4bb32b91034b6b83632b6b2b73a32b960611b60608201520190565b0390fd5b610286610ed0565b8061029157506101fd565b61027a8491865193849362461bcd60e51b85528401526024830190610b1e565b6001600160e01b0319160390506102c85750610204565b905162461bcd60e51b815290819061027a90820160809060208152602860208201527f455243313135353a204552433131353552656365697665722072656a656374656040820152676420746f6b656e7360c01b60608201520190565b610346919250843d861161034d575b61033e8183610b5e565b810190610e92565b90386101e0565b503d610334565b508386526066602090815289872033885290528886205460ff166100d7565b8480fd5b8280fd5b5050346103c957806003193601126103c95760ff8160209361039b610aed565b6103a3610b08565b6001600160a01b0391821683526066875283832091168252855220549151911615158152f35b5080fd5b50346103775781600319360112610377576103e6610aed565b60243590811515809203610373576001600160a01b0316913383146104555750338452606660205282842082855260205282842060ff1981541660ff831617905582519081527f17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c3160203392a351f35b608490602085519162461bcd60e51b8352820152602960248201527f455243313135353a2073657474696e6720617070726f76616c20737461747573604482015268103337b91039b2b63360b91b6064820152fd5b503461037757816003193601126103775780356001600160401b0380821161037357366023830112156103735781830135906104e582610b95565b926104f286519485610b5e565b82845260209260248486019160051b8301019136831161064657602401905b8282106106235750505060243590811161061f576105329036908501610bac565b9282518451036105cc575081519461054986610b95565b9561055686519788610b5e565b808752610565601f1991610b95565b0136838801375b82518110156105b6576105b1906105a16001600160a01b0361058e8387610d3e565b511661059a8388610d3e565b5190610c94565b6105ab8289610d3e565b52610d19565b61056c565b8451828152806105c881850189610c60565b0390f35b60849185519162461bcd60e51b8352820152602960248201527f455243313135353a206163636f756e747320616e6420696473206c656e677468604482015268040dad2e6dac2e8c6d60bb1b6064820152fd5b8580fd5b81356001600160a01b0381168103610642578152908401908401610511565b8980fd5b8880fd5b508234610953576003199260a0368501126103c957610667610aed565b91610670610b08565b946001600160401b039360443585811161094f576106919036908801610bac565b606435868111610373576106a89036908901610bac565b95608435908111610373576106c09036908901610c0a565b966001600160a01b03928316923384148015610930575b6106e090610d68565b82518851036108dc578916936106f7851515610dcc565b855b835181101561077c57806107106107779286610d3e565b5161071b828c610d3e565b5190808a5260656020918183528c8c208a8d528352838d8d8c828220549261074585851015610e26565b858352868852822091528552038d8d20558b5281528a8a2090898b52526107708a8a20918254610e85565b9055610d19565b6106f9565b50949897909692959784878a518b81527f4a39dc06d4c0dbc64b70af90fd698a233a518aa5d07e595d983b8c0526c8f7fb6107b98d830188610c60565b918083036020820152806107ce33948b610c60565b0390a43b6107db57888851f35b8751948593849363bc197c8160e01b98898652338b87015260248601526044850160a0905260a4850161080d91610c60565b8285820301606486015261082091610c60565b9083820301608484015261083391610b1e565b0381885a94602095f18591816108bc575b506108a75750506001610855610eb2565b6308c379a014610872575b61021157505b82808080808080888851f35b61087a610ed0565b806108855750610860565b61027a91506020935193849362461bcd60e51b85528401526024830190610b1e565b6001600160e01b031916036102c85750610866565b6108d591925060203d811161034d5761033e8183610b5e565b9086610844565b865162461bcd60e51b8152602081840152602860248201527f455243313135353a2069647320616e6420616d6f756e7473206c656e677468206044820152670dad2e6dac2e8c6d60c31b6064820152608490fd5b508386526066602090815287872033885290528686205460ff166106d7565b8380fd5b80fd5b509190346103c9576020908160031936011261037757805192806067549060019082821c928281168015610a46575b8785108114610a33578899509688969785829a529182600014610a0c5750506001146109ce575b5050506105c892916109bf910385610b5e565b51928284938452830190610b1e565b9190869350606783528383205b8284106109f457505050820101816109bf6105c86109ac565b8054848a0186015288955087949093019281016109db565b60ff19168782015293151560051b860190930193508492506109bf91506105c890506109ac565b634e487b7160e01b835260228a52602483fd5b93607f1693610985565b503461037757602036600319011261037757359063ffffffff60e01b82168092036103775760209250636cdb3d1360e11b8214918215610aac575b8215610a9b575b50519015158152f35b6301ffc9a760e01b14915038610a92565b6303a24d0760e21b81149250610a8b565b5050346103c957806003193601126103c957602090610ae6610add610aed565b60243590610c94565b9051908152f35b600435906001600160a01b0382168203610b0357565b600080fd5b602435906001600160a01b0382168203610b0357565b919082519283825260005b848110610b4a575050826000602080949584010152601f8019910116010190565b602081830181015184830182015201610b29565b90601f801991011681019081106001600160401b03821117610b7f57604052565b634e487b7160e01b600052604160045260246000fd5b6001600160401b038111610b7f5760051b60200190565b81601f82011215610b0357803591610bc383610b95565b92610bd16040519485610b5e565b808452602092838086019260051b820101928311610b03578301905b828210610bfb575050505090565b81358152908301908301610bed565b81601f82011215610b03578035906001600160401b038211610b7f5760405192610c3e601f8401601f191660200185610b5e565b82845260208383010111610b0357816000926020809301838601378301015290565b90815180825260208080930193019160005b828110610c80575050505090565b835185529381019392810192600101610c72565b6001600160a01b0316908115610cc157600052606560205260406000209060005260205260406000205490565b60405162461bcd60e51b815260206004820152602a60248201527f455243313135353a2061646472657373207a65726f206973206e6f742061207660448201526930b634b21037bbb732b960b11b6064820152608490fd5b6000198114610d285760010190565b634e487b7160e01b600052601160045260246000fd5b8051821015610d525760209160051b010190565b634e487b7160e01b600052603260045260246000fd5b15610d6f57565b60405162461bcd60e51b815260206004820152602f60248201527f455243313135353a2063616c6c6572206973206e6f7420746f6b656e206f776e60448201526e195c881b9bdc88185c1c1c9bdd9959608a1b6064820152608490fd5b15610dd357565b60405162461bcd60e51b815260206004820152602560248201527f455243313135353a207472616e7366657220746f20746865207a65726f206164604482015264647265737360d81b6064820152608490fd5b15610e2d57565b60405162461bcd60e51b815260206004820152602a60248201527f455243313135353a20696e73756666696369656e742062616c616e636520666f60448201526939103a3930b739b332b960b11b6064820152608490fd5b91908201809211610d2857565b90816020910312610b0357516001600160e01b031981168103610b035790565b60009060033d11610ebf57565b905060046000803e60005160e01c90565b600060443d10610f2d57604051600319913d83016004833e81516001600160401b03918282113d602484011117610f3057818401948551938411610f38573d85010160208487010111610f305750610f2d92910160200190610b5e565b90565b949350505050565b50949350505050565b60405190604082018281106001600160401b03821117610b7f5760405260018252602082016020368237825115610d5257529056fea2646970667358221220d5dff43a4f155cb57206df6b5de161f1b3e04f2dd9d5c365df4540bd5ddea2b064736f6c63430008100033" . parse () . expect ("invalid bytecode")
        });
    pub struct ERC1155Upgradeable<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ERC1155Upgradeable<M> {
        fn clone(&self) -> Self {
            ERC1155Upgradeable(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ERC1155Upgradeable<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for ERC1155Upgradeable<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ERC1155Upgradeable))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ERC1155Upgradeable<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ERC1155UPGRADEABLE_ABI.clone(), client)
                .into()
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
                ERC1155UPGRADEABLE_ABI.clone(),
                ERC1155UPGRADEABLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `balanceOf` (0x00fdd58e) function"]
        pub fn balance_of(
            &self,
            account: ethers::core::types::Address,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([0, 253, 213, 142], (account, id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOfBatch` (0x4e1273f4) function"]
        pub fn balance_of_batch(
            &self,
            accounts: ::std::vec::Vec<ethers::core::types::Address>,
            ids: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([78, 18, 115, 244], (accounts, ids))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isApprovedForAll` (0xe985e9c5) function"]
        pub fn is_approved_for_all(
            &self,
            account: ethers::core::types::Address,
            operator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (account, operator))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeBatchTransferFrom` (0x2eb2c2d6) function"]
        pub fn safe_batch_transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            ids: ::std::vec::Vec<ethers::core::types::U256>,
            amounts: ::std::vec::Vec<ethers::core::types::U256>,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 178, 194, 214], (from, to, ids, amounts, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeTransferFrom` (0xf242432a) function"]
        pub fn safe_transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            id: ethers::core::types::U256,
            amount: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 66, 67, 42], (from, to, id, amount, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setApprovalForAll` (0xa22cb465) function"]
        pub fn set_approval_for_all(
            &self,
            operator: ethers::core::types::Address,
            approved: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supportsInterface` (0x01ffc9a7) function"]
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `uri` (0x0e89341c) function"]
        pub fn uri(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([14, 137, 52, 28], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ApprovalForAll` event"]
        pub fn approval_for_all_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ApprovalForAllFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TransferBatch` event"]
        pub fn transfer_batch_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TransferBatchFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TransferSingle` event"]
        pub fn transfer_single_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TransferSingleFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `URI` event"]
        pub fn uri_filter(&self) -> ethers::contract::builders::Event<M, UriFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ERC1155UpgradeableEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ERC1155Upgradeable<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ethers::core::types::Address,
        pub approved: bool,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TransferBatch",
        abi = "TransferBatch(address,address,address,uint256[],uint256[])"
    )]
    pub struct TransferBatchFilter {
        #[ethevent(indexed)]
        pub operator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub ids: Vec<ethers::core::types::U256>,
        pub values: Vec<ethers::core::types::U256>,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TransferSingle",
        abi = "TransferSingle(address,address,address,uint256,uint256)"
    )]
    pub struct TransferSingleFilter {
        #[ethevent(indexed)]
        pub operator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
        pub value: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "URI", abi = "URI(string,uint256)")]
    pub struct UriFilter {
        pub value: String,
        #[ethevent(indexed)]
        pub id: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ERC1155UpgradeableEvents {
        ApprovalForAllFilter(ApprovalForAllFilter),
        InitializedFilter(InitializedFilter),
        TransferBatchFilter(TransferBatchFilter),
        TransferSingleFilter(TransferSingleFilter),
        UriFilter(UriFilter),
    }
    impl ethers::contract::EthLogDecode for ERC1155UpgradeableEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(ERC1155UpgradeableEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(ERC1155UpgradeableEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = TransferBatchFilter::decode_log(log) {
                return Ok(ERC1155UpgradeableEvents::TransferBatchFilter(decoded));
            }
            if let Ok(decoded) = TransferSingleFilter::decode_log(log) {
                return Ok(ERC1155UpgradeableEvents::TransferSingleFilter(decoded));
            }
            if let Ok(decoded) = UriFilter::decode_log(log) {
                return Ok(ERC1155UpgradeableEvents::UriFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ERC1155UpgradeableEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ERC1155UpgradeableEvents::ApprovalForAllFilter(element) => element.fmt(f),
                ERC1155UpgradeableEvents::InitializedFilter(element) => element.fmt(f),
                ERC1155UpgradeableEvents::TransferBatchFilter(element) => element.fmt(f),
                ERC1155UpgradeableEvents::TransferSingleFilter(element) => element.fmt(f),
                ERC1155UpgradeableEvents::UriFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address,uint256)` and selector `[0, 253, 213, 142]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address,uint256)")]
    pub struct BalanceOfCall {
        pub account: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `balanceOfBatch` function with signature `balanceOfBatch(address[],uint256[])` and selector `[78, 18, 115, 244]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "balanceOfBatch", abi = "balanceOfBatch(address[],uint256[])")]
    pub struct BalanceOfBatchCall {
        pub accounts: ::std::vec::Vec<ethers::core::types::Address>,
        pub ids: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `[233, 133, 233, 197]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub account: ethers::core::types::Address,
        pub operator: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `safeBatchTransferFrom` function with signature `safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)` and selector `[46, 178, 194, 214]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "safeBatchTransferFrom",
        abi = "safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct SafeBatchTransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub ids: ::std::vec::Vec<ethers::core::types::U256>,
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,uint256,bytes)` and selector `[242, 66, 67, 42]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,uint256,bytes)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `[162, 44, 180, 101]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ethers::core::types::Address,
        pub approved: bool,
    }
    #[doc = "Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    #[doc = "Container type for all input parameters for the `uri` function with signature `uri(uint256)` and selector `[14, 137, 52, 28]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "uri", abi = "uri(uint256)")]
    pub struct UriCall(pub ethers::core::types::U256);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ERC1155UpgradeableCalls {
        BalanceOf(BalanceOfCall),
        BalanceOfBatch(BalanceOfBatchCall),
        IsApprovedForAll(IsApprovedForAllCall),
        SafeBatchTransferFrom(SafeBatchTransferFromCall),
        SafeTransferFrom(SafeTransferFromCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SupportsInterface(SupportsInterfaceCall),
        Uri(UriCall),
    }
    impl ethers::core::abi::AbiDecode for ERC1155UpgradeableCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC1155UpgradeableCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC1155UpgradeableCalls::BalanceOfBatch(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC1155UpgradeableCalls::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) =
                <SafeBatchTransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC1155UpgradeableCalls::SafeBatchTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC1155UpgradeableCalls::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SetApprovalForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC1155UpgradeableCalls::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC1155UpgradeableCalls::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <UriCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ERC1155UpgradeableCalls::Uri(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ERC1155UpgradeableCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ERC1155UpgradeableCalls::BalanceOf(element) => element.encode(),
                ERC1155UpgradeableCalls::BalanceOfBatch(element) => element.encode(),
                ERC1155UpgradeableCalls::IsApprovedForAll(element) => element.encode(),
                ERC1155UpgradeableCalls::SafeBatchTransferFrom(element) => element.encode(),
                ERC1155UpgradeableCalls::SafeTransferFrom(element) => element.encode(),
                ERC1155UpgradeableCalls::SetApprovalForAll(element) => element.encode(),
                ERC1155UpgradeableCalls::SupportsInterface(element) => element.encode(),
                ERC1155UpgradeableCalls::Uri(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ERC1155UpgradeableCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ERC1155UpgradeableCalls::BalanceOf(element) => element.fmt(f),
                ERC1155UpgradeableCalls::BalanceOfBatch(element) => element.fmt(f),
                ERC1155UpgradeableCalls::IsApprovedForAll(element) => element.fmt(f),
                ERC1155UpgradeableCalls::SafeBatchTransferFrom(element) => element.fmt(f),
                ERC1155UpgradeableCalls::SafeTransferFrom(element) => element.fmt(f),
                ERC1155UpgradeableCalls::SetApprovalForAll(element) => element.fmt(f),
                ERC1155UpgradeableCalls::SupportsInterface(element) => element.fmt(f),
                ERC1155UpgradeableCalls::Uri(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BalanceOfCall> for ERC1155UpgradeableCalls {
        fn from(var: BalanceOfCall) -> Self {
            ERC1155UpgradeableCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BalanceOfBatchCall> for ERC1155UpgradeableCalls {
        fn from(var: BalanceOfBatchCall) -> Self {
            ERC1155UpgradeableCalls::BalanceOfBatch(var)
        }
    }
    impl ::std::convert::From<IsApprovedForAllCall> for ERC1155UpgradeableCalls {
        fn from(var: IsApprovedForAllCall) -> Self {
            ERC1155UpgradeableCalls::IsApprovedForAll(var)
        }
    }
    impl ::std::convert::From<SafeBatchTransferFromCall> for ERC1155UpgradeableCalls {
        fn from(var: SafeBatchTransferFromCall) -> Self {
            ERC1155UpgradeableCalls::SafeBatchTransferFrom(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromCall> for ERC1155UpgradeableCalls {
        fn from(var: SafeTransferFromCall) -> Self {
            ERC1155UpgradeableCalls::SafeTransferFrom(var)
        }
    }
    impl ::std::convert::From<SetApprovalForAllCall> for ERC1155UpgradeableCalls {
        fn from(var: SetApprovalForAllCall) -> Self {
            ERC1155UpgradeableCalls::SetApprovalForAll(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for ERC1155UpgradeableCalls {
        fn from(var: SupportsInterfaceCall) -> Self {
            ERC1155UpgradeableCalls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<UriCall> for ERC1155UpgradeableCalls {
        fn from(var: UriCall) -> Self {
            ERC1155UpgradeableCalls::Uri(var)
        }
    }
    #[doc = "Container type for all return fields from the `balanceOf` function with signature `balanceOf(address,uint256)` and selector `[0, 253, 213, 142]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BalanceOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `balanceOfBatch` function with signature `balanceOfBatch(address[],uint256[])` and selector `[78, 18, 115, 244]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BalanceOfBatchReturn(pub ::std::vec::Vec<ethers::core::types::U256>);
    #[doc = "Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `[233, 133, 233, 197]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsApprovedForAllReturn(pub bool);
    #[doc = "Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SupportsInterfaceReturn(pub bool);
    #[doc = "Container type for all return fields from the `uri` function with signature `uri(uint256)` and selector `[14, 137, 52, 28]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct UriReturn(pub String);
}
