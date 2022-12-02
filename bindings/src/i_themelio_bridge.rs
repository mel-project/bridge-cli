pub use i_themelio_bridge::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_themelio_bridge {
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
    #[doc = "IThemelioBridge was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AdminChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ApprovalForAll\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"beacon\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"BeaconUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"height\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"HeaderVerified\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"stakesHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"StakesVerified\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"themelioRecipient\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32[]\",\"name\":\"txHashes\",\"type\":\"bytes32[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokensBurned\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TransferBatch\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TransferSingle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"height\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"txHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TxVerified\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"value\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"URI\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Upgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"accounts\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOfBatch\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"txHash_\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"themelioRecipient_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"txHashes_\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"themelioRecipient_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burnBatch\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"coins\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"denom\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"status\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"headerLimbo\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"votes\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"bytesVerified\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"stakeDocIndex\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"headers\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"transactionsHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"stakesHash\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockHeight_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"transactionsHash_\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"stakesHash_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isApprovedForAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proxiableUUID\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeBatchTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setApprovalForAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"stakesHashes\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgradeTo\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"upgradeToAndCall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"uri\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"verifierHeight_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"header_\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"stakes_\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"signatures_\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"verificationLimit_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"verifyHeader\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"stakes_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"verifyStakes\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"transaction_\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"txIndex_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blockHeight_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"proof_\",\"type\":\"bytes32[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"verifyTx\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static ITHEMELIOBRIDGE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IThemelioBridge<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IThemelioBridge<M> {
        fn clone(&self) -> Self {
            IThemelioBridge(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IThemelioBridge<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IThemelioBridge<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IThemelioBridge))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IThemelioBridge<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ITHEMELIOBRIDGE_ABI.clone(), client)
                .into()
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
        #[doc = "Calls the contract's `burn` (0x195f1cff) function"]
        pub fn burn(
            &self,
            account: ethers::core::types::Address,
            tx_hash: [u8; 32],
            themelio_recipient: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 95, 28, 255], (account, tx_hash, themelio_recipient))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burnBatch` (0x112ab37a) function"]
        pub fn burn_batch(
            &self,
            account: ethers::core::types::Address,
            tx_hashes: ::std::vec::Vec<[u8; 32]>,
            themelio_recipient: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([17, 42, 179, 122], (account, tx_hashes, themelio_recipient))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `coins` (0x3b1afd1f) function"]
        pub fn coins(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                [u8; 32],
            ),
        > {
            self.0
                .method_hash([59, 26, 253, 31], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `headerLimbo` (0xae567a4b) function"]
        pub fn header_limbo(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, (u128, u64, u64)> {
            self.0
                .method_hash([174, 86, 122, 75], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `headers` (0x56f90d79) function"]
        pub fn headers(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ([u8; 32], [u8; 32])> {
            self.0
                .method_hash([86, 249, 13, 121], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x09f10a3c) function"]
        pub fn initialize(
            &self,
            block_height: ethers::core::types::U256,
            transactions_hash: [u8; 32],
            stakes_hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [9, 241, 10, 60],
                    (block_height, transactions_hash, stakes_hash),
                )
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
        #[doc = "Calls the contract's `proxiableUUID` (0x52d1902d) function"]
        pub fn proxiable_uuid(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
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
        #[doc = "Calls the contract's `stakesHashes` (0x7d5dc7c8) function"]
        pub fn stakes_hashes(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([125, 93, 199, 200], p0)
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
        #[doc = "Calls the contract's `upgradeTo` (0x3659cfe6) function"]
        pub fn upgrade_to(
            &self,
            new_implementation: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 89, 207, 230], new_implementation)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgradeToAndCall` (0x4f1ef286) function"]
        pub fn upgrade_to_and_call(
            &self,
            new_implementation: ethers::core::types::Address,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 30, 242, 134], (new_implementation, data))
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
        #[doc = "Calls the contract's `verifyHeader` (0x6cf569a2) function"]
        pub fn verify_header(
            &self,
            verifier_height: ethers::core::types::U256,
            header: ethers::core::types::Bytes,
            stakes: ethers::core::types::Bytes,
            signatures: ::std::vec::Vec<[u8; 32]>,
            verification_limit: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [108, 245, 105, 162],
                    (
                        verifier_height,
                        header,
                        stakes,
                        signatures,
                        verification_limit,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifyStakes` (0x6c0fd94c) function"]
        pub fn verify_stakes(
            &self,
            stakes: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([108, 15, 217, 76], stakes)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifyTx` (0x42217b63) function"]
        pub fn verify_tx(
            &self,
            transaction: ethers::core::types::Bytes,
            tx_index: ethers::core::types::U256,
            block_height: ethers::core::types::U256,
            proof: ::std::vec::Vec<[u8; 32]>,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [66, 33, 123, 99],
                    (transaction, tx_index, block_height, proof),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AdminChanged` event"]
        pub fn admin_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AdminChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ApprovalForAll` event"]
        pub fn approval_for_all_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ApprovalForAllFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BeaconUpgraded` event"]
        pub fn beacon_upgraded_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BeaconUpgradedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `HeaderVerified` event"]
        pub fn header_verified_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, HeaderVerifiedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `StakesVerified` event"]
        pub fn stakes_verified_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, StakesVerifiedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokensBurned` event"]
        pub fn tokens_burned_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokensBurnedFilter> {
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
        #[doc = "Gets the contract's `TxVerified` event"]
        pub fn tx_verified_filter(&self) -> ethers::contract::builders::Event<M, TxVerifiedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `URI` event"]
        pub fn uri_filter(&self) -> ethers::contract::builders::Event<M, UriFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Upgraded` event"]
        pub fn upgraded_filter(&self) -> ethers::contract::builders::Event<M, UpgradedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IThemelioBridgeEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IThemelioBridge<M> {
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
    #[ethevent(name = "AdminChanged", abi = "AdminChanged(address,address)")]
    pub struct AdminChangedFilter {
        pub previous_admin: ethers::core::types::Address,
        pub new_admin: ethers::core::types::Address,
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
    #[ethevent(name = "BeaconUpgraded", abi = "BeaconUpgraded(address)")]
    pub struct BeaconUpgradedFilter {
        #[ethevent(indexed)]
        pub beacon: ethers::core::types::Address,
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
    #[ethevent(name = "HeaderVerified", abi = "HeaderVerified(uint256)")]
    pub struct HeaderVerifiedFilter {
        #[ethevent(indexed)]
        pub height: ethers::core::types::U256,
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
    #[ethevent(name = "StakesVerified", abi = "StakesVerified(bytes32)")]
    pub struct StakesVerifiedFilter {
        pub stakes_hash: [u8; 32],
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
    #[ethevent(name = "TokensBurned", abi = "TokensBurned(bytes32,bytes32[])")]
    pub struct TokensBurnedFilter {
        #[ethevent(indexed)]
        pub themelio_recipient: [u8; 32],
        pub tx_hashes: Vec<[u8; 32]>,
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
    #[ethevent(name = "TxVerified", abi = "TxVerified(uint256,bytes32)")]
    pub struct TxVerifiedFilter {
        #[ethevent(indexed)]
        pub height: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub tx_hash: [u8; 32],
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
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IThemelioBridgeEvents {
        AdminChangedFilter(AdminChangedFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        BeaconUpgradedFilter(BeaconUpgradedFilter),
        HeaderVerifiedFilter(HeaderVerifiedFilter),
        InitializedFilter(InitializedFilter),
        StakesVerifiedFilter(StakesVerifiedFilter),
        TokensBurnedFilter(TokensBurnedFilter),
        TransferBatchFilter(TransferBatchFilter),
        TransferSingleFilter(TransferSingleFilter),
        TxVerifiedFilter(TxVerifiedFilter),
        UriFilter(UriFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ethers::contract::EthLogDecode for IThemelioBridgeEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(IThemelioBridgeEvents::AdminChangedFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(IThemelioBridgeEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = BeaconUpgradedFilter::decode_log(log) {
                return Ok(IThemelioBridgeEvents::BeaconUpgradedFilter(decoded));
            }
            if let Ok(decoded) = HeaderVerifiedFilter::decode_log(log) {
                return Ok(IThemelioBridgeEvents::HeaderVerifiedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(IThemelioBridgeEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = StakesVerifiedFilter::decode_log(log) {
                return Ok(IThemelioBridgeEvents::StakesVerifiedFilter(decoded));
            }
            if let Ok(decoded) = TokensBurnedFilter::decode_log(log) {
                return Ok(IThemelioBridgeEvents::TokensBurnedFilter(decoded));
            }
            if let Ok(decoded) = TransferBatchFilter::decode_log(log) {
                return Ok(IThemelioBridgeEvents::TransferBatchFilter(decoded));
            }
            if let Ok(decoded) = TransferSingleFilter::decode_log(log) {
                return Ok(IThemelioBridgeEvents::TransferSingleFilter(decoded));
            }
            if let Ok(decoded) = TxVerifiedFilter::decode_log(log) {
                return Ok(IThemelioBridgeEvents::TxVerifiedFilter(decoded));
            }
            if let Ok(decoded) = UriFilter::decode_log(log) {
                return Ok(IThemelioBridgeEvents::UriFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(IThemelioBridgeEvents::UpgradedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IThemelioBridgeEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IThemelioBridgeEvents::AdminChangedFilter(element) => element.fmt(f),
                IThemelioBridgeEvents::ApprovalForAllFilter(element) => element.fmt(f),
                IThemelioBridgeEvents::BeaconUpgradedFilter(element) => element.fmt(f),
                IThemelioBridgeEvents::HeaderVerifiedFilter(element) => element.fmt(f),
                IThemelioBridgeEvents::InitializedFilter(element) => element.fmt(f),
                IThemelioBridgeEvents::StakesVerifiedFilter(element) => element.fmt(f),
                IThemelioBridgeEvents::TokensBurnedFilter(element) => element.fmt(f),
                IThemelioBridgeEvents::TransferBatchFilter(element) => element.fmt(f),
                IThemelioBridgeEvents::TransferSingleFilter(element) => element.fmt(f),
                IThemelioBridgeEvents::TxVerifiedFilter(element) => element.fmt(f),
                IThemelioBridgeEvents::UriFilter(element) => element.fmt(f),
                IThemelioBridgeEvents::UpgradedFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `burn` function with signature `burn(address,bytes32,bytes32)` and selector `[25, 95, 28, 255]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "burn", abi = "burn(address,bytes32,bytes32)")]
    pub struct BurnCall {
        pub account: ethers::core::types::Address,
        pub tx_hash: [u8; 32],
        pub themelio_recipient: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `burnBatch` function with signature `burnBatch(address,bytes32[],bytes32)` and selector `[17, 42, 179, 122]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "burnBatch", abi = "burnBatch(address,bytes32[],bytes32)")]
    pub struct BurnBatchCall {
        pub account: ethers::core::types::Address,
        pub tx_hashes: ::std::vec::Vec<[u8; 32]>,
        pub themelio_recipient: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `coins` function with signature `coins(bytes32)` and selector `[59, 26, 253, 31]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "coins", abi = "coins(bytes32)")]
    pub struct CoinsCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `headerLimbo` function with signature `headerLimbo(bytes32)` and selector `[174, 86, 122, 75]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "headerLimbo", abi = "headerLimbo(bytes32)")]
    pub struct HeaderLimboCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `headers` function with signature `headers(uint256)` and selector `[86, 249, 13, 121]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "headers", abi = "headers(uint256)")]
    pub struct HeadersCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(uint256,bytes32,bytes32)` and selector `[9, 241, 10, 60]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initialize", abi = "initialize(uint256,bytes32,bytes32)")]
    pub struct InitializeCall {
        pub block_height: ethers::core::types::U256,
        pub transactions_hash: [u8; 32],
        pub stakes_hash: [u8; 32],
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
    #[doc = "Container type for all input parameters for the `proxiableUUID` function with signature `proxiableUUID()` and selector `[82, 209, 144, 45]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "proxiableUUID", abi = "proxiableUUID()")]
    pub struct ProxiableUUIDCall;
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
    #[doc = "Container type for all input parameters for the `stakesHashes` function with signature `stakesHashes(bytes32)` and selector `[125, 93, 199, 200]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "stakesHashes", abi = "stakesHashes(bytes32)")]
    pub struct StakesHashesCall(pub [u8; 32]);
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
    #[doc = "Container type for all input parameters for the `upgradeTo` function with signature `upgradeTo(address)` and selector `[54, 89, 207, 230]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "upgradeTo", abi = "upgradeTo(address)")]
    pub struct UpgradeToCall {
        pub new_implementation: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `upgradeToAndCall` function with signature `upgradeToAndCall(address,bytes)` and selector `[79, 30, 242, 134]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "upgradeToAndCall", abi = "upgradeToAndCall(address,bytes)")]
    pub struct UpgradeToAndCallCall {
        pub new_implementation: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
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
    #[doc = "Container type for all input parameters for the `verifyHeader` function with signature `verifyHeader(uint256,bytes,bytes,bytes32[],uint256)` and selector `[108, 245, 105, 162]`"]
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
        name = "verifyHeader",
        abi = "verifyHeader(uint256,bytes,bytes,bytes32[],uint256)"
    )]
    pub struct VerifyHeaderCall {
        pub verifier_height: ethers::core::types::U256,
        pub header: ethers::core::types::Bytes,
        pub stakes: ethers::core::types::Bytes,
        pub signatures: ::std::vec::Vec<[u8; 32]>,
        pub verification_limit: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `verifyStakes` function with signature `verifyStakes(bytes)` and selector `[108, 15, 217, 76]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "verifyStakes", abi = "verifyStakes(bytes)")]
    pub struct VerifyStakesCall {
        pub stakes: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `verifyTx` function with signature `verifyTx(bytes,uint256,uint256,bytes32[])` and selector `[66, 33, 123, 99]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "verifyTx", abi = "verifyTx(bytes,uint256,uint256,bytes32[])")]
    pub struct VerifyTxCall {
        pub transaction: ethers::core::types::Bytes,
        pub tx_index: ethers::core::types::U256,
        pub block_height: ethers::core::types::U256,
        pub proof: ::std::vec::Vec<[u8; 32]>,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IThemelioBridgeCalls {
        BalanceOf(BalanceOfCall),
        BalanceOfBatch(BalanceOfBatchCall),
        Burn(BurnCall),
        BurnBatch(BurnBatchCall),
        Coins(CoinsCall),
        HeaderLimbo(HeaderLimboCall),
        Headers(HeadersCall),
        Initialize(InitializeCall),
        IsApprovedForAll(IsApprovedForAllCall),
        ProxiableUUID(ProxiableUUIDCall),
        SafeBatchTransferFrom(SafeBatchTransferFromCall),
        SafeTransferFrom(SafeTransferFromCall),
        SetApprovalForAll(SetApprovalForAllCall),
        StakesHashes(StakesHashesCall),
        SupportsInterface(SupportsInterfaceCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        Uri(UriCall),
        VerifyHeader(VerifyHeaderCall),
        VerifyStakes(VerifyStakesCall),
        VerifyTx(VerifyTxCall),
    }
    impl ethers::core::abi::AbiDecode for IThemelioBridgeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IThemelioBridgeCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IThemelioBridgeCalls::BalanceOfBatch(decoded));
            }
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IThemelioBridgeCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <BurnBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IThemelioBridgeCalls::BurnBatch(decoded));
            }
            if let Ok(decoded) = <CoinsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IThemelioBridgeCalls::Coins(decoded));
            }
            if let Ok(decoded) =
                <HeaderLimboCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IThemelioBridgeCalls::HeaderLimbo(decoded));
            }
            if let Ok(decoded) =
                <HeadersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IThemelioBridgeCalls::Headers(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IThemelioBridgeCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IThemelioBridgeCalls::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) =
                <ProxiableUUIDCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IThemelioBridgeCalls::ProxiableUUID(decoded));
            }
            if let Ok(decoded) =
                <SafeBatchTransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IThemelioBridgeCalls::SafeBatchTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IThemelioBridgeCalls::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SetApprovalForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IThemelioBridgeCalls::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) =
                <StakesHashesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IThemelioBridgeCalls::StakesHashes(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IThemelioBridgeCalls::SupportsInterface(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IThemelioBridgeCalls::UpgradeTo(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IThemelioBridgeCalls::UpgradeToAndCall(decoded));
            }
            if let Ok(decoded) = <UriCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IThemelioBridgeCalls::Uri(decoded));
            }
            if let Ok(decoded) =
                <VerifyHeaderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IThemelioBridgeCalls::VerifyHeader(decoded));
            }
            if let Ok(decoded) =
                <VerifyStakesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IThemelioBridgeCalls::VerifyStakes(decoded));
            }
            if let Ok(decoded) =
                <VerifyTxCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IThemelioBridgeCalls::VerifyTx(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IThemelioBridgeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IThemelioBridgeCalls::BalanceOf(element) => element.encode(),
                IThemelioBridgeCalls::BalanceOfBatch(element) => element.encode(),
                IThemelioBridgeCalls::Burn(element) => element.encode(),
                IThemelioBridgeCalls::BurnBatch(element) => element.encode(),
                IThemelioBridgeCalls::Coins(element) => element.encode(),
                IThemelioBridgeCalls::HeaderLimbo(element) => element.encode(),
                IThemelioBridgeCalls::Headers(element) => element.encode(),
                IThemelioBridgeCalls::Initialize(element) => element.encode(),
                IThemelioBridgeCalls::IsApprovedForAll(element) => element.encode(),
                IThemelioBridgeCalls::ProxiableUUID(element) => element.encode(),
                IThemelioBridgeCalls::SafeBatchTransferFrom(element) => element.encode(),
                IThemelioBridgeCalls::SafeTransferFrom(element) => element.encode(),
                IThemelioBridgeCalls::SetApprovalForAll(element) => element.encode(),
                IThemelioBridgeCalls::StakesHashes(element) => element.encode(),
                IThemelioBridgeCalls::SupportsInterface(element) => element.encode(),
                IThemelioBridgeCalls::UpgradeTo(element) => element.encode(),
                IThemelioBridgeCalls::UpgradeToAndCall(element) => element.encode(),
                IThemelioBridgeCalls::Uri(element) => element.encode(),
                IThemelioBridgeCalls::VerifyHeader(element) => element.encode(),
                IThemelioBridgeCalls::VerifyStakes(element) => element.encode(),
                IThemelioBridgeCalls::VerifyTx(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IThemelioBridgeCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IThemelioBridgeCalls::BalanceOf(element) => element.fmt(f),
                IThemelioBridgeCalls::BalanceOfBatch(element) => element.fmt(f),
                IThemelioBridgeCalls::Burn(element) => element.fmt(f),
                IThemelioBridgeCalls::BurnBatch(element) => element.fmt(f),
                IThemelioBridgeCalls::Coins(element) => element.fmt(f),
                IThemelioBridgeCalls::HeaderLimbo(element) => element.fmt(f),
                IThemelioBridgeCalls::Headers(element) => element.fmt(f),
                IThemelioBridgeCalls::Initialize(element) => element.fmt(f),
                IThemelioBridgeCalls::IsApprovedForAll(element) => element.fmt(f),
                IThemelioBridgeCalls::ProxiableUUID(element) => element.fmt(f),
                IThemelioBridgeCalls::SafeBatchTransferFrom(element) => element.fmt(f),
                IThemelioBridgeCalls::SafeTransferFrom(element) => element.fmt(f),
                IThemelioBridgeCalls::SetApprovalForAll(element) => element.fmt(f),
                IThemelioBridgeCalls::StakesHashes(element) => element.fmt(f),
                IThemelioBridgeCalls::SupportsInterface(element) => element.fmt(f),
                IThemelioBridgeCalls::UpgradeTo(element) => element.fmt(f),
                IThemelioBridgeCalls::UpgradeToAndCall(element) => element.fmt(f),
                IThemelioBridgeCalls::Uri(element) => element.fmt(f),
                IThemelioBridgeCalls::VerifyHeader(element) => element.fmt(f),
                IThemelioBridgeCalls::VerifyStakes(element) => element.fmt(f),
                IThemelioBridgeCalls::VerifyTx(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BalanceOfCall> for IThemelioBridgeCalls {
        fn from(var: BalanceOfCall) -> Self {
            IThemelioBridgeCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BalanceOfBatchCall> for IThemelioBridgeCalls {
        fn from(var: BalanceOfBatchCall) -> Self {
            IThemelioBridgeCalls::BalanceOfBatch(var)
        }
    }
    impl ::std::convert::From<BurnCall> for IThemelioBridgeCalls {
        fn from(var: BurnCall) -> Self {
            IThemelioBridgeCalls::Burn(var)
        }
    }
    impl ::std::convert::From<BurnBatchCall> for IThemelioBridgeCalls {
        fn from(var: BurnBatchCall) -> Self {
            IThemelioBridgeCalls::BurnBatch(var)
        }
    }
    impl ::std::convert::From<CoinsCall> for IThemelioBridgeCalls {
        fn from(var: CoinsCall) -> Self {
            IThemelioBridgeCalls::Coins(var)
        }
    }
    impl ::std::convert::From<HeaderLimboCall> for IThemelioBridgeCalls {
        fn from(var: HeaderLimboCall) -> Self {
            IThemelioBridgeCalls::HeaderLimbo(var)
        }
    }
    impl ::std::convert::From<HeadersCall> for IThemelioBridgeCalls {
        fn from(var: HeadersCall) -> Self {
            IThemelioBridgeCalls::Headers(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for IThemelioBridgeCalls {
        fn from(var: InitializeCall) -> Self {
            IThemelioBridgeCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<IsApprovedForAllCall> for IThemelioBridgeCalls {
        fn from(var: IsApprovedForAllCall) -> Self {
            IThemelioBridgeCalls::IsApprovedForAll(var)
        }
    }
    impl ::std::convert::From<ProxiableUUIDCall> for IThemelioBridgeCalls {
        fn from(var: ProxiableUUIDCall) -> Self {
            IThemelioBridgeCalls::ProxiableUUID(var)
        }
    }
    impl ::std::convert::From<SafeBatchTransferFromCall> for IThemelioBridgeCalls {
        fn from(var: SafeBatchTransferFromCall) -> Self {
            IThemelioBridgeCalls::SafeBatchTransferFrom(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromCall> for IThemelioBridgeCalls {
        fn from(var: SafeTransferFromCall) -> Self {
            IThemelioBridgeCalls::SafeTransferFrom(var)
        }
    }
    impl ::std::convert::From<SetApprovalForAllCall> for IThemelioBridgeCalls {
        fn from(var: SetApprovalForAllCall) -> Self {
            IThemelioBridgeCalls::SetApprovalForAll(var)
        }
    }
    impl ::std::convert::From<StakesHashesCall> for IThemelioBridgeCalls {
        fn from(var: StakesHashesCall) -> Self {
            IThemelioBridgeCalls::StakesHashes(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for IThemelioBridgeCalls {
        fn from(var: SupportsInterfaceCall) -> Self {
            IThemelioBridgeCalls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<UpgradeToCall> for IThemelioBridgeCalls {
        fn from(var: UpgradeToCall) -> Self {
            IThemelioBridgeCalls::UpgradeTo(var)
        }
    }
    impl ::std::convert::From<UpgradeToAndCallCall> for IThemelioBridgeCalls {
        fn from(var: UpgradeToAndCallCall) -> Self {
            IThemelioBridgeCalls::UpgradeToAndCall(var)
        }
    }
    impl ::std::convert::From<UriCall> for IThemelioBridgeCalls {
        fn from(var: UriCall) -> Self {
            IThemelioBridgeCalls::Uri(var)
        }
    }
    impl ::std::convert::From<VerifyHeaderCall> for IThemelioBridgeCalls {
        fn from(var: VerifyHeaderCall) -> Self {
            IThemelioBridgeCalls::VerifyHeader(var)
        }
    }
    impl ::std::convert::From<VerifyStakesCall> for IThemelioBridgeCalls {
        fn from(var: VerifyStakesCall) -> Self {
            IThemelioBridgeCalls::VerifyStakes(var)
        }
    }
    impl ::std::convert::From<VerifyTxCall> for IThemelioBridgeCalls {
        fn from(var: VerifyTxCall) -> Self {
            IThemelioBridgeCalls::VerifyTx(var)
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
    #[doc = "Container type for all return fields from the `coins` function with signature `coins(bytes32)` and selector `[59, 26, 253, 31]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CoinsReturn {
        pub denom: ethers::core::types::U256,
        pub value: ethers::core::types::U256,
        pub status: [u8; 32],
    }
    #[doc = "Container type for all return fields from the `headerLimbo` function with signature `headerLimbo(bytes32)` and selector `[174, 86, 122, 75]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct HeaderLimboReturn {
        pub votes: u128,
        pub bytes_verified: u64,
        pub stake_doc_index: u64,
    }
    #[doc = "Container type for all return fields from the `headers` function with signature `headers(uint256)` and selector `[86, 249, 13, 121]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct HeadersReturn {
        pub transactions_hash: [u8; 32],
        pub stakes_hash: [u8; 32],
    }
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
    #[doc = "Container type for all return fields from the `proxiableUUID` function with signature `proxiableUUID()` and selector `[82, 209, 144, 45]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ProxiableUUIDReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `stakesHashes` function with signature `stakesHashes(bytes32)` and selector `[125, 93, 199, 200]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct StakesHashesReturn(pub [u8; 32]);
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
    #[doc = "Container type for all return fields from the `verifyHeader` function with signature `verifyHeader(uint256,bytes,bytes,bytes32[],uint256)` and selector `[108, 245, 105, 162]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VerifyHeaderReturn(pub bool);
    #[doc = "Container type for all return fields from the `verifyStakes` function with signature `verifyStakes(bytes)` and selector `[108, 15, 217, 76]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VerifyStakesReturn(pub bool);
    #[doc = "Container type for all return fields from the `verifyTx` function with signature `verifyTx(bytes,uint256,uint256,bytes32[])` and selector `[66, 33, 123, 99]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VerifyTxReturn(pub bool);
}
