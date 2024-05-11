fn main() {
    let my_u8: u8 = 0b0001_0010;
    let my_i8 = 0b1000_0111_u8 as i8;
    let neg_23 = 0b11101001_u8 as i8;

    println!("{}", my_u8);
    println!("{}", my_i8);
    println!("{}", neg_23);

    let a: u8 = 0b00000010;
    let b: u8 = 0b00000011;

    println!("a: {}", a);
    println!("b: {}", b);

    // AND / & operator returns a 1 in each bit position for which the corresponding bits of each operands are 1s
    println!("a & b: {}", a & b);

    // OR / | operator returns a 1 in in each bit position for which the corresponding bits of either operand is equal or both operands are equal to 1
    println!("a | b: {}", a | b);

    // XOR / ^ (exclusive or) operator returns a 1 in in each bit position for which the corresponding bits of either operand is equal to 1 but NOT both operands
    println!("a ^ b: {}", a ^ b);

    // Not / ! reverses all bits in an operand
    println!("!b: {:#b} / {}", !b, !b);

    // left shift / << moves all bits to the left one space. Shifting a value by one space multiplies it by 2.
    println!("a << b: {}", a << b);

    // binary right shift / >> shifts all bits to the right
    println!("a >> 1: {} / {:#b}", a >> 1, a >> 1);

    // right shift with zero / >>> shifts all bits to the right except that bits shifted to the left are always zero
    println!("a >> 1: {} / {:#b}", a >> 1, a >> 1);

    let data = vec![1, 3];

    let total = data.into_iter().fold(0, |a, b| a ^ b);
    println!("{}", total);

    // pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    //     let total = nums.into_iter().reduce(|a, b| a ^ b);
    //     total.unwrap()
    // }
}
