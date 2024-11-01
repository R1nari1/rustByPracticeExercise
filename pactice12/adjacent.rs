use rand::Rng;
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}
fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_index = (0, 1);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = (i, i + 1);
        }
    }

    (min_sum, min_index.0, min_index.1)
}

fn print_vector(data: &[i32]) {
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:2}. ", i);
    }
    println!();

    print!("data:    [");
    for (i, &value) in data.iter().enumerate() {
        print!("{}", value);
        if i < data.len() - 1 {
            print!(", ");
        }
    }
    println!("]");

    let (min_sum, idx1, idx2) = min_adjacent_sum(data);
    println!("indexes: {}__ __{}", " ".repeat(idx1 * 3), " ".repeat((idx2 - idx1 - 1) * 3));
    println!("min adjacent sum={}+{}={} at indexes:{},{}", data[idx1], data[idx2], min_sum, idx1, idx2);
}

fn main() {
    let n = 20;

    for _ in 0..4 {
        let random_vector = gen_random_vector(n);
        print_vector(&random_vector);
        println!();
    }
}
