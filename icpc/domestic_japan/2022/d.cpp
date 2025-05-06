#include <bits/stdc++.h>
#include "input.hpp"

#include "atcoder/modint.hpp"

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

const int inf=1ll<<30;
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

using mint = modint998244353;
unordered_map<int, int> mp;
vector<int> s, t, q;
void solve(int n, int k) {
    int a;
    
    s.clear();
    rep(i,n) {
        cin >> a;
        s.emplace_back(a);
    }

    t.clear();
    rep(i,n) {
        cin >> a;
        t.emplace_back(a);
    }

    rep(i,n) {
        mp[t[i]] = i;
    }

    q.clear();
    rep(i,n){
        q.emplace_back(mp[s[i]]);
    }

    vector<vector<mint>> dp(2, vector<mint>(k+1,0));
    int cur = 0;
    dp[cur][0] = 1;
    
    rep(i,n) {
        int nxt = 1-cur;
        bool flag = i>0 && q[i-1] < q[i];
        bool flag2 = i==0 || q[i-1] > q[i] || s[i-1] < s[i];
        rep(j,k) {
            if(flag2) dp[nxt][j+1] += dp[cur][j];
            if(flag) dp[nxt][j] += dp[cur][j];
        }
        if(flag) dp[nxt][k] += dp[cur][k];
        
        // rep(j,k+1) cerr << "dp[" << i+1 << "][" << j << "]=" << dp[nxt][j].val() << '\n';
        
        fill(all(dp[cur]), 0);
        cur = nxt;
    }
    mint sum = 0;
    rep(i,k){
        sum+=dp[cur][i+1];
    }
    cout << sum.val() << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int n, k;
    cin >> n >> k;
    while(n) {
        solve(n, k);
        cin >> n >> k;
    }
}
