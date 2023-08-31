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

// fn main() {
//     let mut  i = vec![1,5,7,2,4,3];

//     let mut  s = vec!['b','a','d','e','c','f'];

//     let mut strs = vec!["s","a","b","e","c","f"];

//     t_sort(&mut i);

//     t_sort(&mut s);

//     t_sort(&mut strs);


//     println!("{:?}",i);


//     println!("{:?}",s);

//     println!("{:?}",strs);

// }



// fn t_sort<T: PartialOrd + Copy>(list: &mut Vec<T>) -> &Vec<T> {
//     for i in 0..list.len() {
//         for x in 0..list.len() - 1 {
//             if list[x] > list[x + 1] {
//                 list.swap(x, x + 1); 
//             }
//         }
//     }
//     list
// }


//第一题
// trait Signal{
//     type item;

//     fn continued(&self) -> Self::item;
// }

// enum SignalType {
//     RedSignal,
//     GreenSignal
// }

// impl Signal for SignalType{
//     type item = u64;

//     fn continued(&self) -> Self::item{

//         match self {
//             SignalType::GreenSignal => {
//                 10
//             }
//             SignalType::RedSignal => {
//                 5
//             }
//         }
//     }
// }

// fn main(){

//     let rs = SignalType::RedSignal;
//     let rg = SignalType::GreenSignal;

//     println!("红灯持续时间为{}分钟",rs.continued());

//     println!("绿灯持续时间为{}分钟",rg.continued());
// }

//第二题
// fn main() {
    
//     let ss1 = vec![1,2,3,4,4294967295];

//     let ss2 = vec![1,2,3,4,5];

//     println!("溢出sum = {:?}",sum(ss1.as_slice()));
//     println!("sum = {:?}",sum(ss2.as_slice()));
// }

// fn sum(is:&[u32]) -> Option<u32>{

//     let mut ss:u32= 0;

//     let iter = is.into_iter();
//     for i in iter {
//         ss = ss.checked_add(*i)?;
//     }
//     return Some(ss);
// }


//第三题
trait area {

    fn AreaCalculate(&self) -> u64;
}

struct Round {}

impl area for Round {

    fn AreaCalculate(&self) -> u64 {
        10
    }
}

struct Triangle {}

impl area for Triangle {

    fn AreaCalculate(&self) -> u64 {
        27
    }
}

struct Square {}
impl area for Square {

    fn AreaCalculate(&self) -> u64 {
        50
    }
}


fn main() {

    let round = Round{};
    
    Calculate(&round);
}

fn Calculate(a:&impl area) -> u64{

    a.AreaCalculate()
}