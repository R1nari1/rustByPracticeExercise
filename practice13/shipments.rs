fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        return usize::MAX;
    }

    let average = total / n;

    let mut moves: usize = 0;
    let mut surplus: usize = 0;
    for &shipment in shipments {
        if shipment > average {
            surplus += (shipment - average) as usize;
        } else {
            moves += (average - shipment) as usize;
        }
    }
    moves + surplus
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut shipments = vec![0; n];
    let total: u32 = n as u32 * (n as u32 / 2 + 1);

    for i in 0..n {
        shipments[i] = (total / n as u32) + (i % 2) as u32;
    }

    shipments
}

fn main() {
    let shipments1 = vec![8, 2, 2, 4, 4];
    let answer1 = count_permutation(&shipments1);
    if answer1 == usize::MAX {
        println!("вантаж не розподілено shipments1");
    } else {
        println!("кол-во переміщень для shipments1: {}", answer1);
    }

    let shipments2 = vec![9, 3, 7, 2, 9];
    let answer2 = count_permutation(&shipments2);
    if answer2 == usize::MAX {
        println!("вантаж не розподілено shipments2");
    } else {
        println!("кол-во переміщень для shipments2: {}", answer2);
    }

    let generated_shipments = gen_shipments(5);
    println!("створені вантажі: {:?}", generated_shipments);
}