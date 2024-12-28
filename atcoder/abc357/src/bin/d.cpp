#include <iostream>
#include <string>
#include <cstdint>
#include <cassert>

// オイラーのトーシェント関数
constexpr uint32_t totient(uint32_t x) {
    uint32_t ans = x;
    for (uint32_t i = 2; i * i <= x; i++) {
        if (x % i == 0) {
            ans /= i;
            ans *= i - 1;
            do x /= i; while (x % i == 0);
        }
    }
    if (x != 1) {
        ans /= x;
        ans *= x - 1;
    }
    return ans;
}

// Modint構造体の定義
template<uint32_t P>
struct Modint {
    static_assert(P < 0x80000000, "P must be smaller than 2^31");
    uint32_t a;
    Modint<totient(P)> b;
private:
    static uint32_t mod(uint64_t x) {
        return x < P ? x : x % P;
    }
    static uint32_t mul(uint32_t a, uint32_t b) {
        return mod(uint64_t(a) * b);
    }
    static uint32_t pow(uint32_t a, uint32_t b) {
        uint32_t ans = 1;
        while (b) {
            if (b & 1) ans = mul(ans, a);
            a = mul(a, a);
            b >>= 1;
        }
        return ans;
    }
public:
    Modint(uint64_t x) : a(mod(x)), b(x) {}
    Modint(uint32_t a, Modint<totient(P)> b) : a(a), b(b) {}
    uint32_t val() const {
        return a;
    }
    Modint& operator*=(const Modint& other) {
        a = mul(a, other.a);
        b *= other.b;
        return *this;
    }
    Modint operator*(const Modint& other) const {
        return Modint(*this) *= other;
    }
    Modint& operator+=(const Modint& other) {
        a = mod(uint64_t(a) + other.a);
        b += other.b;
        return *this;
    }
    Modint operator+(const Modint& other) const {
        return Modint(*this) += other;
    }
    Modint& operator-=(const Modint& other) {
        a = mod(uint64_t(a) + P - other.a);
        b -= other.b;
        return *this;
    }
    Modint operator-(const Modint& other) const {
        return Modint(*this) -= other;
    }
    Modint pow(const Modint& other) const {
        return { pow(a, other.b.a), b.pow(other.b) };
    }
};

template<>
struct Modint<1> {
    uint32_t a;
    Modint(uint64_t x) : a(bool(x)) {}
    uint32_t val() const {
        return 0;
    }
    Modint& operator*=(const Modint& other) {
        a &= other.a;
        return *this;
    }
    Modint operator*(const Modint& other) const {
        return Modint(*this) *= other;
    }
    Modint& operator+=(const Modint& other) {
        a |= other.a;
        return *this;
    }
    Modint operator+(const Modint& other) const {
        return Modint(*this) += other;
    }
    Modint& operator-=(const Modint& other) {
        a &= !other.a;
        return *this;
    }
    Modint operator-(const Modint& other) const {
        return Modint(*this) -= other;
    }
    Modint pow(const Modint& other) const {
        return { a || !other.a };
    }
};

using ModInt = Modint<998244353>;

void solve() {
    uint64_t n;
    std::cin >> n;

    ModInt n_mod(n);
    uint64_t len = std::to_string(n).length();

    ModInt n_len = ModInt(len) * ModInt(n);
    
    ModInt power_10_n_len = ModInt(10).pow(n_len);
    
    ModInt ue = power_10_n_len - 1;
    ModInt sita = ModInt(10).pow(len) - 1;
    
    ModInt result = n_mod * (ue * sita.pow(998244351));
    
    std::cout << result.val() << std::endl;
}

int main() {
    solve();
    return 0;
}
