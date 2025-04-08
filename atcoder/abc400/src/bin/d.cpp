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
    int h, w; cin >> h >> w;
    vector<string> s(h);
    rep(i, h) cin >> s[i];
    int si, sj; cin >> si >> sj;
    si--; sj--;
    int gi, gj; cin >> gi >> gj;
    gi--; gj--;
    vector<vector<ll>> seen(h, vector<ll>(w, inf));
    priority_queue<pair<ll, pair<ll, ll>>, vector<pair<ll, pair<ll, ll>>>, greater<pair<ll, pair<ll, ll>>>> pq;
    pq.push({0, {-1, si*w+sj}});

    while(!pq.empty()){
        auto [d, p] = pq.top(); pq.pop();
        auto [r, v] = p;
        int i = v/w, j = v%w;
        if(seen[i][j] <= d) continue;
        seen[i][j] = d;

        rep(k, 4){
            int ni = i + dx[k], nj = j + dy[k];
            if(ni < 0 || ni >= h || nj < 0 || nj >= w) continue;
            if(s[ni][nj] == '.') {
                if(seen[ni][nj] == inf) {
                    pq.push({d, {-1, ni*w+nj}});
                }
            } else {
                if(seen[ni][nj] == inf) {
                    pq.push({d+1, {k, ni*w+nj}});
                    int nni = ni + dx[k], nnj = nj + dy[k];
                    if(nni >= 0 && nni < h && nnj >= 0 && nnj < w) {
                        pq.push({d+1, {-1, nni*w+nnj}});
                    }
                }
            }
        }
    }

    cout << seen[gi][gj] << '\n';
}

int main() {
    // cin.tie(0);
    // ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


