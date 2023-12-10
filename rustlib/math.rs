pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

pub fn lcm_of_vec(numbers: &[usize]) -> usize {
    numbers.iter().cloned().fold(1, |acc, num| lcm(acc, num))
}

pub fn lagrange_interpolation(numbers: &[isize], x: isize) -> isize {
    // Calculate the Lagrange interpolation polynomial at the point x
    let mut result = 0.0;

    for i in 0..numbers.len() {
        let mut term = numbers[i] as f64;

        for j in 0..numbers.len() {
            if i != j {
                term *= (x as isize - j as isize) as f64 / (i as isize - j as isize) as f64;
            }
        }

        result += term;
    }

    return result.round() as isize;
}
