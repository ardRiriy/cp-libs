#include <bits/stdc++.h>

using namespace std;

// using namespace atcoder;

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

void solve() {
    ll n; cin >> n;

    vvll g(n);
    
    rep(i, n) {
        cout << "? " << i+1 << " " << n;
        rep(j, n) {
            cout << " " << j+1;
        }
        cout << endl;
        
        ll d; cin >> d;
        g[d-1].push_back(i);
    }
    
    ll cur = -1;
    ll d = -1;
    per(i, n) {
        if(g[i].empty()) continue;
        cur = g[i][0];
        d = i;
        break;
    }
    
    vll ans; 
    ans.push_back(cur);
    
    while(d>0) {
        for(auto ni: g[d-1]) {
            cout << "? " << cur+1 << " 2 " << cur+1 << " " << ni+1 << endl;
            ll res; cin >> res;
            if (res == 2) {
                cur = ni;
                d -= 1;
                ans.push_back(cur);
                break;
            }
        }
    }
    
    cout << "! " << ans.size();
    for(auto i: ans) {
        cout << " " << i+1;
    }
    cout << endl;
}

int main() {
    int t=1;
    cin >> t;
    while(t--)solve();
}


