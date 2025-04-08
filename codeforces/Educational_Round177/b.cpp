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
    ll n, k, x;cin>>n>>k>>x;

    vll a(2*n);rep(i,n)cin>>a[i], a[n+i]=a[i];
    vll csum(2*n+1,0); rep(i,2*n)csum[i+1]=csum[i]+a[i];
    ll sum = csum[n] - csum[0];
    vll r(n);
    rep(i,n){
        ll full = 0;
        ll left = x;
        if(x>=sum){
            full = x/sum;
            left = x%sum;    
        }
        
        ll ok = 2*n+1;
        ll ng = i-1;
        while(abs(ok-ng)>1){
            ll mid = (ok+ng)/2;
            if(csum[mid]-csum[i]>=left) ok = mid;
            else ng = mid;
        }

        r[i]=full*n+ok-1;
    }
    ll ans = 0;
    rep(i,n){
        if(r[i]>=n*k) continue;
        ll t = r[i]/n;
        ans+=(k-t);
    }
    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--)solve();
}
