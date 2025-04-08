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

// p: 2^level
// x, y: 0-indexed
// px, py: 左上の座標
ll cell_num(ll level, ll x, ll y, ll base) {
    if(level==0){
        //cerr << "return: " << base << '\n';
        return base;
    }

    ll p = 1LL<<level;
    ll half = p/2;
    ll k = 1LL<<(2*(level-1));
    ll nx = ((x<half)?x:x-half);
    ll ny = ((y<half)?y:y-half);

    if(x>=half&&y<half) {
        base += 3*k;
    } else if (x<half&&y>=half){
        base += 2*k;
    } else if(x>=half&&y>=half){
        base += k;
    }
    
    return cell_num(level-1, nx, ny, base);
}

pair<ll, ll> pos(ll level, ll x, ll y, ll left) {
    if(level==0){
        return {x,y};
    }
    ll p = 1LL<<level;
    ll half = p/2;
    ll k = 1LL<<(2*(level-1));

    if(left < k) {
        return pos(level-1, x, y, left);
    } else if(left < 2*k) {
        return pos(level-1, x+half, y+half, left-k);
    } else if(left < 3*k) {
        return pos(level-1, x, y+half, left-2*k);
    } else {
        return pos(level-1, x+half, y, left-3*k);

    }
}

void solve() {
    ll n, q; cin >> n >> q;
    rep(_, q) {
        string s;
        cin >> s;
        if(s=="->"){
            ll x, y; cin >> x >> y;
            x--;y--;
            ll ans = cell_num(n, y, x, 1);
            cout << ans << '\n';
        } else {
            ll d; cin >> d;
            auto [y, x] = pos(n, 0, 0, d-1);
            cout << x+1 << ' ' << y+1 << '\n';
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