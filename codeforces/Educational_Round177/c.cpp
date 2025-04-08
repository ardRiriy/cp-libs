#include <bits/stdc++.h>
#include <atcoder/dsu>
using namespace atcoder;
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
    int n; cin >> n;
    vll p(n); rep(i,n)cin>>p[i],p[i]--;
    vll q(n); rep(i,n)cin>>q[i],q[i]--;

    dsu uf(n);
    rep(i,n) {
        uf.merge(p[i], p[p[i]]);
    }

    vector<bool> seen(n, false);
    vll ans(n);
    ll cur=0;
    rep(i,n){
        if(!seen[uf.leader(p[q[i]])]) {
            cur+=uf.size(p[q[i]]);
            seen[uf.leader(p[q[i]])]=true;
        }
        ans[i]=cur;
    }

    rep(i,n)cout<<ans[i]<<((i+1==n)?'\n':' ');
}


int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--)solve();
}
