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

const int inf=1<<30;
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
    int n,k;cin>>n>>k;
    string s;cin>>s;
    
    vector<int> cur(k,inf);
    vector<vector<int>> next_appeer(n+1, vector<int>(k, inf));

    per(i,n){
        next_appeer[i+1]=cur;
        cur[s[i]-'a']=i+1;
    }
    next_appeer[0]=cur;
    vector<int> needy(n, inf); // 最後がiだったときに必要な個数の最小
    per(i,n){
        int mn = inf;
        rep(j,k){
            if(next_appeer[i+1][j]==inf) mn = 1;
            else chmin(mn, needy[next_appeer[i+1][j]-1]+1);
        }
        needy[i]=mn;
    }

    int q;cin>>q;
    string t;
    rep(_,q){
        cin>>t;
        int c = 0;
        int cnt = 0;

        rep(i,t.size()){
            c = next_appeer[c][t[i]-'a'];
            if(c==inf) break;
            cnt++;
        }

        if(cnt!=t.size()){
            cout<<"0\n";
            continue;
        }

        cout << needy[c-1] << '\n';
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    // cin >> t;
    while(t--)solve();
}


