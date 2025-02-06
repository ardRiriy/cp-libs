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

int lb(ll key, vll &a) {
    int ng = -1;
    int ok = a.size();
    while(ok-ng>1){
        int mid = (ng+ok)>>1;
        if(a[mid] >= key) ok = mid;
        else ng = mid;
    }
    return ok;
}

bool judge(ll t, vll& a) {
    ll x1 = a[0]+t;

    int i = lb(x1+t+1, a);
    if (i == a.size()) {
        return true;
    }

    ll x2 = a[i]+t;
    int j = lb(x2+t+1, a);
    if (j == a.size()) {
        return true;
    }

    ll x3 = a[j]+t;
    int k = lb(x3+t+1, a);
    return k == a.size();
}

void solve() {
    int n; cin >> n;
    vll a(n); rep(i, n) cin >> a[i];
    sort(all(a));

    ll ng = -1;
    ll ok = 2e9;
    while(ok-ng>1){
        ll mid = (ok+ng)>>1;
        if(judge(mid, a)) {
            ok = mid;
        } else {
            ng = mid;
        }
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