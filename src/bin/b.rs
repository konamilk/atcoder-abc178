use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        a:i64,
        b:i64,
        c:i64,
        d:i64
    }

    let kouho = vec![a*c, a*d, b*c, b*d];

    let ans = kouho.into_iter().max().unwrap();

    println!("{}", ans);
}
