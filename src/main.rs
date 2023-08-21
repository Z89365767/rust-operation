// use rocket_contrib::json::JsonValue;
// use bip39::{MnemonicType, Mnemonic, Language};
// use sp_core::{
// 	hexdisplay::HexDisplay,
//     crypto::{Ss58Codec, Ss58AddressFormat},
// };

// use sp_runtime::{MultiSigner, traits::IdentifyAccount};

// fn main() {
//     new_address();
// }

// pub fn new_address() {
//     generate::<sp_core::sr25519::Pair>()
// }

// fn generate<Pair>() where Pair: sp_core::Pair, Pair::Public: Into<MultiSigner> {

//     // 生成12个助记词   
//     let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);

//     let phrase = mnemonic.phrase();

//     // 生成地址及助记词
//     match Pair::from_phrase(phrase, None) {
//         Ok((pair, seed)) => {
//             let account = pair.public().into().into_account();
//                // Substrate地址
//                let address = account.to_ss58check();
//                println!("{:?}",address)
//             }
//          Err(err) => println!("{:?}",err)
//     }
// }

fn main() {
    let mut  i = vec![1,5,7,2,4,3];

    let mut  s = vec!['b','a','d','e','c','f'];

    let mut strs = vec!["s","a","b","e","c","f"];

    t_sort(&mut i);

    t_sort(&mut s);

    t_sort(&mut strs);


    println!("{:?}",i);


    println!("{:?}",s);

    println!("{:?}",strs);

}



fn t_sort<T: PartialOrd + Copy>(list: &mut Vec<T>) -> &Vec<T> {
    for i in 0..list.len() {
        for x in 0..list.len() - 1 {
            if list[x] > list[x + 1] {
                list.swap(x, x + 1); 
            }
        }
    }
    list
}