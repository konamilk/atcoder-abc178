use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input!{
        n:i32
    }

    let ans = if n == 1 {0} else {1};
    println!("{}", ans)
}
