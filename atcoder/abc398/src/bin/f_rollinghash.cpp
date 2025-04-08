#include <bits/stdc++.h>
#include <atcoder/all>

using namespace std;
using namespace atcoder;

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

// thanks: https://kyoroid.github.io/algorithm/string/rolling_hash.html
class RollingHash{
    private:
        const __uint128_t modulo = (1LL << 62) - 1;
        __uint128_t base;
        vector<__uint128_t> prefix;
        vector<__uint128_t> power;
        int n;

    public:
        RollingHash(string& S, long long base = 0) {
            random_device seed_gen;
            mt19937 engine(seed_gen());

            if(base==0)base = engine();
            n = S.size();

            prefix = vector<__uint128_t>(n+1, 0);
            power = vector<__uint128_t>(n+1, 1);
            for(int i = 0; i<n; i++) {
                int c = S[i];
                prefix[i+1] = (prefix[i] * base + c) % modulo;
                power[i+1] = (power[i] * base) % modulo;
            }
        }
    
        long long hash(int l, int r) {
            __uint128_t res = prefix[r] + modulo - ((power[r-l] * prefix[l]) % modulo);
            res %= modulo;
            return (long long)res;
        }
};

void solve() {
    random_device seed_gen;
    mt19937 engine(seed_gen());
    string s; cin >> s;
    ll n = s.size();
    string rev_s = s; reverse(all(rev_s));

    long long base = engine();
    RollingHash sh(s, base), rsh(rev_s, base);    
    rep(i, n) {
        if(sh.hash(i, n) == rsh.hash(0, n-i)) {
            cout << s;
            per(j, i) {
                cout << s[j];
            }

            cout << '\n';
            return;
        }  
    }


}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


