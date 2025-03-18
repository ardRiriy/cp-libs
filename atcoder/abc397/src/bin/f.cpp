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

using S = ll;
using F = ll;
S op(S a, S b) { return max(a, b); }
S e() { return -inf; }
S mapping(F f, S x) { return f + x; }
F composition(F f, F g) { return f+g; }
F id() { return 0; }

void solve() {
    int n; cin >> n;
    vll a(n); rep(i, n) cin >> a[i];
    
    ll unique_r = 0, unique_l = 0;
    vector<int> cnt(300001, 0);
    for(ll ai: a) if(cnt[ai]++==0)unique_r++;

    lazy_segtree<S, op, e, F, mapping, composition, id> seg(vll(n, 0));
    vll last_find(300001, -1);

    ll ans = 0;
    rep(i, n-1){
        seg.set(i, unique_l); // 左からのunique数
        seg.apply(last_find[a[i]]+1, i+1, 1);
        if(last_find[a[i]] == -1) unique_l++;
        if(--cnt[a[i]]==0) unique_r--;
        ll val = unique_r + seg.all_prod();
        chmax(ans, val);

        last_find[a[i]] = i;
    }
    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


