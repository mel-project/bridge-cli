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
            "0x6080806040523461001a5761263b9081610020823930815050f35b600080fdfe60406104c0815260048036101561001557600080fd5b600090813560e01c9081630d056d9b146103a857816312a63b5d146102175781635163dae5146101835781639301d7021461011b575063c51d2cd71461005a57600080fd5b610117916100979161006b36610597565b61007694919461095b565b50610092610082610659565b9586926101003685373691610752565b610a7b565b61009f61095b565b506100a8610928565b6100b06108ea565b506100b9610699565b93838552833660208701376100cc6105e5565b9481865283602087015284860152826060860152826080860152601060a08601526100f561061a565b94855260208501528284015260608301526010608083015251918291826104bc565b0390f35b8380925060031936011261017f576001600160401b03813581811161017b5761014790369084016107a1565b9360243591821161017857509261016761016d9261011795369101610783565b90610c58565b9051918291826104bc565b80fd5b8480fd5b8280fd5b905082602091600319838136011261017b578135906001600160401b03908183116102135761376090833603011261020f57835195868501918211878310176101fc57506101e884936101ed93889361011797528784528736818601373691016107a1565b612271565b5192828493845283019061047c565b634e487b7160e01b815260418452602490fd5b8580fd5b8680fd5b828461022236610597565b61022d93919361095b565b506102c06102ba61023c61099e565b9561024561095b565b5061024e610928565b936102576108ea565b50610260610699565b9487865260209888368b8901376102756105e5565b96818852888b890152898801528760608801528760808801528960a088015261029c61061a565b96875289870152878601528560608601528760808601523691610752565b82610c58565b5082516001600160401b03916101208201838111838210176103955785526102f4826101009283825283368a840137612271565b84519281840190811184821017610395578552610117965061031a918391368337610a7b565b61032261095b565b5061032b610928565b906103346108ea565b5061033d610699565b9484865284368288013761034f6105e5565b958287528482880152858701528360608701528360808701528460a087015261037661061a565b95865285015282840152606083015280608083015251918291826104bc565b634e487b7160e01b855260418852602485fd5b82848160031936011261044a5790610117916103c261095b565b506103cb61099e565b916103d461095b565b506103dd610928565b6103e56108ea565b506103ee610699565b93838552833660208701376104016105e5565b94818652836020870152848601528260608601528260808601528260a086015261042961061a565b948552602085015282840152806060840152608083015251918291826104bc565b5080fd5b6000915b6008831061045f57505050565b60019063ffffffff83511681526020809101920192019190610452565b919082519283825260005b8481106104a8575050826000602080949584010152601f8019910116010190565b602081830181015184830182015201610487565b602080825260a08351936137609283818601526104de6137808601875161044e565b6001600160401b03818701511661388086015261050d60408701516101a06138a088015261392087019061047c565b9560608101516138c087015260808101516138e087015263ffffffff93849101511661390086015261054681830151604087019061044e565b604082015190600061014087015b6036821061057957505050606082015160ff1661374086015250608001511691015290565b826101008261058b600194885161044e565b01940191019092610554565b9060206003198301126105e0576004356001600160401b03928382116105e057806023830112156105e05781600401359384116105e057602484830101116105e0576024019190565b600080fd5b6040519060c082018281106001600160401b0382111761060457604052565b634e487b7160e01b600052604160045260246000fd5b6040519060a082018281106001600160401b0382111761060457604052565b6040519061020082018281106001600160401b0382111761060457604052565b6040519061010082018281106001600160401b0382111761060457604052565b604051906106c082018281106001600160401b0382111761060457604052565b60405190606082018281106001600160401b0382111761060457604052565b6040519190601f01601f191682016001600160401b0381118382101761060457604052565b359063ffffffff821682036105e057565b9080601f830112156105e057610702610659565b809261010081019283116105e057905b82821061071f5750505090565b6020809161072c846106dd565b815201910190610712565b6001600160401b03811161060457601f01601f191660200190565b92919261076661076183610737565b6106b8565b93828552828201116105e057816000926020928387013784010152565b9080601f830112156105e05781602061079e93359101610752565b90565b919091613760818403126105e0576040519060a08201936001600160401b03908386108287111761060457839583358381116105e0578401906101a0828403126105e057610160808701828110868211176106045760405261080384846106ee565b8252610100928381013586811681036105e05760c089015261012095868201359081116105e0578161083d876101809361085b9501610783565b60e08b01526101409384820135878c0152810135888b0152016106dd565b908701528552602091610870818487016106ee565b838701528061013f860112156105e057610888610679565b92836137208701958387116105e0578701905b8682106108d25750505050506040840152359060ff821682036105e0576137406108cd916080936060860152016106dd565b910152565b8285916108df86856106ee565b81520191019061089b565b6108f26105e5565b906108fb610659565b610100368237825260006020830152606060408301526000606083015260006080830152600060a0830152565b610930610679565b9060005b6106c081106109405750565b60209061094b610659565b6101003682378185015201610934565b61096361061a565b9061096c6108ea565b8252610976610659565b6101003682376020830152610989610928565b60408301526000606083015260006080830152565b6109a6610659565b61010080913690376040519081018181106001600160401b0382111761060457604052636a09e667815263bb67ae856020820152633c6ef372604082015263a54ff53a606082015263510e527f6080820152639b05688c60a0820152631f83d9ab60c0820152635be0cd1960e082015290565b6000198114610a285760010190565b634e487b7160e01b600052601160045260246000fd5b806000190460041181151516610a285760021b90565b906008811015610a655760051b0190565b634e487b7160e01b600052603260045260246000fd5b91906020835111610ace5760005b835160021c811015610ac85780610aab610aa5610ac393610a3e565b86610b5a565b63ffffffff610aba8386610a54565b91169052610a19565b610a89565b50509050565b60405162461bcd60e51b815260206004820152603360248201527f4461746120627974657320697320746f6f206c6f6e6720746f20636f6e7665726044820152727420746f203820342d6279746520776f72647360681b6064820152608490fd5b91908201809211610a2857565b91908203918211610a2857565b908151811015610a65570160200190565b805160049081840190818511610bea5710610c1457919060009283925b818410610b85575050505090565b909192936003858103818111610bff57610bad610ba78560ff60f81b93610b2f565b87610b49565b51166001600160fd1b038711600116610bff5763ffffffff9283169187901b1c60e01c01908111610bea57610be29094610a19565b929190610b77565b601183634e487b7160e01b6000525260246000fd5b601185634e487b7160e01b6000525260246000fd5b60649060206040519162461bcd60e51b8352820152601f60248201527f6c655f62797465735f6765745f75696e7433325f6f75744f66426f756e6473006044820152fd5b9091610c6261095b565b506000905b835182101561121f5761040080610c7e8551611fdc565b146110d9575b610c8e8451611fdc565b8103908111610a2857610cb29063ffffffff610cab858851610b3c565b16906120cc565b90610cc683610cc18185610b2f565b610b3c565b94610cd361076187610737565b86815295601f19610ce382610737565b0136602089013760005b8181106110a557505084519460005b875181101561108f576040606088015114610db4575b606087015160400360408111610a2857610d3790610d31838b51610b3c565b906120cc565b9060005b828110610d65575090610d6091610d568260608b0151610b2f565b60608a0152610b2f565b610cfc565b610daf9089610da96001600160f81b0319610d8a8e610d848689610b2f565b90610b49565b5116610d9f6060604085015194015185610b2f565b9060001a92610b49565b53610a19565b610d3b565b9490610dc4979492969397610639565b97610200368a37610dd9896040870151612003565b845197602086015163ffffffff60a088015116610df5886120ba565b61020036610e01610639565b37610e0a61099e565b92610e13610639565b9361020036863760005b6010811061106457508d9e50610e349d9c9d610639565b8d819e5163ffffffff168252602081015163ffffffff166020830152604081015163ffffffff166040830152606081015163ffffffff166060830152608081015163ffffffff16608083015260a081015163ffffffff1660a083015260c08082015163ffffffff169083015260e08091015163ffffffff169101528c815163ffffffff169061010001528c602082015163ffffffff169061012001528c604082015163ffffffff169061014001526060015163ffffffff166101608d015263ffffffff81166101808d015260201c63ffffffff166101a08c01526101c08b016040905263ffffffff16176101e08a0152610f2e818a611475565b610f3781611df3565b610f41818a611475565b610f4a81611df3565b610f54818a611475565b610f5d81611df3565b610f67818a611475565b610f7081611df3565b610f7a818a611475565b610f8381611df3565b610f8d818a611475565b610f9681611df3565b610fa09089611475565b60005b6008811015611017576008810190818111610a28578163ffffffff610aba8c8f611008858561100089829583610fdc6110129e8a611464565b511684610fe9848b611464565b51161884610ff7848b611464565b91169052610a54565b511693611464565b511618928d611464565b610fa3565b5093959850939690919561102a90611f8b565b87526080870180519060018201809211610a285752611047610699565b604081526040366020830137604088015260006060880152610d12565b8f9063ffffffff6110788261108a94611464565b51166110848289611464565b52610a19565b610e1d565b5094509450919061109f91610b2f565b90610c67565b6110d4906001600160f81b03196110c56110bf838a610b2f565b86610b49565b511660001a610da9828b610b49565b610ced565b6110eb6110e68551612121565b611225565b60016001600160401b03906020928183858a510151160190838211610a285791815b818116156111aa5750604090818a015192606091611140838d019660ff96878951169161113a8383612260565b52612260565b50848651160192848411610a28576000958c958795169052878501519763ffffffff60808097015116976111726108ea565b5061117b610699565b9383855283368487013761118d6105e5565b9a8b52169089015287015285015283015260a08201528452610c84565b9261010080366111b8610659565b3760608b0160ff91600019838351160192808411610a2857677fffffffffffffff946111f68f92946110e69561121697168091526040840151612260565b5163ffffffff60808d850151940151169336611210610659565b3761219c565b93821c1661110d565b92505090565b9061122e610659565b61010080913690376080835193602081015190604081015190606081015163ffffffff8095819301511692610200948536611267610639565b3761127061099e565b90611279610639565b9636883760005b6010811061143b57505090838092611296610639565b99828d818151168d52602001511660208c0152828d8c826040830151169060400152606001511660608c0152828d8c82608083015116906080015260a001511660a08c0152828d8c8260c0830151169060c0015260e001511660e08c015282825116908b0152816020820151166101208b0152816040820151166101408b0152606001511661016089015281811661018089015260201c166101a0870152166101c08501526101e084015261134b8184611475565b61135481611df3565b61135e8184611475565b61136781611df3565b6113718184611475565b61137a81611df3565b6113848184611475565b61138d81611df3565b6113978184611475565b6113a081611df3565b6113aa8184611475565b6113b381611df3565b6113bd9083611475565b60005b60088082101561142d57810190818111610a285781836113e36114289487611464565b5116846113f08488611464565b511618846113fe8488611464565b9116905283610aba81611411858b610a54565b51168261141e858a611464565b5116189287611464565b6113c0565b50505061079e919250611f8b565b819293949561144d8261145994611464565b5116611084828a611464565b908794939291611280565b906010811015610a655760051b0190565b90816102e0528060c052805163ffffffff169081610320526020015163ffffffff166101a052815163ffffffff1680608052608083015163ffffffff1690816104a0520163ffffffff160163ffffffff169052610320516104a05190816080510163ffffffff160163ffffffff166102e051908061018083015163ffffffff16186114ff90611eff565b63ffffffff16928361010084015163ffffffff160163ffffffff169081610220521861152a90611f22565b63ffffffff1680610420526101a051910163ffffffff160163ffffffff1690526101a05161042051610320516104a0516080510163ffffffff160163ffffffff160163ffffffff160163ffffffff161861158390611f46565b63ffffffff168060a0526102e051610180015260a051610220510163ffffffff166102e051610100015260a051610220510163ffffffff1661042051186115c990611f6a565b63ffffffff168060e0526102e0516080015260c051604081015163ffffffff169081610300526060015163ffffffff16806101c0526102e05190602082015163ffffffff16806102a05260a083015163ffffffff1693846102c0528085830163ffffffff160163ffffffff166101a085015163ffffffff161861164b90611eff565b63ffffffff16948561012086015163ffffffff160163ffffffff168061024052811861167690611f22565b63ffffffff169283610460520163ffffffff160163ffffffff160163ffffffff160163ffffffff1690602001526101c05161046051610300516102c0516102a0510163ffffffff160163ffffffff160163ffffffff160163ffffffff16186116dd90611f46565b63ffffffff1680610200526102e0516101a0015261020051610240510163ffffffff166102e051610120015261020051610240510163ffffffff16610460511861172690611f6a565b63ffffffff1680610120526102e05160a0015260c051608081015163ffffffff1690816103405260a0015163ffffffff16806101e0526102e05190604082015163ffffffff16806104805260c083015163ffffffff1693846103e0528085830163ffffffff160163ffffffff166101c085015163ffffffff16186117a990611eff565b63ffffffff16948561014086015163ffffffff160163ffffffff16806103a05281186117d490611f22565b63ffffffff1692836103c0520163ffffffff160163ffffffff160163ffffffff160163ffffffff1690604001526101e0516103c051610340516103e051610480510163ffffffff160163ffffffff160163ffffffff160163ffffffff161861183b90611f46565b63ffffffff1680610380526102e0516101c00152610380516103a0510163ffffffff166102e0516101400152610380516103a0510163ffffffff166103c0511861188490611f6a565b63ffffffff1680610140526102e05160c0015260c05160c081015163ffffffff1690816103605260e0015163ffffffff1680610180526102e05190606082015163ffffffff16806102805260e083015163ffffffff169384610160528085830163ffffffff160163ffffffff166101e085015163ffffffff161861190790611eff565b63ffffffff16948561016086015163ffffffff160163ffffffff168061044052811861193290611f22565b63ffffffff169283610400520163ffffffff160163ffffffff160163ffffffff160163ffffffff16906060015261018051610400516103605161016051610280510163ffffffff160163ffffffff160163ffffffff160163ffffffff161861199990611f46565b63ffffffff1680610260526102e0516101e0015261026051610440510163ffffffff166102e051610160015261026051610440510163ffffffff1661040051186119e290611f6a565b63ffffffff1680610100526102e05160e0015260c05161010081015163ffffffff1690610120015163ffffffff1690610120516101a05161042051610320516104a0516080510163ffffffff160163ffffffff160163ffffffff160163ffffffff160163ffffffff160163ffffffff1690816102e05152816102605118611a6890611eff565b63ffffffff1680610380516103a0510163ffffffff160163ffffffff1691826101205118611a9590611f22565b63ffffffff1680940163ffffffff160163ffffffff1690816102e0515218611abc90611f46565b63ffffffff1690816102e0516101e001520163ffffffff1690816102e051610140015218611ae990611f6a565b63ffffffff166102e05160a0015260c05161014081015163ffffffff1690610160015163ffffffff16906101405190816101c05161046051610300516102c0516102a0510163ffffffff160163ffffffff160163ffffffff160163ffffffff160163ffffffff160163ffffffff16918260a05118611b6690611eff565b63ffffffff16908161026051610440510163ffffffff160163ffffffff16809318611b9090611f22565b63ffffffff1680940163ffffffff160163ffffffff1690816102e0516020015218611bba90611f46565b63ffffffff1690816102e05161018001520163ffffffff1690816102e051610160015218611be790611f6a565b63ffffffff166102e05160c0015260c05161018081015163ffffffff16906101a0015163ffffffff16906101005190816101e0516103c051610340516103e051610480510163ffffffff160163ffffffff160163ffffffff160163ffffffff160163ffffffff160163ffffffff1691826102005118611c6590611eff565b63ffffffff16908160a051610220510163ffffffff160163ffffffff16809318611c8e90611f22565b63ffffffff1680940163ffffffff160163ffffffff1690816102e0516040015218611cb890611f46565b63ffffffff1690816102e0516101a001520163ffffffff1690816102e051610100015218611ce590611f6a565b63ffffffff166102e05160e0015260c0516101c081015163ffffffff16906101e0015163ffffffff169060e051908161018051610400516103605161016051610280510163ffffffff160163ffffffff160163ffffffff160163ffffffff160163ffffffff160163ffffffff1691826103805118611d6290611eff565b63ffffffff16908161020051610240510163ffffffff160163ffffffff16809318611d8c90611f22565b63ffffffff1680940163ffffffff160163ffffffff1690816102e0516060015218611db690611f46565b63ffffffff1690816102e0516101c001520163ffffffff1690816102e051610120015218611de390611f6a565b63ffffffff166102e05160800152565b611dfb610639565b906102008092369037611e0c610639565b600281526006602082015260036040820152600a606082015260076080820152600060a0820152600460c0820152600d60e08201526001610100820152600b610120820152600c61014082015260056101608201526009610180820152600e6101a0820152600f6101c082015260086101e0820152611e89610639565b9236843760005b60108110611ece57505060005b60108110611eaa57505050565b8063ffffffff611ebd611ec99386611464565b51166110848285611464565b611e9d565b8063ffffffff611eee60ff611ee6611efa9587611464565b511686611464565b51166110848287611464565b611e90565b60d081901b61ffff60e01b1660f09190911b6001600160f01b0319161760e01c90565b60d481901b620fffff60e01b1660f49190911b6001600160f41b0319161760e01c90565b60d881901b62ffffff60e01b1660f89190911b6001600160f81b0319161760e01c90565b60d981901b6301ffffff60e01b1660f99190911b607f60f91b161760e01c90565b90611f94610659565b6101008091369037611fa4610659565b9036823760005b60088110611fb857509150565b8063ffffffff611fcb611fd79387611464565b51166110848285610a54565b611fab565b60808101516001600160fa1b038111600116610a2857606061079e9201519060061b610b2f565b91908251604081111590816120ae575b501561204c5760005b835160021c811015610ac85780612038610aa561204793610a3e565b63ffffffff610aba8386611464565b61201c565b60405162461bcd60e51b815260206004820152603460248201527f4461746120627974657320697320746f6f206c6f6e6720746f20636f6e7665726044820152737420746f20313620342d6279746520776f72647360601b6064820152608490fd5b60039150161538612013565b608001516120c757600190565b600090565b90808210156120d9575090565b905090565b6120e661061a565b906120ef610659565b61010036823782526120ff610639565b6102003682376020830152600060408301526000606083015260006080830152565b6121296120de565b50612132610639565b90610200368337612147826040830151612003565b8051916001600160401b03602083015116606083015190600263ffffffff946121768660a083015116916120ba565b17179261218161061a565b95865260208601526040850152606084015216608082015290565b909291926121a86120de565b506121b1610639565b90610200368337600092835b6008811061224857505060085b601081106122045750509063ffffffff92916121e461061a565b948552602085015260408401526040606084015260041716608082015290565b6007198101818111612234579063ffffffff61222361222f9385610a54565b51166110848286611464565b6121ca565b634e487b7160e01b85526011600452602485fd5b8063ffffffff611eee61225b9385610a54565b6121bd565b906036811015610a655760051b0190565b91909161227e8151612121565b9260ff806060840151165b63ffffffff80911680156122f05760001901958187116122db576122d5916122c36122bc898360408a0151911690612260565b5192611225565b6020870151916080880151169261219c565b94612289565b60246000634e487b7160e01b81526011600452fd5b50509391509160809282519460208401519560608501519463ffffffff9687910151169661231c610639565b61020080913690378761232d61099e565b97612336610639565b9236843760005b601081106125ea57506124769350816060612356610639565b9a828851168c528260208901511660208d01528260408901511660408d0152828289015116828d01528260808901511660808d01528260a08901511660a08d01528260c08901511660c08d01528260e08901511660e08d0152828151166101008d0152826020820151166101208d0152826040820151166101408d01520151166101608a015260006101808a015260006101a08a0152166101c088015260088099176101e08801526124088188611475565b61241181611df3565b61241b8188611475565b61242481611df3565b61242e8188611475565b61243781611df3565b6124418188611475565b61244a81611df3565b6124548188611475565b61245d81611df3565b6124678188611475565b61247081611df3565b86611475565b60005b87811061258657505060005b8581168781101561257c578661249b8288611464565b511690600490808904821190151516612567579087929160009063fffffffc8460021b16915b8181106124dc575050505016858114610a2857600101612485565b80919293949550600019048b1181151516612552578060031b868111610bea576001811b1561253d579061253291610da96125178387610b2f565b9187901c60f81b6001600160f81b03191660001a918a610b49565b9089949392916124c1565b601283634e487b7160e01b6000525260246000fd5b601182634e487b7160e01b6000525260246000fd5b601190634e487b7160e01b6000525260246000fd5b5050505092505050565b87810190818111610a285781886125a06125e5948a611464565b5116896125ad848b611464565b511618896125bb848b611464565b9116905288610aba816125ce8588610a54565b5116826125db858d611464565b511618928a611464565b612479565b8492611eee826125fb949697611464565b918991939261233d56fea2646970667358221220ee21d034fffcda285209b675ff89ed6d103a781435bb51279d14aa42418d2eb764736f6c63430008100033" . parse () . expect ("invalid bytecode")
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
        // Default,
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
