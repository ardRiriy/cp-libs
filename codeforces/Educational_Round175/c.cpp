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
    int n, k; cin >> n >> k;
    string s; cin >> s;
    int a[n]; rep(i, n) cin >> a[i];

    int ok = 1e9+2;
    int ng = -1;
    while (ok-ng>1){
        int mid = (ok+ng)>>1;
        // ペナルティmidで構成可能か？

        int cnt = 0;
        bool flag = false; // 直前に青に変えたか
        bool judge = true;
        rep(i, n) {
            if(flag) {
                // 赤かつpenalty > midならかえてはいけない 
                if(s[i]=='R' && a[i] > mid) {
                    flag = false;
                }
            } else {
                // 青かつpenalty > midならcnt+1で変える
                if(s[i]=='B'&& a[i]>mid) {
                    cnt += 1;
                    if(cnt>k) judge = false;
                    flag = true;
                }
            }
        } 
        if(judge) ok = mid;
        else ng = mid;
    }
    
    cout << ok << '\n';
}


int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--)solve();
}
