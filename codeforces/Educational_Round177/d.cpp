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
mint fact[300003];

const int n = 26;
vector<int> c(n);

mint choose(int n, int k) {
    return fact[n] / (fact[k] * fact[n-k]);
}

map<pair<int, int>, mint> memo;
mint rec(int i, int e, int o) {
    if(i==n) {
        if(e==0&&o==0){
            return 1;
        } else {
            return 0;
        }
    }
    if(e>o) swap(e, o);
    P p = {e,o};
    if(memo.find(p)!=memo.end()){
        return memo[p];
    }
    if(c[i]==0){
        return rec(i+1, e, o);
    }

    mint res = 0;
    if(e>=c[i]) {
        res = res + choose(e, c[i]) * rec(i+1, e-c[i], o);
    }
    if(o>=c[i]){
        res = res + choose(o, c[i]) * rec(i+1, e, o-c[i]);
    }

    return memo[p]=res;

}

void solve() {
    int sum = 0;
    memo.clear();
    rep(i,n)cin>>c[i], sum+=c[i];

    int even = sum/2;
    int odd = sum-even;

    cout << rec(0, even, odd).value << '\n';

}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    
    fact[0] = 1;
    rep(i,300001){
        fact[i+1] = fact[i]*(i+1);  
    }

    while(t--)solve();
}
