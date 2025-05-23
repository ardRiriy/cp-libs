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
    
    vll ans(2,0);
    ll sum = 0;
    int cur = 0;

    deque<ll> dq;

    rep(_, n) {
        int s; cin >> s;
        if(s==1) {
            ll vi;
            if(cur==0) {
                vi = dq.back(); dq.pop_back();
                dq.push_front(vi);
            } else {
                vi = dq.front(); dq.pop_front();
                dq.push_back(vi);
            }

            ans[cur] += sum;
            ans[cur] -= vi*dq.size();
            
            ans[1-cur] -= sum;
            ans[1-cur] += vi*dq.size();
        } else if(s==2) {
            cur = 1-cur;
        } else {
            ll x; cin >> x;
            if(cur==0) {
                dq.push_back(x);
            } else {
                dq.push_front(x);
            }
            ans[cur] += x * dq.size();
            ans[1-cur] += sum + x;
            sum += x;
        }
        cout << ans[cur] << '\n';
    }
}


int main() {
    int t;
    cin >> t;
    while(t--)solve();
}