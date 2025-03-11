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
    queue<tuple<int, int, int>> que;
    cout << "? 1 2 3" << endl;
    int res; cin >> res;
    if(res == 0) {
        cout << "! 1 2 3" << endl;
        return;
    } 
    que.push({1, 2, res});
    que.push({1, 3, res});
    que.push({3, 2, res});
    while(!que.empty()) {
        auto [x, y, z] = que.front(); que.pop();
        cout << "? " << x << " " << y << " " << z << endl;
        int res; cin >> res;
        if(res == -1) exit(0);
        else if(res == 0) {
            cout << "! " << x << " " << y << " " << z << endl;
            return;
        }
        que.push({y, z, res});
    }
}

int main() {
    int t;
    cin >> t;
    while(t--)solve();
}