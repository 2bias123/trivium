use cute::c;

fn main() {
    let mut key:Vec<u8> = c![i % 2, for i in 1..94];
    //print!("{:?}",key);
    
}

fn lfsrOutput(state: Vec<u8>,feedbackline1: usize,and1: usize, and2: usize) -> u8{
    let anded = state[and1] & state[and2];
    let output = state[feedbackline1] ^ anded ^ state[state.len()];
    output
}

fn lfsrInput(state: &mut Vec<u8>, puttin: u8, feedbackline2: usize) -> Vec<u8>{
    let new_first = state[feedbackline2] ^ puttin;
    state.insert(0,new_first);
    state.pop();
    state.to_vec()
}