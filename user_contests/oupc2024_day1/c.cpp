#include <bits/stdc++.h>
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

ll dp(vll& v) {
    ll n = v.size();

    vvll dp(n+1, vll(2, 0));

    rep(i, n) {
        chmax(dp[i+1][0], dp[i][0]);
        chmax(dp[i+1][1], dp[i][0] + v[i]);
        chmax(dp[i+1][0], dp[i][1]);
    }
    return max(dp[n][0], dp[n][1]);
}

void solve() {
    ll n; cin >> n;
    string s; cin >> s;

    vector<pair<char, ll>> v;
    ll cnt = 0;
    char prev = s[0];
    
    // RLE
    rep(i, n) {
        if(prev == s[i]) cnt++;
        else {
            v.push_back({prev, cnt});
            prev = s[i];
            cnt = 1;
        }
    }
    v.push_back({prev, cnt});

    ll vs = v.size();
    vvll dp(vs+1, vll(2, 0));
    
    rep(i, vs) {
        // 使わない
        chmax(dp[i+1][0], max(dp[i][0], dp[i][1]));
        
        if(i+2 < vs) {
            if((v[i].first == 'K' && v[i+1].first == 'Y' && v[i+2].first == 'B') || (v[i].first == 'B' && v[i+1].first == 'Y' && v[i+2].first == 'K')){
                chmax(dp[i+2][1], dp[i][0] + v[i].second + v[i+1].second + v[i+2].second);
                if(v[i].second > 1) {
                    chmax(dp[i+2][1], dp[i][1] + v[i+1].second + v[i+2].second);
                } 
            }
        } 
    }

    cout << max(dp[vs][0], dp[vs][1]) << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}
