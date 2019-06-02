#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub fn fib(count: u32) -> u32 {
    if count < 2 {
        return count;
    }

    let (_, ret) = (0..count-2).fold((0, 1), |(former, ret), _| (ret, ret + former));
    ret
}

#[no_mangle]
pub fn fib_array(count: u32) -> Vec<u32> {
    match count {
        0 => vec![],
        1 => vec![0],
        2 => vec![0, 1],
        _ => (2..count).fold(vec![0, 1], |mut acc, i| {
            let new = acc[(i-1) as usize] + acc[(i-2) as usize];
            acc.push(new);
            acc
        })
    }
}

fn main() {
    println!("{:?}", add(10, -100));
    println!("{:?}", fib(40));
    println!("{:?}", fib_array(10));
    assert!(fib_array(10).len() == 10);
}

