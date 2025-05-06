#include <bits/stdc++.h>
#include "input.hpp"

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
    int n;cin>>n;
    auto c = i64_vec_IN(n-1);
    auto a = i64_vec_IN(n-1);

    vector<bool> seen(n, false);
    seen[0] = true;

    rep2(i,1,n) {
        if(a[i-1]==0) continue;
        
        vector<int> from(n, 1<<30);
        deque<P> que;
        que.push_back({i, 1<<30});
        
        int last = 1<<30;
        while(!que.empty()){
            auto [p, f] = que.front(); que.pop_front();
            if(from[p] != (1<<30)) continue;
            from[p] = f;

            bool flag = false;
            rep2(j, p-c[p-1], p) {
                if(seen[j]) {
                    last=p;
                    flag = true;
                    break;
                } else {
                    que.push_back({j,p});
                }
            }
            if(flag) break;
        }


        while(last!=(1<<30)){
            seen[last]=true;
            last = from[last];
        }
    }
    ll ans = 0;
    rep(i,n) {
        // if(seen[i]) cerr << i << ": true\n";
        // else cerr << i << ": false\n";
        ans += (seen[i])?1:0;
    }
    cout << ans-1 << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


