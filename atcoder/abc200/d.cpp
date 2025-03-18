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
    vll a(n); rep(i, n) cin >> a[i];

    vector<bitset<200>> v(200, bitset<200>(0ull));
    
    queue<bitset<200>> que;
    queue<ll> sum;
    que.emplace(bitset<200>(0));
    sum.push(0);

    while(!que.empty()) {
        auto q = que.front(); que.pop();
        ll s = sum.front(); sum.pop();
        per(i, n) {
            if(q.test(i)) break;
            auto p = q;
            p.flip(i);
            ll t = (s+a[i])%200;

            if(v[t].none()){
                v[t] = p;
                que.push(p);
                sum.push(t);
            } else {
                cout << "Yes\n";
                cout << p.count();
                rep(i, n) if(p.test(i)) cout << ' ' << i+1;
                cout << '\n';
                
                cout << v[t].count();
                rep(i, n) if(v[t].test(i)) cout << ' ' << i+1;
                cout << '\n';
                return; 
            }
        }
    }
    cout << "No\n";
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


