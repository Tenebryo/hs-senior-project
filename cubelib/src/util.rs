fn factorial(n : u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n*factorial(n-1)
    }
}

fn choose(n : u32, k : u32) -> u32 {
    factorial(n)/(factorial(k)*factorial(n-k))
}