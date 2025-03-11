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

vll divisor(ll n) {
    vll ret;
    for (ll i = 1; i * i <= n; i++) {
        if (n % i == 0) {
            ret.push_back(i);
            if (i * i != n) ret.push_back(n / i);
        }
    }
    return ret;
}

bool dfs(vll &v, ll target){
    int n = v.size();
    // rep(i, n) {
    //     cout << v[i] << ' ';
    // }
    // cout << '\n';

    vll divs = divisor(n);
    bool flag = true;
    rep(i, n) {
        flag = flag && v[i] == target;
    }
    if(flag) return true;

    for(ll div: divs) {
        if (div == 1) continue;
        ll len = n/div;
        vll nv(len, 0);

        rep(i, n) {
            nv[i/div] += v[i]; 
        }
        bool flag = true;
        rep(i, len) {
            flag = flag && nv[i] % div == 0;
            nv[i] /= div;
        }
        if(!flag) continue;
        if(dfs(nv, target)) return true;
    }
    return false;
}

void solve() {
    ll n, x; cin >> n >> x;
    vll a(n); rep(i, n) cin >> a[i];
    if(dfs(a, x)) {
        cout << "Yes\n";
    } else {
        cout << "No\n";
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--)solve();
}