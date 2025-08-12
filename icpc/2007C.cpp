#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;

// using namespace atcoder;

#ifdef ADRY
#include <dbg.h>
#else
// DO NOTHING
#define dbg(x)
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

int solve(){
    int n, w, d; cin >> n >> w >> d;
    if(n==0 && d==0 && w==0) return 1;

    vector<pair<int,int>> rectangles;
    rectangles.push_back({d,w});
    //cutted.emplace_back(false);

    auto cut = [&](int idx, int si) {
        // idx番目のケーキをsiでカットする

        auto [i,j] = rectangles[idx];
        si %= 2*(i+j);
        if(si<=j || (si>(i+j) && si<=(i+2*j))) {
            int l = (si<=j) ? si : si-(i+j);
            int other = j-l;
            rectangles.push_back({i,min(l,other)});
            rectangles.push_back({i,max(l,other)});
        } else {
            int l = (si<=i+j) ? si-j : si-(2*j+i);
            int other = i-l;
            rectangles.push_back({min(l,other), j});
            rectangles.push_back({max(l,other), j});
        }
        rectangles.erase(rectangles.begin()+idx);
    };

    int p, s;
    rep(i,n) {
        cin >> p >> s;
        p--;
        cut(p,s);
    }

    vll ans;
    rep(i,rectangles.size()) {
        ans.emplace_back(rectangles[i].first*rectangles[i].second);
    }
    sort(all(ans));
    rep(i,ans.size()) cout << ans[i] << " \n"[i+1==ans.size()];

    return 0;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}


