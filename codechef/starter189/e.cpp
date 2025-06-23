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
    int n; cin >> n;
    string s; cin >> s;
    vector<pair<char, int>> rle;
    char prev = '-';
    int cnt = 0;

    int one_cnt = 0;
    int hatena_cnt = 0;
    for(char c: s) {
        if(c=='1') one_cnt++;
        if(c=='?') hatena_cnt++;

        if(c==prev) {
            cnt++;
        } else {
            if(prev != '-') {
                rle.push_back({prev, cnt});
            }
            prev = c;
            cnt = 1;
        }
    }
    rle.push_back({prev, cnt});

    if(hatena_cnt == 0) {
        vll ans(n+1, -1);
        ans[one_cnt] = rle.size();
        rep(i,n+1) cout << ans[i] << " \n"[i==n];
        return;
    }

    vll rank1;
    ll rank2 = 0; // コスト0なので
    vll rank3;
    rep(i, rle.size()) {
        auto [c, cnt] = rle[i];
        if(c!='?') {
            continue;
        }
        if(rle[i-1].first != rle[i+1].first) {
            rank2+=cnt;
        } else if(rle[i-1].first == '0') {
            rank3.push_back(cnt);
        } else {
            rank1.push_back(cnt);
        }
    }

    string t = s;
    ll cur = 1;
    rep(i,n-1) {
        char c = (s[i] == '1') ? '1' : '0';
        char d = (s[i+1] == '1') ? '1' : '0';
        if(c!=d) cur++;
    }
    sort(all(rank1));
    reverse(all(rank1));
    sort(all(rank3));
    vll ans(n+1, -1);
    ll i = 0;
    while(i<n+1) {
        if(i<one_cnt || one_cnt+hatena_cnt<i) {
            i++;
            continue;
        }
        dbg(ans);
        dbg(i);
        if(!rank1.empty()) {
            auto k = rank1[rank1.size()-1];
            rank1.pop_back();
            // 1~k-1まではcur
            rep(_, k) {
                ans[i] = cur;
                i++;
            }    
            cur-=2;
            ans[i] = cur;
            i++;
        } else if(rank2 > 0) {
            // ans[i] = cur;
            // i++;
            rep(_, rank2) {
                ans[i] = cur;
                i++;
            }
            rank2 = 0;
            dbg(i);
            
        } else {
            ans[i] = cur;
            i++;
            //assert(!rank3.empty());
            if(rank3.empty()) continue;
            auto k = rank3[rank3.size()-1];
            rank3.pop_back();
            cur+=2;
            rep(_,k) {
                ans[i] = cur;
                i++;
            }
        }
    }
    rep(i,n+1) cout << ans[i] << " \n"[i==n];
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}
