// a^n % p を計算
// 64bitじゃないとオーバーフローするのでFrom<u64>をつけてる
pub fn modpow<T>(a: T, n: T, p: T) -> T 
where T: num::Integer + From<u64> + std::ops::Shr<usize, Output = T> + std::ops::BitAnd<Output = T> + Copy
{
    let mut x:T = a;
    let mut res :T = T::from(1);
    let one = T::from(1);
    for i in 0..64 {
        if (n >> i) & one == one {
            res = (res * x) % p;
        }
        x = (x * x) % p;
    }
    res % p
}
