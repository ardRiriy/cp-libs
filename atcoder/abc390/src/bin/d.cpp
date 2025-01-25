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

unordered_set<ll> st;
ll a[12];
int n;
void dfs(int i, vector<ll>& v) {
    if (i == n) {
        ll res = 0;
        rep(j, v.size()) {
            res ^= v[j];
        } 
        st.insert(res);
        return;
    }

    rep(j, v.size()) {
        v[j] += a[i];
        dfs(i+1, v);
        v[j] -= a[i];
    }
    v.push_back(a[i]);
    dfs(i+1, v);
    v.pop_back();
    return;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);

    cin >> n;
    rep(i, n) cin >> a[i];
    vector<ll> v;
    dfs(0, v);

    cout << st.size() << '\n';
    return 0;
}
