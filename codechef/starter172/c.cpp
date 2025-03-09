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
    int n; cin >> n;
    int a[n]; rep(i, n) cin >> a[i];
    set<int> st;
    rep2(i, 1, n) {
        if ((i%2==1&&a[i]==0) || (i%2==0&&a[i]==1)) {
            st.insert(i);
        }
    }

    int q; cin >> q;
    rep(_, q) {
        int p; cin >> p; p--;
        st.erase(p);
        cin >> a[p];
        if ((p%2==1&&a[p]==0) || (p%2==0&&a[p]==1)) st.insert(p);
        cout << (st.empty() ? a[0] : a[*st.rbegin()]) << '\n';
    }

}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--) solve();
}