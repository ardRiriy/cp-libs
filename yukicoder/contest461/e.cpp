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

void solve() {
    ll a, b, c, d, n; cin >> a >> b >> c >> d >> n;
    ll p, q, r, s, t; cin >> p >> q >> r >> s >> t;

    rep(t1, c+1) {
        rep(t2, d+1) {
            ll rhs = (q-p)*n-(q-r)*t1-(q-s)*t2;
            if((rhs + (q-p)*100) % (q-p) != 0) continue;
            ll x = rhs / (q-p);

            rhs = t-(r-p)*t1-(s-p)*t2;
            if((rhs + (q-p)*100) % (q-p) != 0) continue;
            ll y = rhs / (q-p);
            cerr << x << ' ' << y << ' ' << t1 << ' ' << t2 << '\n';

            if(x<=a && y<=b) {
            
                cout << x << ' ' << y << ' ' << t1 << ' ' << t2 << '\n';
                return;
            }

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


