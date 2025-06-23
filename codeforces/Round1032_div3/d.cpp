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

    #ifdef ADRY
        std::random_device seed_gen;
        std::mt19937 engine(seed_gen());
        int n = 40;
        vll p(2*n); iota(all(p), 1);
        shuffle(all(p), engine);
        vvll v(2, vll(n));
        rep(i,n) v[0][i] = p[i];
        rep(i,n) v[1][i] = p[i+n];
    #else
        int n; cin >> n;
        vvll v(2);
        v[0] = i64_vec_IN(n);
        v[1] = i64_vec_IN(n);
    #endif

    auto input = v;
    vp ans;

    rep(cur, 2*n) {
        ll val = cur+1;
        P idx = {-1,-1};
        rep(i,2) rep(j,n) {
            if(v[i][j] == val) {
                idx = {i,j}; 
                break; 
            }
        }
        assert(idx.first>=0);

        P pos = {(val>n)?1:0, cur%n};
        dbg(pos);
        if(idx.second>pos.second) {
            per2(i, pos.second, idx.second) {
                ans.push_back({idx.first+1, i+1});
                swap(v[idx.first][i],v[idx.first][i+1]);
            }
        } else {
            rep2(i, idx.second, pos.second) {
                ans.push_back({idx.first+1, i+1});
                swap(v[idx.first][i],v[idx.first][i+1]);
            }
        }
        
        if(pos.first != idx.first) {
            ans.push_back({3, pos.second+1});
            swap(v[0][pos.second], v[1][pos.second]);
        }

    }
    {
        bool checker = true;
        checker = checker && ans.size() <= 1709;
        rep(i,n-1) checker = checker && (v[0][i] < v[0][i+1]);
        rep(i,n-1) checker = checker && (v[1][i] < v[1][i+1]);
        rep(i,n) checker = checker && (v[0][i] < v[1][i]);
        if(!checker) {
            // cerr << n << '\n';
            // rep(i,n) cerr << input[0][i] << " \n"[i+1==n];
            // rep(i,n) cerr << input[1][i] << " \n"[i+1==n];

            // rep(i,2) rep(j,n) cerr << v[i][j] << " \n"[j+1==n];
            exit(1);
        }
    }

    cout << ans.size() << '\n';
    for(auto [i,x]: ans) {
        cout << i << " " << x << "\n";
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


