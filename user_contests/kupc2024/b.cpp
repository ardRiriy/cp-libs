
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
    ll a, b;
    cin >> a >> b;
    bool isSwap = false;
    if (a > b) {
        swap(a, b);
        isSwap = true;
    }

    if(4*a < b) {
        cout << "-1\n";
        return;
    }
    
    vector<vector<char>> bd = {
        {'.', '@', '.', '.', '.'},
        {'.', '.', '.', '@', '.'},
        {'@', '.', '.', '.', '.'},
        {'.', '.', '@', '.', '.'},
        {'.', '.', '.', '.', '@'},
    };

    a *= 5;
    b *= 5;
    ll times = (a + b) / 5;
    vector<vector<char>> ans;
    ll cnt_w = 0;
    rep(i, times) {
        ans.push_back(bd[i%5]);
        cnt_w += 1;
    }

    ll left = a - cnt_w;
    rep(i, ans.size()) rep(j, 5) {
        if(left > 0 && ans[i][j] == '.' && ans[(i+ans.size()-1)%ans.size()][j] != '@') {
            ans[i][j] = '@';
            left --;
        } 
    }

    if(!isSwap) {
        rep(i, ans.size()) {
            rep(j, 5) {
                ans[i][j] = (ans[i][j] == '.') ? '@' : '.';
            }
        }
    }

    cout << ans.size() << " 5\n";
    rep(i, ans.size()) {
        rep(j, 5) {
            cout << ans[i][j];
        }
        cout << "\n";
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    
    while(t--) solve();
}
