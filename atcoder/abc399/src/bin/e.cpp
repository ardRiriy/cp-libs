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
    int n; cin >> n;
    string s, t;
    cin >> s >> t;
    vector<char> targets(26, '-');
    rep(i, n) {
        char ss = s[i];
        char tt = t[i];
        if(targets[ss-'a'] == '-') {
            targets[ss-'a'] = t[i];
        } else {
            if(targets[ss-'a'] != t[i]) {
                cout << -1 << '\n';
                return;
            }
        }
    }

    dsu uf(26);
    rep(i, 26) {
        if(targets[i] == '-') continue;
        uf.merge(i, targets[i]-'a');
    }

    vll cnt_edges(26, 0);
    rep(i, 26) {
        if(targets[i] == '-') continue;
        if(targets[i] == i + 'a') continue;
        cnt_edges[uf.leader(i)]++;
    }
    vll cycles_size;
    ll ans = 0;

    vector<bool> seen(26, false);
    rep(i, 26) {
        if(seen[uf.leader(i)]) continue;
        seen[uf.leader(i)] = true;
        if(cnt_edges[uf.leader(i)] == uf.size(i)) {
            // サイクル
            cycles_size.push_back(uf.size(i));
        } else {
            // サイクルではない
            ans += uf.size(i) - 1;
        }
    } 
    ll sum = 0;
    for(auto c : cycles_size) {
        ans += c + 1;
        sum += c;
    }

    bool falg = false;
    set<int> seen_targets;
    rep(i, 26) {
        if(targets[i] == '-') continue;
        if(seen_targets.count(targets[i]-'a')) {
            falg = true;
            break;
        }
        seen_targets.insert(targets[i]-'a');
    }
    if(sum == 26 && !falg) {
        cout << "-1\n";
    } else {
        cout << ans << '\n';
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


