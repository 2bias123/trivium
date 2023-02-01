use cute::c;

fn main() {
    let mut key:Vec<u8> = c![i % 2, for i in 1..81];
    let mut a = init84(&mut key);
    let mut b = init93(&mut key);
    let mut c = init111();

    let stream = lsfr(1152, &mut b, &mut a, &mut c,&mut Vec::<u8>::new());

    println!("{:?}",stream);
        
}

fn init84(key: &mut Vec<u8>) -> Vec<u8>{
    while key.len() < 84 {
        key.push(0);
    }
    key.to_vec()
}

fn init93(key: &mut Vec<u8>) -> Vec<u8>{
    while key.len() < 93 {
        key.push(0);
    }
    key.to_vec()
}

fn init111() -> Vec<u8>{
    let mut init: Vec<u8> = c![0, for _i in 0..108];
    while init.len() < 111 {
        init.push(1);
    }
    init
}

fn lfsr_output(state: &Vec<u8>,feedbackline1: usize,and1: usize, and2: usize) -> u8{
    let anded = state[and1] & state[and2];
    let output = state[feedbackline1] ^ anded ^ state[state.len()-1];
    output
}

fn lfsr_input(state: &mut Vec<u8>, puttin: u8, feedbackline2: usize) -> Vec<u8>{
    let new_first = state[feedbackline2] ^ puttin;
    state.insert(0,new_first);
    state.pop();
    state.to_vec()
}

fn lsfr(clockings: u32, state93:&mut Vec<u8>, state84:&mut Vec<u8>, state111:&mut Vec<u8>, putout: &mut Vec<u8>) -> Vec<u8>{
    if clockings > 0 {
        let lsfr_output1 = lfsr_output(state93, 65, 90, 91);
        let lsfr_output2 = lfsr_output(state84, 68, 81, 82);
        let lsfr_output3 = lfsr_output(state111, 65, 108, 109);

        let mut new_state1 = lfsr_input(state93,lsfr_output1 , 68);
        let mut new_state2 = lfsr_input(state84, lsfr_output2, 77);
        let mut new_state3 = lfsr_input(state111, lsfr_output3, 86);

        let output = lsfr_output1 ^ lsfr_output2 ^ lsfr_output3;

        putout.insert(0,output);

        lsfr(clockings-1, &mut new_state1, &mut new_state2, &mut new_state3,putout)
    } else {
        return putout.to_vec();
    }
}