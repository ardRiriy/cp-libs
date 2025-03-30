
#include <bits/stdc++.h>
#include <atcoder/all>
using namespace atcoder;
using namespace std;

#define all(v) v.begin(),v.end()
#define resort(v) sort(v.rbegin(),v.rend())
using ll = long long;
using ull = unsigned long long;
using vll=vector<ll>;
using vvll = vector<vector<ll>>;
using P = pair<ll,ll>;
using vp=vector<pair<ll, ll>>;

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
    int n; cin >> n;
    string s; cin >> s;

    int cnt = 0;
    int last_start = 0;
    char prev = '^';
    vector<tuple<char, int, int>> rle;

    rep(i, n) {
        if(prev == s[i]) {
            cnt ++;
        } else if(prev != '^') {
            rle.push_back({prev, cnt, last_start});
            last_start = i;
            cnt = 1;
        }
        prev = s[i];
    }
    if(cnt != 0 && prev != '^') rle.push_back({prev, cnt, last_start});


    // dp[i][j] := i番目にきて、直前の値がjとなった時の操作数としてありうる最大値
    vvll dp(n+1, vll(2, -inf));
    dp[0][0] = 0;
    ll idx = 0;

    rep(i, n-1) {
        if(i > 0 && s[i-1] != s[i]) idx++;
                
        ll k = s[i] - '0';
        rep(j, 2) {
            if(dp[i][j] == -inf) continue;
            // 次へ
            ll val = dp[i][j] + ((s[i+1] == '1' && j == 1) ? 1 : 0);
            dp[i+1][s[i+1]-'0'] = max(dp[i+1][s[i+1]-'0'], val);
        }

        // ここから
        if(k==0 && idx+2<rle.size()) {
            ll additional = get<1>(rle[idx+1]) - 1;

            if(i>0 && s[i-1] == '1') additional ++;

            ll ni = idx+2;
            cerr << "ni: " << ni << '\n';
            
            ll d = get<2>(rle[idx+1]) - i;
            cerr << "get<2>(rle[idx+1]): " << get<2>(rle[idx+1]) << '\n';
            cerr << "i: " << i << '\n';
            cerr << "d: " << d << '\n';

            auto [_, j_cnt, j_base_i] = rle[idx+2];
            if(j_cnt >= d) {
                ll next_idx = j_base_i + d - 1;
                cerr << "next_idx: " << next_idx << '\n';

                chmax(dp[min(next_idx, ll(n))][1], max(dp[i][0], dp[i][1]) + d + additional);
            }
        }
    }
    rep(i, 2) {
        rep(j, n+1) {
            cerr << ((dp[j][i] == -inf) ? -1 : dp[j][i]) << ' ';
        }
        cerr << '\n';
    } 
    cout << max(dp[n-1][0], dp[n-1][1]) << '\n';

}

int main() {
    // cin.tie(0);
    // ios::sync_with_stdio(false);
    int t;
    cin >> t;
    
    while(t--) solve();
}
