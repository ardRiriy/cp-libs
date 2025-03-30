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

void solve() {
    int n; cin >> n;
    vll a(2*n); rep(i, 2*n) cin >> a[i], a[i]--;


    vp pos(n, P(-1, -1));
    rep(i, 2*n) {
        if(pos[a[i]].first == -1) pos[a[i]].first = i;
        else pos[a[i]].second = i;
    }

    ll v[] = {-1, 1};
    ll ans = 0;
    set<P> checked;
    rep(i, n) {

        
        auto [p1, p2] = pos[i];
        assert(p1 != -1 && p2 != -1);
        if(abs(p1-p2) == 1) {
            continue;
        }

        rep(j, 2) {
            rep(k, 2) {
                ll p1n = p1 + v[j];
                ll p2n = p2 + v[k];
                if(p1n < 0 || p2n < 0 || p1n >= 2*n || p2n >= 2*n) continue;
                if(p1n == p2n) continue;
                if(abs(p1n-p2n) == 1) continue;
                if(a[p1n] == a[p2n]) {
                    checked.insert({a[p1], a[p1n]});
                    checked.insert({a[p1n], a[p1]});
                }
            }
        }
    }

    cout << checked.size() / 2 << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


