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

int m = 5;
int times[5];

void solve() {
    ll n, c; cin >> n >> c;
    string s; cin >> s;
    rep(i, m) times[i] = 0;
    rep(i, n) times[s[i] - 'A'] += 1;

    ll ans = inf;
    rep(i, 1<<m) {
        ll sum = 0;
        ll sum2 = 0;
        rep(j, m) {
            if(times[j]==0) continue;
            if ((i>>j)&1==1) sum2 += times[j];
            else sum += c;
        }
        chmin(ans, sum+(sum2 * (sum2+1)/2));
    }
    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t; cin >> t;
    while(t--) solve();
}