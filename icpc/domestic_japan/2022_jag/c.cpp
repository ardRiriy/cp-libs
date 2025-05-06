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
using djks=priority_queue<P, vp, greater<P>>;

const ll inf=1ll<<60;
#define mod10 (ll)1e9+7
#define mod99 (ll)998244353
const double PI = acos(-1);

#define rep(i,n) for (ll i=0;i<(n);++i)
#define per(i,n) for(ll i=(n)-1;i>=0;--i)
#define rep2(i,a,n) for (ll i=(a);i<(n);++i)
#define per2(i,a,n) for (ll i=(n)-1;i>=(a);--i)


template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};

void solve(ll n) {
    vector<string> v(n);
    rep(i,n)cin>>v[i];
    ll ans = 0;
    rep(i,n) {
        ll mn = 0;
        rep2(a, 1, v[i].size()-1){
            // 前からa文字残すとき, 後から何文字消す必要があるか？
            ll mx = 0;
            rep(j,n){
                if(i==j || v[i].size() != v[j].size()) continue;
                // prefixが一致するか
                bool is_same_prefix = true;
                rep(k, min(a, ll(v[j].size()))) {
                    is_same_prefix = is_same_prefix && v[i][k] == v[j][k];
                }

                if(is_same_prefix){
                    bool checked = false;
                    rep(k, min(v[i].size(), v[j].size())) {
                        if(v[i][v[i].size()-k-1] != v[j][v[j].size()-k-1]) {
                            chmax(mx,k);
                            checked = true;
                            break;
                        }
                    }
                    if(!checked) {
                        mx = min(v[i].size(), v[j].size());
                        break;
                    }
                }
            }
            chmax(mn, max(ll(v[i].size())-(a+mx)-1,0LL));
        }
        ans += mn;
    }

    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int k;cin>>k;
    while(k>0){
        solve(k);
        cin>>k;
    }
}
