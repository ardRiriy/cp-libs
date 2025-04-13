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

ll dfs(int p, vvll& g, vector<bool>& seen, vll& a) {
    seen[p] = true;
    ll res = a[p];
    for(auto ni: g[p]) {
        if(seen[ni]) continue;
        res += dfs(ni, g, seen, a);
    }
    return res;
}

void solve() {
    int n, q; cin >> n >> q;
    vll a(n); rep(i,n) cin >> a[i];
    vvll g(n); 
    rep(i,n-1) {
        int u, v; cin >> u >> v;
        u--; v--;
        g[u].push_back(v);
        g[v].push_back(u);
    }

    ll k; cin >> k;

    // 部分木ごとのsumを計算
    // O(N)で可能
    vll subtree_sum;
    vector<bool> seen(n, false);
    seen[0] = true;

    for(auto ni: g[0]) {
        subtree_sum.push_back(dfs(ni, g, seen, a));
    }
    
    // a_0を全部渡したときにクリアできるか
    ll sum = 0;
    for(auto x: subtree_sum) {
        sum += x;
    }

    ll subs = sum+a[0];
    cerr <<"(subs/subtree_sum.size()+((subs%subtree_sum.size()==0)?0:1))=" << (subs/subtree_sum.size()+((subs%subtree_sum.size()==0)?0:1)) << '\n';
    if((subs/subtree_sum.size()+((subs%subtree_sum.size()==0)?0:1)) > k) {
        cout << "-1\n";
        return;
    }
     
    // a_0の範囲は？
    // : 0 <= a_0 <= kでないとだめですよね
    // a_0 = x (0 <= x <= k)として考えたときに、max(subtreeごとのsum)<=k-x であればOK
    // a_0に集めるのは基本的に悪手ですよね？手数が変わらない or 増えるのどちらかなので。
    // subtree側のmaxを1増やす -> a_0がsubtreeの個数分だけ減る
    // として見れるのか。
    
    // ということは、a_0はもしかして0にしたほうが嬉しいですか？
    // そんな気がしますね。
    // 判定はそれで良いように思えるけど、事前操作数の最小が難しい。
    // 操作数、もしかして単調性がある？二分探索が可能？
    // できそう、やるか

    // 誤読してました、できません 終わりです

    ll ng = -1;
    ll ok = a[0];

    while(abs(ok-ng)>1){
        ll mid = (ok+ng)/2;
        ll subs = sum+mid;
        ll base = subs/subtree_sum.size()+(subs%subtree_sum.size()==0)?0:1;
        ll addr = a[0]-mid;
        if(base+addr<=k) {
            ok=mid;
        }else{
            ng=mid;
        }
    }

    cerr << "ok=" << ok << '\n';
    ll kd = k - (a[0]-ok);
    ll ans = ok;
    for(auto x: subtree_sum) {
        ans += max(x-kd, 0ll);
    }
    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--) solve();
}