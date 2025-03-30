#include <bits/stdc++.h>
using namespace std;

#define all(v) v.begin(),v.end()
#define resort(v) sort(v.rbegin(),v.rend())
using ll = long long;
using ull = unsigned long long;
using vll=vector<ll>;
using vvll = vector<vector<ll>>;
using P = pair<ll,ll>;
using vp=vector<pair<ll, ll>>;

const ll inf=1ll<<60;
#define mod10 (ll)1e9+7
#define mod99 (ll)998244353
const double PI = acos(-1);

#define rep(i,n) for (ll i=0;i<n;++i)
#define per(i,n) for(ll i=n-1;i>=0;--i)
#define rep2(i,a,n) for (ll i=a;i<n;++i)
#define per2(i,a,n) for (ll i=n-1;i>=a;--i)


template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};

template<int MOD>
struct ModInt {
    int value;

    ModInt() : value(0) {}
    ModInt(int value) : value(value % MOD) {
        if (this->value < 0) this->value += MOD;
    }

    ModInt operator+(const ModInt& other) const {
        return ModInt((value + other.value) % MOD);
    }

    ModInt operator-(const ModInt& other) const {
        return ModInt((value - other.value + MOD) % MOD);
    }

    ModInt operator*(const ModInt& other) const {
        return ModInt((1LL * value * other.value) % MOD);
    }

    ModInt operator/(const ModInt& other) const {
        return *this * other.inv();
    }

    ModInt inv() const {
        int a = value, b = MOD, u = 1, v = 0;
        while (b) {
            int t = a / b;
            a -= t * b; std::swap(a, b);
            u -= t * v; std::swap(u, v);
        }
        return ModInt(u);
    }
};

using mint = ModInt<998244353>;

void solve() {
    int n; cin >> n;
    vector<set<ll>> g(n); 
    rep2(i, 1, n) {
        ll p; cin >> p; p--;
        // p -> i
        g[p].insert(i);
    }

    vll d(n, inf);
    vvll d_list(n+1);
    d[0] = 0;
    d_list[0].push_back(0);
    deque<ll> que;
    que.push_back(0);
    while(!que.empty()) {
        ll p = que.front(); que.pop_front();
        for(ll ni: g[p]) {
            if (d[ni] != inf) continue;
            d[ni] = d[p] + 1;
            d_list[d[ni]].push_back(ni);
            que.push_back(ni);     
        }
    }

    mint prev_sum = 0;
    vector<mint> val(n, 0);
    per(i, n) {
        mint new_sum = 0;
        for(ll p: d_list[i]) {
            mint s = 0;
            for(ll ni: g[p]) {
                s = s + val[ni];
            }
            if(p==0) val[p] = s+1;
            else val[p] = prev_sum - s + 1;

            new_sum = new_sum + val[p];
        }
        prev_sum = new_sum;
    }
    cout << val[0].value << '\n';

}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--)solve();
}
