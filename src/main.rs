// algorithm Sieve of Eratosthenes is
//     input: an integer n > 1.
//     output: all prime numbers from 2 through n.

//     let A be an array of Boolean values, indexed by integers 2 to n,
//     initially all set to true.
//     for i = 2, 3, 4, ..., not exceeding √n do
//         if A[i] is true
//             for j = i2, i2+i, i2+2i, i2+3i, ..., not exceeding n do
//                 A[j] := false

//     return all i such that A[i] is true.

fn sieve(n: usize) -> Vec<usize> {
    let mut a: Vec<bool> = vec![true; n + 1];
    let upper = (n as f64).sqrt() as usize;
    for i in 2..upper {
        if a[i] {
            for j in (i * i..=n).step_by(i) {
                a[j] = false;
            }
        }
    }

    a.iter()
        .enumerate()
        .skip(2)
        .filter(|(_, b)| **b)
        .map(|(i, _)| i)
        .collect()
}

fn main() {
    let v = sieve(100);
    for (i, prime) in v.iter().enumerate() {
        println!("{}: {}", i, prime);
    }
}
