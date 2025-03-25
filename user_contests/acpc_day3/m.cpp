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

vector<int> ans;
void rec(ll k, ll cur_i, vector<int> cur) {
    bool flag = false;

    while(k%cur_i==0) {
        if(flag) cur.push_back(0);
        else cur.push_back(1);

        flag = true;
        k /= cur_i;
        if(k==1) {
            if(ans.empty() || ans.size() > cur.size() ||cur.size()==ans.size() && cur < ans) {
                ans = cur;
                return;
            }
        }
        // 次に行く
        rec(k, cur_i+1, cur);

        if(cur_i==1) break;
    }
}

void solve() {
    ll k; cin >> k;
    if(k==0) {
        cout << 0 << '\n';
        return;
    }
    vector<int>v;
    rec(k, 1, v);
    if(ans.empty()) cout << "-1\n";
    else rep(i, ans.size()) cout << ans[i] << ((i+1==ans.size()) ? '\n' : ' ');

}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


