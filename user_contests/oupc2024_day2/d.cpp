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
    ll k, a, b; cin >> k >> a >> b;
    
    vll dist(k, inf);
    priority_queue<P> pq;
    pq.push({-(a+b), 1%k});

    while(!pq.empty()) {
        auto [c, i] = pq.top(); pq.pop();

        if(dist[i] != inf) continue;
        dist[i] = -c;

        ll ni = (i*2)%k;
        if(dist[ni] == inf)
            pq.push({-(dist[i] + b), ni});
        
        ni = (ni+1)%k;
        if(dist[ni] == inf) {
            pq.push({-(dist[i] + a + b), ni});
        }
    }

    cout << dist[0] << '\n';
} 

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}
