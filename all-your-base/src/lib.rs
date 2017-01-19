///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits. However, your function must be able to
///     process input with leading 0 digits.
///
#[allow(unused_variables)]
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, ()> {
    if from_base ==1 || to_base == 1 || from_base ==0 || to_base == 0{
        return Err(())
    }
    let mut res: Vec<u32> = Vec::new();
    let mut x:u32 = 0;
    for (i,j) in number.iter().rev().zip(0..number.len()){
        if *i >= from_base{
            return Err(());
        }
        x += i * from_base.pow(j as u32) as u32;
    }
    println!("{}",x);
    while x>0 {
        res.push(x % to_base);
        x /= to_base;
    }
    (*res).reverse();
    Ok(res)
}
