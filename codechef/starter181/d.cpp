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
    int n;cin>>n;
    vll a(n-1); rep(i,n-1)cin>>a[i], a[i]--;

    vvll cnt(n);
    rep(i,n-1) {
        cnt[a[i]].push_back(i+1);
    }

    // 0は奇数 それ以外は偶数
    rep(i,n){
        // cerr << i << " " << cnt[i].size() << '\n';
        if(!((i==0&&cnt[i].size()%2==1) || (i>0&&cnt[i].size()%2==0))) {
            cout << "-1\n";
            return;
        }
    }

    vll ans(n, 0);
    ll cur = 1;
    rep(i,n) {
        if(i==0) {
            ans[0] = cur;
            ans[cnt[0][0]] = cur;
            cur++;

            for(int j=1; j<cnt[i].size(); j+=2) {
                ans[cnt[i][j]] = cur;
                ans[cnt[i][j+1]] = cur;
                cur++;
            }

        } else {
            for(int j=0; j<cnt[i].size(); j+=2) {
                ans[cnt[i][j]] = cur;
                ans[cnt[i][j+1]] = cur;
                cur++;
            }
        }
    }
    
    rep(i,n)cout << ans[i] << ((i+1==n)?"\n":" ");
}

int main() {
    // cin.tie(0);
    // ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--) solve();
}