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
using djks=priority_queue<P, vp, greater<P>>;

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

mint fact[5005];
mint choose_memo[5005][5005];
bool choose_visited[5005][5005];

mint choose(int n, int k) {
    if (k > n) return 0;
    if (k == 0 || k == n) return 1;
    
    if (choose_visited[n][k]) {
        return choose_memo[n][k];
    }
    
    choose_visited[n][k] = true;
    return choose_memo[n][k] = fact[n] / (fact[k] * fact[n-k]);
}

int cnt[26];
mint memo[26*5005];
bool visited[26*5005];

mint rec(int i, int left) {
    if(i==26) {
        return (left==0)?1:0;
    }

    int key = i*5005+left;
    if(visited[key]) {
        return memo[key];
    } 
    visited[key] = true;

    mint res = 0;
    rep(j, min(left+1,cnt[i]+1)) {
        res = res + rec(i+1, left-j) * choose(left, j);
    }
    
    return memo[key] = res;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);

    fact[0] = 1;
    rep(i,5003){
        fact[i+1] = fact[i]*(i+1);
    }

    rep(i, 26) {
        rep(j, 5005) visited[i*5005+j] = false;
        cnt[i] = 0;
    }

    rep(i, 5005) rep(j,5005) choose_visited[i][j] = false;

    string s; cin >> s;
    for(char c: s) cnt[c-'a']++;
    mint ans = 0;
    rep(i,s.size()) {
        mint res = rec(0, i+1);
        ans = ans + res;
    }
    cout << ans.value << '\n';
}


