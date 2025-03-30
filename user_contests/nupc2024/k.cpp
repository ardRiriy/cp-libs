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
    ll n; cin >> n;
    cout << "? 1 " << n << endl;
    ll res = 0; cin >> res;

    ll dist = n-res;
    ll ok = 1;
    ll ng = n;

    while(abs(ok-ng)>1) {
        ll mid = (ok+ng)/2;
        cout << "? 1 " << mid << endl;
        cin >> res;

        if(mid-1==res) ok = mid;
        else ng = mid;
    }

    if(dist%2==0) {
        cout << "! " << ok-dist/2 << " " << ok+dist/2 << endl; 
    } else {
        cout << "! " << ok-dist/2-1 << " " << ok + dist/2 << endl;
    }
}

int main() {
    int t=1;
    cin >> t;
    while(t--)solve();
}


// H~L