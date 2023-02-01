use cute::c;

/// Initializes an 84-bit initial state for an lfsr from a given seed value
///
/// # Arguments
///
/// * `key` - The seed value to use for initialization.
///
/// # Returns
///
/// An 84-bit key, padded with zeros if the seed value was shorter than 84 bits.
pub fn init84(key: &mut Vec<u8>) -> Vec<u8>{
    while key.len() < 84 {
        key.push(0);
    }
    key.to_vec()
}

/// Initializes a 93-bit initialization vector for an lfsr from a given seed value
///
/// # Arguments
///
/// * `iv` - The seed value to use for initialization.
///
/// # Returns
///
/// A 93-bit initialization vector, padded with zeros if the seed value was shorter than 93 bits.
pub fn init93(iv: &mut Vec<u8>) -> Vec<u8>{
    while iv.len() < 93 {
        iv.push(0);
    }
    iv.to_vec()
}

/// Initializes a 111-bit seed value for an lfsr using a default pattern
///
/// # Returns
///
/// A 111-bit seed value with 108 zeros followed by three ones.
pub fn init111() -> Vec<u8>{
    let mut init: Vec<u8> = c![0, for _i in 0..108];
    while init.len() < 111 {
        init.push(1);
    }
    init
}