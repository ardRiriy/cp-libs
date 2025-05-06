#include <bits/stdc++.h>
#include "input.hpp"
#include "atcoder/modint.hpp"
#include "atcoder/dsu.hpp"
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

#define rep(i,n) for (ll i=0;i<(n);++i)
#define per(i,n) for(ll i=(n)-1;i>=0;--i)
#define rep2(i,a,n) for (ll i=(a);i<(n);++i)
#define per2(i,a,n) for (ll i=(n)-1;i>=(a);--i)


template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};

int a[401][401];
int cnt[401];
using mint = modint998244353;
mint dfs(vector<int>& v) {

    int n = v.size();
    dsu uf(n); 
    rep(i,n){
        cnt[v[i]] = 0;
        rep(j,n){
            if(a[v[i]][v[j]]==1) {
                cnt[v[i]]++;
                uf.merge(i,j);
            }
        }
    }

    unordered_map<int,vector<int>> mp;
    rep(i,n) {
        if(uf.size(i) != cnt[v[i]]){
            // まだ除去できない
            mp[v[uf.leader(i)]].emplace_back(v[i]);
        } else {
            mp[v[uf.leader(i)]]; // do nothing(イテレータの生成だけする)
        }
    }

    mint res = 1;
    for(auto [leader, nv]: mp) {
        mint deleted = uf.size(leader) - nv.size();
        if(deleted==0){
            return 0;
        }
        if(!nv.empty()){
            res *= deleted * dfs(nv);
        } else {
            res *= deleted;
        }
    }
    return res;
}

void solve() {
    int n;cin>>n;
    rep(i,n)rep(j,n)cin>>a[i][j];
    // 1を先に除去
    rep(i,n){
        if(a[0][i]!=1){
            cout << "0\n";
            return;
        }
    }
    vector<int> v(n-1);
    iota(all(v), 1);
    auto ans = dfs(v);
    cout << ans.val() << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}
