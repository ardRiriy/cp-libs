#include <bits/stdc++.h>
#include <atcoder/all>

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

#define rep(i,n) for (ll i=0;i<n;++i)
#define per(i,n) for(ll i=n-1;i>=0;--i)
#define rep2(i,a,n) for (ll i=a;i<n;++i)
#define per2(i,a,n) for (ll i=n-1;i>=a;--i)


template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};

void solve() {
    int n, k; cin >> n >> k;
    string s; cin >> s;



    // 前後から見て配置可能なOの個数
    vector<int> p(n+1, 0), r(n+1, 0);
    ll cnt = 0;
    // oの左右の処理
    rep(i,n) {
        if(s[i] == 'o') {
            if(i>0) s[i-1] = '.';
            if(i+1<n) s[i+1] = '.';
            cnt++;
        }
    }
    if(cnt==k) {
        // ?を全部埋める
        rep(i,n) {
            if(s[i]=='?') {
                s[i] = '.';
            }
        }
        cout << s << '\n';
        return;
    }

    int last_used = -3;
    rep(i, n) {
        if(s[i] == 'o') {
            p[i+1] = p[i]+1;
            last_used=i;
        } else if(s[i] == '.') {
            p[i+1] = p[i];
        } else {
            if( last_used + 1 != i) {
                p[i+1] = p[i]+1;
                last_used = i;
            } else {
                p[i+1] = p[i];
            }
        }
    }

    last_used = n+10;
    per(i,n) {
        if(s[i] == 'o') {
            r[i] = r[i+1]+1;
            last_used = i;
        }  else if(s[i] == '.') {
            r[i] = r[i+1];
        } else {
            if(last_used-1 != i) {
                r[i] = r[i+1] + 1;
                last_used = i;
            } else {
                r[i] = r[i+1];
            }
        }
    }

    // rep(i, n+1) {
    //     cout << p[i] << ' ';
    // }
    // cout << '\n';
    
    // rep(i, n+1) {
    //     cout << r[i] << ' ';
    // }
    // cout << '\n';

    string ans = s;


    rep(i,n) {
        if(ans[i] == '?') {
            // oを置く
            int sum = 0;
            if(i>0) {
                sum += p[i-1];
            }
            if(i+2<=n) {
                sum += r[i+2];
            }
            if(sum+1<k) {
                // o配置不可
                ans[i] = '.';
                continue;
            }

            // .を置く
            sum = 0;
            sum += p[i];
            sum += r[i+1];
            if(sum<k) {
                ans[i] = 'o'; 
            }
        }
    }
    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


