fn main() {
    const N: usize = 6;

    let mut result = String::new();


    for i in 0..N {
        let spaces = N - i - 1;
        let stars = 2 * i + 1;

        result.push_str(&" ".repeat(spaces));
        result.push_str(&"*".repeat(stars));
        result.push('\n');
    }


    for i in (0..N-1).rev() {
        let spaces = N - i - 1;
        let stars = 2 * i + 1;

        result.push_str(&" ".repeat(spaces));
        result.push_str(&"*".repeat(stars));
        result.push('\n');
    }

    print!("{}", result);
}
