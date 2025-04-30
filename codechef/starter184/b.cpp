#include <bits/stdc++.h>
#include "input.hpp"
#include "atcoder/modint"

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

using mint = modint998244353;

void solve() {
    int n; cin >> n;
    auto a = i64_vec_IN(n);

    vector<int> cnt(n);
    for(auto ai: a) cnt[min(ai, n-ai-1)]++;

    rep(i,n/2){
        if(cnt[i]!=2){
            cout << "0\n";
            return;
        }
    }

    if(n%2==1) {
        if(cnt[n/2]!=1) {
            cout << "0\n";
            return;
        }
    }
    mint ans = 1;
    rep(i,n/2){
        ans*=cnt[i];
    }
    cout << ans.val() << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


