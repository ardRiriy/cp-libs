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

void solve() {
    int n, m; cin >> n >> m;
    vector<string> a(n); rep(i,n)cin>>a[i];
    vector<vector<string>> b(m, vector<string>(n));
    rep(i,m){
        rep(j,n) {
            cin >> b[i][j];
        }
    }

    rep(j,n){
        bool flag = false;
        rep(i,m) {
            if(b[i][j] == a[j]) {
                flag = true;
                break;
            } 
        }
        if(!flag) {
            cout << "-1\n";
            return;
        }
    }


    // 一致数の最大化
    int mx = 0;
    rep(i,m){
        int cnt = 0;
        rep(j,n){
            if(a[j] == b[i][j]) cnt++;
        }
        chmax(mx, cnt);
    }
    cout << n + (n-mx)*2 << '\n';
} 

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--)solve();
}