#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;

// using namespace atcoder;

#ifdef ADRY
#include <dbg.h>
#else
// DO NOTHING
#define dbg(x)
#endif


#define all(v) v.begin(),v.end()
#define resort(v) sort(v.rbegin(),v.rend())
using ll = long long;
using ull = unsigned long long;
using vll=vector<ll>;
using vvll = vector<vector<ll>>;
using P = pair<ll,ll>;
using vp=vector<pair<ll, ll>>;
using djks=priority_queue<P, vp, greater<P>>;

const ll inf=1ll<<60;
#define mod10 (ll)1e9+7
#define mod99 (ll)998244353
const double PI = acos(-1);

#define rep(i,n) for (ll i=0;i<(n);++i)
#define per(i,n) for(ll i=(n)-1;i>=0;--i)
#define rep2(i,a,n) for (ll i=(a);i<(n);++i)
#define per2(i,a,n) for (ll i=(n)-1;i>=(a);--i)


template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};

// https://algo-method.com/tasks/553/editorial
// Miller-Rabin 素数判定法
template<class T> T pow_mod(T A, T N, T M) {
    T res = 1 % M;
    A %= M;
    while (N) {
        if (N & 1) res = (res * A) % M;
        A = (A * A) % M;
        N >>= 1;
    }
    return res;
}

bool is_prime(long long N) {
    if (N <= 1) return false;
    if (N == 2 || N == 3) return true;
    if (N % 2 == 0) return false;
    vector<long long> A = {2, 325, 9375, 28178, 450775,
                           9780504, 1795265022};
    long long s = 0, d = N - 1;
    while (d % 2 == 0) {
        ++s;
        d >>= 1;
    }
    for (auto a : A) {
        if (a % N == 0) return true;
        long long t, x = pow_mod<__int128_t>(a, d, N);
        if (x != 1) {
            for (t = 0; t < s; ++t) {
                if (x == N - 1) break;
                x = __int128_t(x) * x % N;
            }
            if (t == s) return false;
        }
    }
    return true;
}

// Pollard のロー法
long long gcd(long long A, long long B) {
    A = abs(A), B = abs(B);
    if (B == 0) return A;
    else return gcd(B, A % B);
}
    
long long pollard(long long N) {
    if (N % 2 == 0) return 2;
    if (is_prime(N)) return N;

    long long step = 0;
    auto f = [&](long long x) -> long long {
        return (__int128_t(x) * x + step) % N;
    };
    while (true) {
        ++step;
        long long x = step, y = f(x);
        while (true) {
            long long p = gcd(y - x + N, N);
            if (p == 0 || p == N) break;
            if (p != 1) return p;
            x = f(x);
            y = f(f(y));
        }
    }
}

vector<long long> prime_factorize(long long N) {
    if (N == 1) return {};
    long long p = pollard(N);
    if (p == N) return {p};
    vector<long long> left = prime_factorize(p);
    vector<long long> right = prime_factorize(N / p);
    left.insert(left.end(), right.begin(), right.end());
    sort(left.begin(), left.end());
    return left;
}


int solve(){
    ll p; cin >> p;
    if(p==0) return 1;
    auto a = prime_factorize(p);
    
    auto uniqued_a = a;
    uniqued_a.erase(unique(all(uniqued_a)), uniqued_a.end());

    map<ll,ll> mp;
    for(auto x: a) mp[x]++;
    dbg(a);
    dbg(mp);
    vll divisors;

    auto dfs = [&](auto self, ll cur, int i) {
        if(i==uniqued_a.size()) {
            divisors.emplace_back(cur);
            return;
        }

        ll muler = 1;
        rep(c, mp[uniqued_a[i]]+1) {
            cur *= muler;
            self(self,cur,i+1);
            cur /= muler;
            muler *= uniqued_a[i];
        }
        return;
    };

    dfs(dfs,1,0);
    sort(all(divisors));

    dbg(lower_bound(all(divisors), 100000) - divisors.begin());

    ll ans = inf;
    for (auto a : divisors) {
        if (a * a * a > p) break;
        ll left = p / a;
        ll root = sqrtl((long double)left);
        auto it = upper_bound(divisors.begin(), divisors.end(), root);
        while (it != divisors.begin()) {
            --it;
            ll d = *it;
            if (d < a) break;
            if (left % d == 0) {
                ll h = left / d;
                ans = min(ans, a + d + h);
                break;
            }
        }
    }
    cout << ans << '\n';
    return 0;
}
int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}


