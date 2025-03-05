fn fib(n: u32) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }

    let mut result: Vec<u32> = vec![0, 1];

    if n == 1 {
        return result;
    }

    for i in 2..=n {
        let next_fib = result[(i - 1) as usize] + result[(i - 2) as usize];
        result.push(next_fib);
    }

    return result;
}

fn p_square() {
    println!("     1  2  3  4  5  6  7  8  9 10");

    for i in 1..=9 {
        print!("{} |", i);
        for j in 1..=10 {
            print!("{:3}", i * j);
        }
        println!();
    }
}

fn main() {
    let n1 = 1;
    println!("fib({}) = {:?}", n1, fib(n1));

    let n10 = 10;
    println!("fib({}) = {:?}", n10, fib(n10));

    p_square();
}