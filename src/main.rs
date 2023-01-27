
fn main() {

}

fn lfsr(feedbackline1: usize, feedbackline2: usize, and1: usize, and2: usize, firstbit: u8, state: &mut Vec<u8>) -> u8{
    //let state = initial_state;

    let frontcell = state.get(feedbackline1).unwrap();
    let backcell = state.get(feedbackline2).unwrap();

    let firstand = state.get(and1).unwrap();
    let secondand = state.get(and2).unwrap();

    let anded = firstand * secondand;

    let lastbit = state.last().unwrap();

    let xored = frontcell ^ anded ^ lastbit;

    let new_first = firstbit ^ backcell;

    state.pop();

    state.insert(0,new_first);

    xored
}

