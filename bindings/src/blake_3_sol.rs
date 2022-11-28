pub use blake_3_sol::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod blake_3_sol {
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
    #[doc = "Blake3Sol was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"struct Blake3Sol.Hasher\",\"name\":\"self\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Blake3Sol.ChunkState\",\"name\":\"chunk_state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32[8]\",\"name\":\"chaining_value\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"chunk_counter\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"block_bytes\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"block_len\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blocks_compressed\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]},{\"internalType\":\"uint32[8]\",\"name\":\"key_words\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint32[8][54]\",\"name\":\"cv_stack\",\"type\":\"uint32[8][54]\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"cv_stack_len\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"finalize\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"context\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"new_derive_key\",\"outputs\":[{\"internalType\":\"struct Blake3Sol.Hasher\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Blake3Sol.ChunkState\",\"name\":\"chunk_state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32[8]\",\"name\":\"chaining_value\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"chunk_counter\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"block_bytes\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"block_len\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blocks_compressed\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]},{\"internalType\":\"uint32[8]\",\"name\":\"key_words\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint32[8][54]\",\"name\":\"cv_stack\",\"type\":\"uint32[8][54]\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"cv_stack_len\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"new_hasher\",\"outputs\":[{\"internalType\":\"struct Blake3Sol.Hasher\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Blake3Sol.ChunkState\",\"name\":\"chunk_state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32[8]\",\"name\":\"chaining_value\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"chunk_counter\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"block_bytes\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"block_len\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blocks_compressed\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]},{\"internalType\":\"uint32[8]\",\"name\":\"key_words\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint32[8][54]\",\"name\":\"cv_stack\",\"type\":\"uint32[8][54]\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"cv_stack_len\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"key\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"new_keyed\",\"outputs\":[{\"internalType\":\"struct Blake3Sol.Hasher\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Blake3Sol.ChunkState\",\"name\":\"chunk_state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32[8]\",\"name\":\"chaining_value\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"chunk_counter\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"block_bytes\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"block_len\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blocks_compressed\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]},{\"internalType\":\"uint32[8]\",\"name\":\"key_words\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint32[8][54]\",\"name\":\"cv_stack\",\"type\":\"uint32[8][54]\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"cv_stack_len\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct Blake3Sol.Hasher\",\"name\":\"self\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Blake3Sol.ChunkState\",\"name\":\"chunk_state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32[8]\",\"name\":\"chaining_value\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"chunk_counter\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"block_bytes\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"block_len\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blocks_compressed\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]},{\"internalType\":\"uint32[8]\",\"name\":\"key_words\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint32[8][54]\",\"name\":\"cv_stack\",\"type\":\"uint32[8][54]\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"cv_stack_len\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"input\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"update_hasher\",\"outputs\":[{\"internalType\":\"struct Blake3Sol.Hasher\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Blake3Sol.ChunkState\",\"name\":\"chunk_state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32[8]\",\"name\":\"chaining_value\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"chunk_counter\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"block_bytes\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"block_len\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blocks_compressed\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]},{\"internalType\":\"uint32[8]\",\"name\":\"key_words\",\"type\":\"uint32[8]\",\"components\":[]},{\"internalType\":\"uint32[8][54]\",\"name\":\"cv_stack\",\"type\":\"uint32[8][54]\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"cv_stack_len\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"flags\",\"type\":\"uint32\",\"components\":[]}]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static BLAKE3SOL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static BLAKE3SOL_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080806040523461001a5761242c9081610020823930815050f35b600080fdfe60806040526004361015610013575b600080fd5b6000803560e01c9081630d056d9b1461007a5750806312a63b5d146100715780635163dae5146100685780639301d7021461005f5763c51d2cd71461005757600080fd5b61000e6107cc565b5061000e610779565b5061000e61039d565b5061000e6102a1565b806003193601126100e6576100e290610091610915565b5061009a610958565b906100a3610915565b506100ac6108e2565b6100b583610a55565b926100be61043e565b93845260208401526040830152806060830152608082015260405191829182610199565b0390f35b80fd5b6000915b600883106100fa57505050565b60019063ffffffff835116815260208091019201920191906100ed565b91908251928382526000905b848210610150575092806020939411610143575b601f01601f1916010190565b6000838284010152610137565b90602090818082850101519082860101520190610123565b906000905b6036821061017a57505050565b60206101008261018d60019487516100e9565b0193019101909161016d565b61025590602081528251926080613760918260208501526101bf613780850187516100e9565b6001600160401b0360208701511661388085015263ffffffff60a06101f660408901516101a06138a0890152613920880190610117565b9760608101516138c0880152848101516138e0880152015116613900850152610227602082015160408601906100e9565b61023a6040820151610140860190610168565b606081015160ff16613740850152015163ffffffff16910152565b90565b90602060031983011261000e576004356001600160401b039283821161000e578060238301121561000e57816004013593841161000e576024848301011161000e576024019190565b506100e261039161038c6102b436610258565b91906102be610915565b506103596103536102cd610958565b926102d6610915565b506102df6108e2565b956102e86108a4565b506102f1610509565b9660408852602095604036888b013761030861046a565b98818a526000888b015260408a0152600060608a0152600060808a01528660a08a015261033361043e565b9889528689015260408801526000606088015284608088015236916105e5565b84610e76565b50610379610365610489565b809461010080948184523690840137612046565b6103816104a9565b928391368337610bc4565b6109be565b60405191829182610199565b506003196020368201811361000e57600435916001600160401b039081841161000e5761376090843603011261000e576104069061040160405194859260408401908482109082111761041a575b604052848352843681850137369060040161068c565b612046565b6100e2604051928284938452830190610117565b610422610427565b6103eb565b50634e487b7160e01b600052604160045260246000fd5b6040519060a082018281106001600160401b0382111761045d57604052565b610465610427565b604052565b6040519060c082018281106001600160401b0382111761045d57604052565b6040519061012082018281106001600160401b0382111761045d57604052565b6040519061010082018281106001600160401b0382111761045d57604052565b604051906106c082018281106001600160401b0382111761045d57604052565b6040519061020082018281106001600160401b0382111761045d57604052565b60405190606082018281106001600160401b0382111761045d57604052565b6040519190601f01601f191682016001600160401b0381118382101761045d57604052565b359063ffffffff8216820361000e57565b9080601f8301121561000e576105726104a9565b8092610100810192831161000e57905b82821061058f5750505090565b6020809161059c8461054d565b815201910190610582565b35906001600160401b038216820361000e57565b6020906001600160401b0381116105d8575b601f01601f19160190565b6105e0610427565b6105cd565b9291926105f96105f4836105bb565b610528565b938285528282011161000e57816000926020928387013784010152565b9080601f8301121561000e57816020610255933591016105e5565b81601f8201121561000e576106446104c9565b809261360083019281841161000e57915b838310610663575050505090565b602061010091610673848661055e565b815201920191610655565b359060ff8216820361000e57565b91906137608382031261000e576106a161043e565b926001600160401b0390803582811161000e578101916101a08385031261000e576106ca61046a565b936106d5818561055e565b85526106e461010085016105a7565b60208601526101208085013592831161000e57610772956107356101808761071486610755986137409b01610616565b6040850152610140810135606085015261016081013560808501520161054d565b60a08201528852610749826020860161055e565b60208901528301610631565b6040860152610767613720820161067e565b60608601520161054d565b6080830152565b50604036600319011261000e576001600160401b0360043581811161000e576107a690369060040161068c565b9060243590811161000e576100e2916107c6610391923690600401610616565b90610e76565b506100e26108086107dc36610258565b6107e7929192610915565b506108036107f36104a9565b93849261010036853736916105e5565b610bc4565b610810610915565b506108196108e2565b6108216108a4565b5061082a610509565b9160408352604036602085013761083f61046a565b928184526000602085015260408401526000606084015260006080840152601060a084015261086c61043e565b92835260208301526040820152600060608201526010608082015260405191829182610199565b61089b6104e9565b90610200368337565b6108ac61046a565b906108b56104a9565b610100368237825260006020830152606060408301526000606083015260006080830152600060a0830152565b6108ea6104c9565b9060005b6106c081106108fa5750565b6020906109056104a9565b61010036823781850152016108ee565b61091d61043e565b906109266108a4565b82526109306104a9565b61010036823760208301526109436108e2565b60408301526000606083015260006080830152565b6101006109636104a9565b36903761096e6104a9565b636a09e667815263bb67ae856020820152633c6ef372604082015263a54ff53a606082015263510e527f6080820152639b05688c60a0820152631f83d9ab60c0820152635be0cd1960e082015290565b6109c6610915565b506109cf6108e2565b906109d86108a4565b506109e1610509565b91604091828452823660208601376109f761046a565b93818552600060208601528385015260006060850152600060808501528260a0850152610a2261043e565b93845260208401528183015260006060830152608082015290565b610a45610509565b9060408252604082602036910137565b610a5d6108a4565b50610a66610509565b604081526040366020830137610a7a61046a565b9182526000602083015260408201526000606082015260006080820152600060a082015290565b9163ffffffff91610ab06108a4565b506001600160401b03610ac1610509565b91604083526040366020850137610ad661046a565b958652166020850152604084015260006060840152600060808401521660a082015290565b50634e487b7160e01b600052601160045260246000fd5b6001906000198114610b22570190565b610b2a610afb565b0190565b50634e487b7160e01b600052601260045260246000fd5b806000190460041181151516610b5c575b60021b90565b610b64610afb565b610b56565b6001600160fd1b038111600116610b81575b60031b90565b610b89610afb565b610b7b565b50634e487b7160e01b600052603260045260246000fd5b906008811015610bb7575b60051b0190565b610bbf610b8e565b610bb0565b91906020835111610c175760005b835160021c811015610c115780610bf4610bee610c0c93610b45565b86610d56565b63ffffffff610c038386610ba5565b91169052610b12565b610bd2565b50509050565b60405162461bcd60e51b815260206004820152603360248201527f4461746120627974657320697320746f6f206c6f6e6720746f20636f6e7665726044820152727420746f203820342d6279746520776f72647360681b6064820152608490fd5b6008906008198111610b22570190565b6001906001198111610b22570190565b81198111610b22570190565b80600310610cb3575b60030390565b610cbb610afb565b610cad565b610400818110610cce570390565b610cd6610afb565b0390565b80604010610ce9575b60400390565b610cf1610afb565b610ce3565b60088110610d06575b6007190190565b610d0e610afb565b610cff565b818110610cce570390565b906020918051821015610d3057010190565b610d38610b8e565b010190565b9063ffffffff8080931691168092038111610b22570190565b8051906004198311610e4a575b60049182840111610e0657919060009283925b828410610d84575050505090565b90919293610df8610dfe91610df2610dec610dd1610dc4610db6610db089610dab8e610ca4565b610c98565b8a610d1e565b516001600160f81b03191690565b6001600160f81b03191690565b610dda8a610b69565b6001600160e01b0319918216901c1690565b60e01c90565b90610d3d565b94610b12565b929190610d76565b60405162461bcd60e51b8152602081840152601f60248201527f6c655f62797465735f6765745f75696e7433325f6f75744f66426f756e6473006044820152606490fd5b610e52610afb565b610d63565b6001600160401b036001911667fffffffffffffffe8111610b22570190565b91610e7f610915565b5060005b8251811015610f5d57610ef790610400610e9d8651611b26565b14610efc575b610ed8610eb8610eb38751611b26565b610cc0565b610ed2610ec9610ec9858951610d13565b63ffffffff1690565b90611d47565b90610dab610ef0610ee98385610c98565b8388611fc2565b8751611b5c565b610e83565b610f56610f11610f0c8751611d9c565b610f62565b6020610f3b610f33610f2e838b51016001600160401b0390511690565b610e57565b80938a611e17565b87015190610f50608089015163ffffffff1690565b91610aa1565b8552610ea3565b509050565b610f6a6104a9565b906101008092369037805190602081015190608060408201519160608101519063ffffffff8093819201511693610f9f610893565b50610fa8610958565b610fb0610893565b9660005b601081106111f05750508290610fc86104e9565b8951831663ffffffff1681529960208a810151841663ffffffff16908c015260408a810151841663ffffffff16908c015260608a810151841663ffffffff16908c015260808a810151841663ffffffff16908c015260a08a810151841663ffffffff16908c015260c08a810151841663ffffffff16908c015260e08a810151841663ffffffff16908c01528151831663ffffffff16908b01526020810151821663ffffffff166101208b01526040810151821663ffffffff166101408b0152606001511663ffffffff1661016089015263ffffffff8183161661018089015260201c166101a08701906110c0919063ffffffff169052565b1663ffffffff166101c085015263ffffffff166101e08401526110e38184611476565b6110ec816119a3565b6110f68184611476565b6110ff816119a3565b6111098184611476565b611112816119a3565b61111c8184611476565b611125816119a3565b61112f8184611476565b611138816119a3565b6111428184611476565b61114b816119a3565b6111559083611476565b60005b6008811061116b57505061025590611acf565b806111b46111916111876111816111eb95610c78565b8761121c565b5163ffffffff1690565b61119e611187848861121c565b186111a9838761121c565b9063ffffffff169052565b6111e66111c46111878386610ba5565b6111a96111d084610c78565b916111de611187848a61121c565b18918761121c565b610b12565b611158565b81929394506111e6611208611187836112129561121c565b6111a9838c61121c565b9085939291610fb4565b906010811015610bb75760051b0190565b9192611237610893565b50611240610958565b93611249610893565b9260005b60108110611453575050611428929161139a60e06113b0936113778860606112736104e9565b9b6113308d8763ffffffff80998761129583809a5116869063ffffffff169052565b6112ae83602083015116602087019063ffffffff169052565b6112c783604083015116604087019063ffffffff169052565b87810151831663ffffffff16858901526112f083608083015116608087019063ffffffff169052565b6113098360a08301511660a087019063ffffffff169052565b6113228360c08301511660c087019063ffffffff169052565b01511663ffffffff16910152565b8051831663ffffffff166101008e01526020810151831663ffffffff166101208e01526040810151831663ffffffff166101408e015201511663ffffffff166101608b0152565b63ffffffff818316166101808a015260201c166101a088019063ffffffff169052565b60406101c087015263ffffffff166101e0860152565b6113ba8185611476565b6113c3816119a3565b6113cd8185611476565b6113d6816119a3565b6113e08185611476565b6113e9816119a3565b6113f38185611476565b6113fc816119a3565b6114068185611476565b61140f816119a3565b6114198185611476565b611422816119a3565b83611476565b60005b6008811061143857505090565b806111b461119161118761118161144e95610c78565b61142b565b806111e6611467611187611471948661121c565b6111a9838961121c565b61124d565b61153f9163ffffffff61149481835116826020850151169085611541565b6114ac816040840151168260608501511690856116ab565b6114c4816080840151168260a0850151169085611734565b6114dc8160c0840151168260e0850151169085611798565b6114f68161010084015116826101208501511690856117fc565b611510816101408401511682610160850151169085611877565b61152a8161018084015116826101a08501511690856118db565b6101e0816101c084015116920151169161193f565b565b61153f926116199161168261167863ffffffff9261158b6115818583511698611577608085019a888c511663ffffffff91011690565b0163ffffffff1690565b63ffffffff168252565b6116656115816101808301926115b56115ab888651168984511618611aac565b63ffffffff168552565b6116386115816101008301996115e46115da8b8d51168c8a511663ffffffff91011690565b63ffffffff168c52565b6116238d6116198d8d808451169151161860d481901b620fffff60e01b1660f49190911b6001600160f41b0319161760e01c90565b63ffffffff169052565b83518d518b16908b160163ffffffff16611577565b86808551169151161860d881901b62ffffff60e01b1660f89190911b6001600160f81b0319161760e01c90565b838086511691511663ffffffff91011690565b63ffffffff168352565b808451169151161860d981901b6301ffffff60e01b1660f99190911b607f60f91b161760e01c90565b61153f926115819261168261167861161994611665611581602083016116ec63ffffffff9a8b986115778a85511660a089019e8f511663ffffffff91011690565b6116386115816101206101a087019661171961170f8c8a51168d88511618611aac565b63ffffffff168952565b01996115e46115da8b8d51168c8a511663ffffffff91011690565b61153f9261158192611682611678611619946116656115816040830161177563ffffffff9a8b986115778a85511660c089019e8f511663ffffffff91011690565b6116386115816101406101c087019661171961170f8c8a51168d88511618611aac565b61153f926115819261168261167861161994611665611581606083016117d963ffffffff9a8b986115778a85511660e089019e8f511663ffffffff91011690565b6116386115816101606101e087019661171961170f8c8a51168d88511618611aac565b61153f926116199161168261167863ffffffff92611832611581858351169861157760a085019a888c511663ffffffff91011690565b6116656115816101e08301926118526115ab888651168984511618611aac565b6116386115816101408301996115e46115da8b8d51168c8a511663ffffffff91011690565b61153f926115819261168261167861161994611665611581602083016118b863ffffffff9a8b986115778a85511660c089019e8f511663ffffffff91011690565b61163861158161016061018087019661171961170f8c8a51168d88511618611aac565b61153f9261158192611682611678611619946116656115816040830161191c63ffffffff9a8b986115778a85511660e089019e8f511663ffffffff91011690565b6116386115816101006101a087019661171961170f8c8a51168d88511618611aac565b61153f9261158192611682611678611619946116656115816060830161198063ffffffff9a8b986115778a855116608089019e8f511663ffffffff91011690565b6116386115816101206101c087019661171961170f8c8a51168d88511618611aac565b6119ab610893565b506119b46104e9565b60028152906006602083015260036040830152600a606083015260076080830152600060a0830152600460c0830152600d60e08301526001610100830152600b610120830152600c61014083015260056101608301526009610180830152600e6101a0830152600f6101c083015260086101e0830152611a32610893565b9160005b60108110611a7357505060005b60108110611a5057505050565b806111e6611a64611187611a6e948761121c565b6111a9838661121c565b611a43565b806111e6611a9d611187611181611a97611a90611aa7978961121c565b5160ff1690565b60ff1690565b6111a9838861121c565b611a36565b60d081901b61ffff60e01b1660f09190911b6001600160f01b0319161760e01c90565b90611ad86104a9565b6101008091369037611ae86104a9565b9036823760005b60088110611afc57509150565b8063ffffffff611b0f611b21938761121c565b5116611b1b8285610ba5565b52610b12565b611aef565b6080810151610255916060906001600160fa1b038311600116611b4f575b01519060061b610c98565b611b57610afb565b611b44565b9060005b8151811015611c7e57606083019060409182815114611bfe575b611b92611b878251610cda565b610ed2848751610d13565b9260005b848110611bbc5750509081611bb084611bb7959451610c98565b9052610c98565b611b60565b80611bd6610db6611bd0611bf99488610c98565b89610d1e565b611bf3848a015191611be9875185610c98565b9060001a92610d1e565b53610b12565b611b96565b611c06610893565b611c59611c5485880192611c1b818551611c83565b885190611c3260208b01516001600160401b031690565b90611c4460a08c015163ffffffff1690565b611c4d8c611d35565b179261122d565b611acf565b865260808601611c698151610c88565b9052611c73610a3d565b905260008152611b7a565b505050565b919091805160408111159081611d29575b5015611cc75760005b815160021c811015610c1157806111e6611a9d611cbc611cc294610b45565b85610d56565b611c9d565b60405162461bcd60e51b815260206004820152603460248201527f4461746120627974657320697320746f6f206c6f6e6720746f20636f6e7665726044820152737420746f20313620342d6279746520776f72647360601b6064820152608490fd5b60039150161538611c94565b60800151611d4257600190565b600090565b9080821015611d54575090565b905090565b611d6161043e565b90611d6a6104a9565b6101003682378252611d7a6104e9565b6102003682376020830152600060408301526000606083015260006080830152565b611da4611d59565b50611dad6104e9565b90610200368337611dc2826040830151611c83565b8051916001600160401b03602083015116606083015190600263ffffffff94611df18660a08301511691611d35565b171792611dfc61043e565b95865260208601526040850152606084015216608082015290565b915b600191828216611e995790611e84677fffffffffffffff9261010036611e3d6104a9565b37611e656060870160ff80825116888110611e8c575b60001901168091526040880151611f67565b5190602087015190611e7e608089015163ffffffff1690565b92611ea5565b921c16611e19565b611e94610afb565b611e53565b91505061153f91611f78565b916102559391610f0c93610100611eba6104a9565b3690375b90929192611eca611d59565b50611ed3610893565b9160005b60088110611f4e57505060085b60108110611f235750509061025591611efb61043e565b93845260208401526000604084015260406060840152600417608083019063ffffffff169052565b806111e6611f3f611187611f39611f4995610cf6565b86610ba5565b6111a9838761121c565b611ee4565b806111e6611a9d611187611f629486610ba5565b611ed7565b906036811015610bb75760051b0190565b90611f9d906060604084015193019260ff84511691611f978383611f67565b52611f67565b5060ff60018183511660fe8111611fb5575b01169052565b611fbd610afb565b611faf565b908092818110612039575b0390611fdb6105f4836105bb565b82815292601f19611feb846105bb565b01366020860137600091825b84811061200657505050505090565b612034906001600160f81b03196120266120208387610c98565b85610d1e565b5116851a611bf38289610d1e565b611ff7565b612041610afb565b611fcd565b9190916120538151611d9c565b60ff606083015116905b63ffffffff80921680156120bd57908160016120ab93106120b0575b6000190192612099612092604087015183871690611f67565b5192610f62565b60208601519160808701511692611ebe565b61205d565b6120b8610afb565b612079565b50805160208201516060830151608090930151959663ffffffff9690955060089450861684176120eb610893565b50866120f5610958565b946120fe610893565b9360005b60108110612309575050816121156104e9565b8651821663ffffffff16815296602087810151831663ffffffff1690890152604087810151831663ffffffff1690890152606087810151831663ffffffff1690890152608087810151831663ffffffff169089015260a087810151831663ffffffff169089015260c087810151831663ffffffff169089015260e087810151831663ffffffff16908901528051821663ffffffff166101008901526020810151821663ffffffff166101208901526040810151821663ffffffff16610140890152606001511663ffffffff16610160870152600061018087018190526101a08701521663ffffffff166101c085015263ffffffff166101e084015261221a8184611476565b612223816119a3565b61222d8184611476565b612236816119a3565b6122408184611476565b612249816119a3565b6122538184611476565b61225c816119a3565b6122668184611476565b61226f816119a3565b6122798184611476565b612282816119a3565b61228c9083611476565b60005b8381106122ee57505060005b848116838110156122e6576001916122ca6122ba61118789948761121c565b876122c484612329565b9161235a565b168581146122d9575b0161229b565b6122e1610afb565b6122d3565b505050505050565b806111b461119161118761118161230495610c78565b61228f565b819293506111e6611467611187836123209561121c565b90899291612102565b6403fffffffc9063ffffffff80821680151591046004111661234d575b60021b1690565b612355610afb565b612346565b916000805b6004811061236e575050505050565b806123ca9160001904600811811515166123e9575b8060031b60ff81116123dc575b6001811b156123cf575b611bf363ffffffff6123ae84828916610c98565b90891690921c60f81b6001600160f81b031916851a9187610d1e565b61235f565b6123d7610b2e565b61239a565b6123e4610afb565b612390565b6123f1610afb565b61238356fea2646970667358221220e40bc0efe8f85267e808dbbd09b4d773c19e41ccf3b1c89ab5857213f326678c64736f6c634300080d0033" . parse () . expect ("invalid bytecode")
        });
    pub struct Blake3Sol<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Blake3Sol<M> {
        fn clone(&self) -> Self {
            Blake3Sol(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Blake3Sol<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Blake3Sol<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Blake3Sol))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Blake3Sol<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), BLAKE3SOL_ABI.clone(), client).into()
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
                BLAKE3SOL_ABI.clone(),
                BLAKE3SOL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `finalize` (0xb787e794) function"]
        pub fn finalize(
            &self,
            self_: Hasher,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([183, 135, 231, 148], (self_,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `new_derive_key` (0x12a63b5d) function"]
        pub fn new_derive_key(
            &self,
            context: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, Hasher> {
            self.0
                .method_hash([18, 166, 59, 93], context)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `new_hasher` (0x0d056d9b) function"]
        pub fn new_hasher(&self) -> ethers::contract::builders::ContractCall<M, Hasher> {
            self.0
                .method_hash([13, 5, 109, 155], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `new_keyed` (0xc51d2cd7) function"]
        pub fn new_keyed(
            &self,
            key: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, Hasher> {
            self.0
                .method_hash([197, 29, 44, 215], key)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `update_hasher` (0xb2032f74) function"]
        pub fn update_hasher(
            &self,
            self_: Hasher,
            input: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, Hasher> {
            self.0
                .method_hash([178, 3, 47, 116], (self_, input))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Blake3Sol<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `finalize` function with signature `finalize(((uint32[8],uint64,bytes,uint256,uint256,uint32),uint32[8],uint32[8][54],uint8,uint32))` and selector `[183, 135, 231, 148]`"]
    #[derive(
        Clone, Debug, Eq, PartialEq, ethers :: contract :: EthCall, ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "finalize",
        abi = "finalize(((uint32[8],uint64,bytes,uint256,uint256,uint32),uint32[8],uint32[8][54],uint8,uint32))"
    )]
    pub struct FinalizeCall {
        pub self_: Hasher,
    }
    #[doc = "Container type for all input parameters for the `new_derive_key` function with signature `new_derive_key(bytes)` and selector `[18, 166, 59, 93]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "new_derive_key", abi = "new_derive_key(bytes)")]
    pub struct NewDeriveKeyCall {
        pub context: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `new_hasher` function with signature `new_hasher()` and selector `[13, 5, 109, 155]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "new_hasher", abi = "new_hasher()")]
    pub struct NewHasherCall;
    #[doc = "Container type for all input parameters for the `new_keyed` function with signature `new_keyed(bytes)` and selector `[197, 29, 44, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "new_keyed", abi = "new_keyed(bytes)")]
    pub struct NewKeyedCall {
        pub key: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `update_hasher` function with signature `update_hasher(((uint32[8],uint64,bytes,uint256,uint256,uint32),uint32[8],uint32[8][54],uint8,uint32),bytes)` and selector `[178, 3, 47, 116]`"]
    #[derive(
        Clone, Debug, Eq, PartialEq, ethers :: contract :: EthCall, ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "update_hasher",
        abi = "update_hasher(((uint32[8],uint64,bytes,uint256,uint256,uint32),uint32[8],uint32[8][54],uint8,uint32),bytes)"
    )]
    pub struct UpdateHasherCall {
        pub self_: Hasher,
        pub input: ethers::core::types::Bytes,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum Blake3SolCalls {
        Finalize(FinalizeCall),
        NewDeriveKey(NewDeriveKeyCall),
        NewHasher(NewHasherCall),
        NewKeyed(NewKeyedCall),
        UpdateHasher(UpdateHasherCall),
    }
    impl ethers::core::abi::AbiDecode for Blake3SolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <FinalizeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(Blake3SolCalls::Finalize(decoded));
            }
            if let Ok(decoded) =
                <NewDeriveKeyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(Blake3SolCalls::NewDeriveKey(decoded));
            }
            if let Ok(decoded) =
                <NewHasherCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(Blake3SolCalls::NewHasher(decoded));
            }
            if let Ok(decoded) =
                <NewKeyedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(Blake3SolCalls::NewKeyed(decoded));
            }
            if let Ok(decoded) =
                <UpdateHasherCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(Blake3SolCalls::UpdateHasher(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for Blake3SolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Blake3SolCalls::Finalize(element) => element.encode(),
                Blake3SolCalls::NewDeriveKey(element) => element.encode(),
                Blake3SolCalls::NewHasher(element) => element.encode(),
                Blake3SolCalls::NewKeyed(element) => element.encode(),
                Blake3SolCalls::UpdateHasher(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for Blake3SolCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Blake3SolCalls::Finalize(element) => element.fmt(f),
                Blake3SolCalls::NewDeriveKey(element) => element.fmt(f),
                Blake3SolCalls::NewHasher(element) => element.fmt(f),
                Blake3SolCalls::NewKeyed(element) => element.fmt(f),
                Blake3SolCalls::UpdateHasher(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<FinalizeCall> for Blake3SolCalls {
        fn from(var: FinalizeCall) -> Self {
            Blake3SolCalls::Finalize(var)
        }
    }
    impl ::std::convert::From<NewDeriveKeyCall> for Blake3SolCalls {
        fn from(var: NewDeriveKeyCall) -> Self {
            Blake3SolCalls::NewDeriveKey(var)
        }
    }
    impl ::std::convert::From<NewHasherCall> for Blake3SolCalls {
        fn from(var: NewHasherCall) -> Self {
            Blake3SolCalls::NewHasher(var)
        }
    }
    impl ::std::convert::From<NewKeyedCall> for Blake3SolCalls {
        fn from(var: NewKeyedCall) -> Self {
            Blake3SolCalls::NewKeyed(var)
        }
    }
    impl ::std::convert::From<UpdateHasherCall> for Blake3SolCalls {
        fn from(var: UpdateHasherCall) -> Self {
            Blake3SolCalls::UpdateHasher(var)
        }
    }
    #[doc = "Container type for all return fields from the `finalize` function with signature `finalize(((uint32[8],uint64,bytes,uint256,uint256,uint32),uint32[8],uint32[8][54],uint8,uint32))` and selector `[183, 135, 231, 148]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FinalizeReturn(pub ethers::core::types::Bytes);
    #[doc = "Container type for all return fields from the `new_derive_key` function with signature `new_derive_key(bytes)` and selector `[18, 166, 59, 93]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct NewDeriveKeyReturn(pub Hasher);
    #[doc = "Container type for all return fields from the `new_hasher` function with signature `new_hasher()` and selector `[13, 5, 109, 155]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct NewHasherReturn(pub Hasher);
    #[doc = "Container type for all return fields from the `new_keyed` function with signature `new_keyed(bytes)` and selector `[197, 29, 44, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct NewKeyedReturn(pub Hasher);
    #[doc = "Container type for all return fields from the `update_hasher` function with signature `update_hasher(((uint32[8],uint64,bytes,uint256,uint256,uint32),uint32[8],uint32[8][54],uint8,uint32),bytes)` and selector `[178, 3, 47, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct UpdateHasherReturn(pub Hasher);
    #[doc = "`ChunkState(uint32[8],uint64,bytes,uint256,uint256,uint32)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ChunkState {
        pub chaining_value: [u32; 8],
        pub chunk_counter: u64,
        pub block_bytes: ethers::core::types::Bytes,
        pub block_len: ethers::core::types::U256,
        pub blocks_compressed: ethers::core::types::U256,
        pub flags: u32,
    }
    #[doc = "`Hasher((uint32[8],uint64,bytes,uint256,uint256,uint32),uint32[8],uint32[8][54],uint8,uint32)`"]
    #[derive(
        Clone,
        Debug,
        //Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Hasher {
        pub chunk_state: ChunkState,
        pub key_words: [u32; 8],
        pub cv_stack: [[u32; 8]; 54],
        pub cv_stack_len: u8,
        pub flags: u32,
    }

    impl Default for Hasher {
        fn default() -> Hasher {
            Hasher {
                chunk_state: Default::default(),
                key_words: Default::default(),
                cv_stack: [[0; 8]; 54],
                cv_stack_len: Default::default(),
                flags: Default::default(),
            }
        }
    }
}
