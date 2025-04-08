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

ll check(vll& a, ll x) {
    ll n = a.size();
    ll cnt = 0;

    set<ll> st;
    for(auto ai: a) {
        if(ai < x) {
            st.insert(ai);
            if(st.size()==x) {
                cnt++;
                st.clear();
            }
        }
    }
    
    return cnt;
}

void solve() {
    int n, k; cin >> n >> k;
    vll a(n); rep(i,n)cin>>a[i];
    ll ok = 0;
    ll ng = n+1;

    while(abs(ok-ng)>1){
        ll mid = (ok+ng)/2;
        ll res = check(a, mid);
        if(res>=k){
            ok=mid;
        } else {
            ng=mid;
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