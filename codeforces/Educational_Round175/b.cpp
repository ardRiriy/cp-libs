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

void solve() {
    ll n, x, k; cin >> n >> x >> k;
    string s; cin >> s;    

    // start to zero
    ll cur = x;
    ll t1 = inf;
    rep(i, s.size()) {
        cur += (s[i]=='L') ? -1 : 1;
        if(cur == 0) {
            t1 = i+1;
            break;
        }
    }

    if(t1 == inf) { 
        cout << 0 << '\n';
        return;
    }

    // zero to zero
    ll t2 = inf;
    rep(i, s.size()) {
        cur += (s[i]=='L') ? -1 : 1;
        if(cur == 0) {
            t2 = i+1;
            break;
        }
    }

    if (t2==inf) {
        cout << 1 << '\n';
        return;
    }

    if(k<t1) cout << 0 << '\n';
    else {
        ll left = k-t1;
        ll cnt = 1;
        cnt += left / t2;
        
        cout << cnt << '\n';
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--)solve();
}
