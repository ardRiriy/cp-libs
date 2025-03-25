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
    int n, ka, kb; cin >> n >> ka >> kb;
    vll va(ka), vb(kb);
    vvll a(n, vll(2, -1));

    rep(i,ka) {
        int val; cin >> val; val--;
        a[val][0] = 1;
        va[i] = val;
    }
    rep(i,kb){
        int val; cin >> val;val--;
        a[val][1] = 1;
        vb[i] = val;
    }

    ll diff = inf;
    int ai = 0, bi=0;
    vvll dist(n, vll(2, inf));
    while(ai<ka&&bi<kb) {
        diff = min(diff,abs(va[ai] - vb[bi]));
        if(va[ai] < vb[bi]) ai++;
        else bi++;
    }
    rep(i, n) {
        if(a[i][0] == 1) {
            dist[i][0]=0LL;
            dist[i][1] = min(dist[i][1], diff);
        }
        if(a[i][1] == 1) {
            dist[i][1]=0LL;
            dist[i][0] = min(dist[i][0], diff);
        }
    }

    ll d_r = inf, d_b = inf;
    rep(i, n) {
        dist[i][0] = min(dist[i][0], d_r);
        dist[i][1] = min(dist[i][1], d_b);

        d_r = min(d_r+1, dist[i][0] + 1);
        d_b = min(d_b+1, dist[i][1]+1);
    }

    d_r = inf, d_b = inf;
    per(i, n) {
        dist[i][0] = min(dist[i][0], d_r);
        dist[i][1] = min(dist[i][1], d_b);

        d_r = min(d_r+1, dist[i][0] + 1);
        d_b = min(d_b+1, dist[i][1]+1);
    }

    int q, s, t; cin >> q;
    rep(_, q) {
        cin >> s >> t; s--; t--;
        cout << min(min(dist[s][0]+dist[t][0], dist[s][1] + dist[t][1]), ll(max(s,t)-min(s,t))) << '\n';
    }

}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


