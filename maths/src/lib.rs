pub fn nth(n: u32)-> Option<u32> {
    if n<=0 {
        return None;
    }

    let mut primes: Vec<u32> = vec![];
    let index = n as usize;
    let mut i = 2;

    loop {
        if is_prime(i){
            primes.push(i);
        }
        if primes.len() >= index{
            break;
        }
        i = i+ 1;
    }
    let maybe_val = primes.get(index - 1);

    match maybe_val{
        Some(val )=> return Some(*val),
        _ => return None,
    }
}

fn is_prime(n: u32)-> bool
{
    for i in 2..n{
        if n%i == 0{
            return false;
        }
    }
    return n !=1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime() {
        let result = nth(9);
        assert_eq!(result, Some(23));
    }
}
