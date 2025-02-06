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
    vector<string> s(2); cin >> s[0] >> s[1];
    set<int> diff_with_unblocked;

    rep(i, s[0].size()) {
        if (s[0][i] != s[1][i]) {
            diff_with_unblocked.insert(i);
        }
    }

    int ti, q; cin >> ti >> q;
    vll unblock_time(q, -1);
    rep(qi, q) {
        if(unblock_time[qi] != -1) {
            int i = unblock_time[qi];
            if(s[0][i]!=s[1][i]) diff_with_unblocked.insert(i);
        }

        int type; cin >> type;
        if(type == 1) {
            int i; cin >> i; i--;
            diff_with_unblocked.erase(i);
            if(qi+ti<q) {
                unblock_time[qi+ti] = i;
            }

        } else if(type == 2) {
            int x, y;
            int posx, posy;
            cin >> x >> posx >> y >> posy;
            x--; posx--; y--; posy--;
            swap(s[x][posx], s[y][posy]);
            if(s[0][posx] == s[1][posx]) diff_with_unblocked.erase(posx);
            else diff_with_unblocked.insert(posx);
            if(s[0][posy] == s[1][posy]) diff_with_unblocked.erase(posy);
            else diff_with_unblocked.insert(posy);
        } else {
            cout << (diff_with_unblocked.empty() ? "YES" : "NO") << '\n';
        }
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--)solve();
}