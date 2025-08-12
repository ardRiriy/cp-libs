#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;

// using namespace atcoder;

#ifdef ADRY
#include <dbg.h>
#else
// DO NOTHING
#define dbg(...)
#endif


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

map<vector<int>, ll> memo;
ll dfs(const int n, int i, vector<vector<int>>& state, vector<int>& win_counts) {
    if(memo.find(win_counts) != memo.end()) return memo[win_counts];
    if(n==i) return 1;

    vector<int> idxs;
    int left = 0;
    rep(j,n) {
        if(state[i][j] == 0) idxs.emplace_back(j),  left++;
    }

    int x = n/2;
    if(x < win_counts[i] || x > win_counts[i] + left) {
        return 0;
    }

    vector<int> part(left,2);
    

    rep(j,x-win_counts[i]) {
        part[j] = 1;
    }
    ll res = 0;
    do {
        rep(j,left) {
            state[i][idxs[j]] = part[j];
            state[idxs[j]][i] = (part[j]==1)?2:1;
            if(part[j]==1) {
                win_counts[i]++;
            } else {
                win_counts[idxs[j]]++;
            }

            
        }
        res += dfs(n,i+1,state,win_counts);
        rep(j,left) {
            if(part[j]==1) {
                win_counts[i]--;
            } else {
                win_counts[idxs[j]]--;
            }
            state[i][idxs[j]] = 0;
            state[idxs[j]][i] = 0;
        }
        
    } while(next_permutation(all(part)));
    
    return memo[win_counts] = res;
}

int solve(){
    int n, m;cin>> n;
    if(n==0) return 1;
    cin >> m;
    memo.clear();
    // 0: 未定
    // 1: 勝利
    // 2: 敗北
    vector<vector<int>> state(n,vector<int>(n,0));
    vector<int> lefts(n,0);
    int w,l;
    rep(_,m) {
        cin >> w >> l;
        w--;
        l--;
        state[w][l] = 1;
        state[l][w] = 2;
        lefts[w]++;
    }

    ll ans = dfs(n,0,state,lefts);
    cout << ans << '\n';
    return 0;
}

int main() {
    #ifdef ADRY
    #else
        cin.tie(0);
        ios::sync_with_stdio(false);
    #endif
    while(!solve());
}


