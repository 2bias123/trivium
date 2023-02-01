use cute::c;
use crate::initial_state_generation::*;
use crate::lfsr::lfsr;

mod initial_state_generation;
mod lfsr;

fn main() {
    let mut key:Vec<u8> = c![i % 2, for i in 1..81];
    let mut iv:Vec<u8> = c![i % 2, for i in 0..80];


    let mut a = init84(&mut key);
    let mut b = init93(&mut iv);
    let mut c = init111();

    let stream = lfsr(2152, &mut b, &mut a, &mut c,&mut Vec::<u8>::new());

    println!("{:?}",stream);
}