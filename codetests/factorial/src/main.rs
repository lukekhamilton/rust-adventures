fn f(n: u64) -> u64 {
    match n  {
        0 => 1,
        1 => 1,
        _ => f(n -1) * n,
    }
}

fn ff(n: u64) -> u64 {
    if n < 2 {
        1
    } else {
        n * ff(n-1)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_f(){
        let x = f(1);
        assert_eq!(x,1);
    }

    #[test]
    fn test_f_1(){
        let x = f(6);
        assert_eq!(x,720);
    }


    #[test]
    fn test_ff(){
        let x = ff(1);
        assert_eq!(x,1);
    }

    #[test]
    fn test_ff_1(){
        let x = ff(6);
        assert_eq!(x,720);
    }
}
