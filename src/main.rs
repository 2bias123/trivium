use cute::c;
use crate::initial_state_generation::*;
use crate::lfsr::lfsr;

mod initial_state_generation;
mod lfsr;

fn main() {
    let mut key:Vec<u8> = c![i % 2, for i in 1..81];
    let mut iv:Vec<u8> = c![i % 2, for i in 0..80];

    let mut a:Vec<u8> = init84(&mut key);
    let mut b:Vec<u8> = init93(&mut iv);
    let mut c:Vec<u8> = init111();

    let stream:Vec<u8> = lfsr(2152, &mut b, &mut a, &mut c,&mut Vec::<u8>::new());

    // let plaintext:Vec<u8> =vec![1;10];

    // let mut chipertext:Vec<u8> = Vec::new();

    // let mut new_plaintext:Vec<u8> = Vec::new();

    // for i in 0..plaintext.len(){
    //     chipertext.push(plaintext[i] ^ stream[i]);
    // }

    // for i in 0..chipertext.len(){
    //     new_plaintext.push(chipertext[i] ^ stream[i]);
    // }
    
    println!("{:?}",stream);

}