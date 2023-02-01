use cute::c;

fn main() {
    let mut key:Vec<u8> = c![i % 2, for i in 1..81];
    let mut iv:Vec<u8> = c![i % 2, for i in 0..80];


    let mut a = init84(&mut key);
    let mut b = init93(&mut iv);
    let mut c = init111();

    let stream = lsfr(2152, &mut b, &mut a, &mut c,&mut Vec::<u8>::new());

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

/// Computes the output of a linear feedback shift register.
///
/// # Arguments
/// * `state` - A vector representing the current state of the register.
/// * `feedbackline1` - An index indicating the position of the first feedback line.
/// * `and1` - An index indicating the first operand for the AND operation.
/// * `and2` - An index indicating the second operand for the AND operation.
///
/// # Returns
/// The output of the register, which is the XOR of the state at the position of the feedback line, the result of the AND operation, and the state at the last position of the register.
///
/// # Example
/// ```
/// let state = vec![1,0,1,0];
/// let feedbackline1 = 0;
/// let and1 = 1;
/// let and2 = 2;
/// assert_eq!(lfsr_output(&state, feedbackline1, and1, and2), 0);
/// ```
fn lfsr_output(state: &Vec<u8>,feedbackline1: usize,and1: usize, and2: usize) -> u8{
    let anded = state[and1] & state[and2];
    let output = state[feedbackline1] ^ anded ^ state[state.len()-1];
    output
}


/// Shift the LFSR state to the right and insert a new value at the beginning.
///
/// # Arguments
///
/// * `state` - A mutable reference to a vector of unsigned 8-bit integers representing the current state of the LFSR.
/// * `puttin` - The value to be inserted at the beginning of the state.
/// * `feedbackline2` - An index indicating the position of the second feedback line.
///
/// # Returns
///
/// A new vector of unsigned 8-bit integers representing the updated state of the LFSR.
///
/// # Example
///
/// ```
/// let mut state = vec![1, 0, 1];
/// let puttin = 1;
/// let feedbackline2 = 1;
/// let updated_state = lfsr_input(&mut state, puttin, feedbackline2);
/// assert_eq!(updated_state, vec![1, 1, 0]);
/// ```
fn lfsr_input(state: &mut Vec<u8>, puttin: u8, feedbackline2: usize) -> Vec<u8>{
    let new_first = state[feedbackline2] ^ puttin;
    state.insert(0,new_first);
    state.pop();
    state.to_vec()
}


/// Simulate the operation of three LFSRs and generate a stream of outputs.
///
/// # Arguments
///
/// * `clockings` - The number of clock cycles to run the simulation for.
/// * `state93` - A mutable reference to a vector of unsigned 8-bit integers representing the initial state of the first LFSR.
/// * `state84` - A mutable reference to a vector of unsigned 8-bit integers representing the initial state of the second LFSR.
/// * `state111` - A mutable reference to a vector of unsigned 8-bit integers representing the initial state of the third LFSR.
/// * `putout` - A mutable reference to a vector of unsigned 8-bit integers that will store the generated outputs.
///
/// # Returns
///
/// A vector of unsigned 8-bit integers representing the generated outputs.
///
/// # Example
///
/// ```
/// let clockings = 5;
/// let mut state93 = vec![1, 0, 1];
/// let mut state84 = vec![1, 0, 1];
/// let mut state111 = vec![1, 0, 1];
/// let mut putout = vec![];
/// let outputs = lsfr(clockings, &mut state93, &mut state84, &mut state111, &mut putout);
/// assert_eq!(outputs, vec![1, 0, 1, 0, 1]);
/// ```
fn lsfr(clockings: u32, state93:&mut Vec<u8>, state84:&mut Vec<u8>, state111:&mut Vec<u8>, putout: &mut Vec<u8>) -> Vec<u8>{
    if clockings > 0 {
        let lsfr_output1 = lfsr_output(state93, 65, 90, 91);
        let lsfr_output2 = lfsr_output(state84, 68, 81, 82);
        let lsfr_output3 = lfsr_output(state111, 65, 108, 109);

        let mut new_state1 = lfsr_input(state93,lsfr_output1 , 68);
        let mut new_state2 = lfsr_input(state84, lsfr_output2, 77);
        let mut new_state3 = lfsr_input(state111, lsfr_output3, 86);

        let output = lsfr_output1 ^ lsfr_output2 ^ lsfr_output3;
        if clockings > 1152 {
            putout.insert(0,output);
        }

        lsfr(clockings-1, &mut new_state1, &mut new_state2, &mut new_state3,putout)
    } else {
        return putout.to_vec();
    }
}