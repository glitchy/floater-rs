// Memory layout of the f32 type in Rust (binary32) showing the three distinct
// components encoded within the bits of a floating-point number for the f32 type.
// '''
// 
// byte 0           byte 1           byte 2           byte 3
// [][][][][][][][] [][][][][][][][] [][][][][][][][] [][][][][][][][]
// |||               ||                                              |
// | |               ||                                              |
// | |               ||                                              |
// | |_______________||______________________________________________|
// |         |                                 |
// |__,   exponent                          mantissa
//    |
// sign bit
// '''

const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn main() {
    let n: f32 = 87.87;
    let (sign, exp, frac) = to_parts(n);
    let (sign_, exp_, mant) = decode(sign, exp, frac);
    let n_ = from_parts(sign_, exp_, mant);

    println!("original: {} -> final: {}", n, n_);
    assert_eq!(n, n_);
    println!("field    |                 as bits | as real number");
    println!("sign     |                       {:01b} | {}", sign, sign_);
    println!("exponent |                {:08b} | {}", exp, exp_);
    println!("mantissa | {:023b} | {}", frac, mant);
}

/// Extract the bits of individual parts within floating point 
/// container.
fn to_parts(n: f32) -> (u32, u32, u32) {
    // Cast f32 as u32 to allow for bit manipulation.
    let bits: u32 = n.to_bits();

    // Shift bits 31 places to the right, putting the sign 
    // bit in the least significant position and filter extra
    // bits away with AND mask.
    let sign = (bits >> 31) & 1;
    // Shift exponent's 8 bits to the right, overwriting the 
    // mantissa and filter sign bit away with AND mask.
    let exp = (bits >> 23) & 0xff;
    let mant = bits & 0x7fffff;

    (sign, exp, mant)
}

/// Decode each value from its raw bit pattern to its actual value.
fn decode(
    sign: u32,
    exp: u32,
    frac: u32
) -> (f32, f32, f32) {
    // Converts the sign bit to 1.0 or -1.0. Parenthesis
    // are required around -1.0_f32 to clarify operator 
    // precedence due to method calls ranking higher than 
    // a unary minus.
    let signed_1 = (-1.0_f32).powf(sign as f32);
    // Exponent must first become an i32 in case subtracting 
    // the BIAS results in a negative number; then it needs 
    // to be cast as a f32 for exponentiation.
    let exp = (exp as i32) - BIAS;
    let exp = RADIX.powf(exp as f32);
    let mut mant: f32 = 1.0;

    // Iterate through fractional bits of the mantissa, adding 
    // those bit's defined values to the mantissa variable.
    for i in 0..23 {
        // Creates a bit mask with the iteration number as then
        // bit allowed to pass through.
        let mask = 1 << i;
        let one_at_bit_i = frac & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mant += weight;
        }
    }

    (signed_1, exp, mant)
}

/// Perform the arithmetic to convert from scientific notation 
/// to an ordinary number.
fn from_parts(
    sign: f32,
    exp: f32,
    mant: f32,
) -> f32 {
    sign * exp * mant
}
