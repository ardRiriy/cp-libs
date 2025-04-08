#include <bits/stdc++.h>
using namespace std;

// --------------------------- //
constexpr long long gcd(long long a, long long b) {
    return b == 0 ? a : gcd(b, a%b);
}

class Rational {        
    public:
        long long num;
        long long den;

        Rational(long long numer, long long denom) : num(numer), den(denom) {
            assert(den != 0);
            reduce();
        }

        Rational operator+(const Rational& other) const {
            return Rational(num * other.den + other.num * den, den * other.den);
        }

        Rational operator-(const Rational& other) const {
            return Rational(num * other.den - other.num * den, den * other.den);
        }

        Rational operator*(const Rational& other) const {
            return Rational(num * other.num, den * other.den);
        }

        Rational operator/(const Rational& other) const {
            if (other.num == 0) {
                throw std::invalid_argument("ゼロによる除算はできません");
            }
            return Rational(num * other.den, den * other.num);
        }

        friend std::ostream& operator<<(std::ostream& os, const Rational& r) {
            os << r.num << "/" << r.den;
            return os;
        }
    private:
        void reduce() {
            long long g = gcd(num, den);
            num /= g;
            den /= g;
            if(den<0){
                num = -num;
                den = -den;
            }
        }      
};

bool operator==(const Rational& lhs, const Rational& rhs) {
    return lhs.num == rhs.num && lhs.den == rhs.den;
}

bool operator!=(const Rational& lhs, const Rational& rhs) {
    return !(lhs == rhs);
}

bool operator<(const Rational& lhs, const Rational& rhs) {
    return lhs.num * rhs.den < rhs.num * lhs.den;
}

bool operator>(const Rational& lhs, const Rational& rhs) {
    return rhs < lhs;
}

bool operator<=(const Rational& lhs, const Rational& rhs) {
    return !(rhs < lhs);
}

bool operator>=(const Rational& lhs, const Rational& rhs) {
    return !(lhs < rhs);
}
