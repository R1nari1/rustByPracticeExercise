fn main() {
    const WIDTH: usize = 30;
    const HEIGHT: usize = 14;

    let mut result = String::new();

    result.push_str(&"*".repeat(WIDTH));
    result.push('\n');

    for i in 1..HEIGHT-1 {
        result.push('*');
        for j in 1..WIDTH-1 {
            if j == i || j == WIDTH - i - 1 {
                result.push('*');
            } else {
                result.push(' ');
            }
        }
        result.push('*');
        result.push('\n');
    }
    result.push_str(&"*".repeat(WIDTH));
    result.push('\n');

    print!("{}", result);
}
