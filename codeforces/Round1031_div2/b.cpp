#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;
// using namespace atcoder;

#ifdef ADRY
#include <dbg.h>
#else
// DO NOTHING
#define dbg(...)
#endif

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

void solve() {
    ll w, h, a, b; cin >> w >> h >> a >> b;

    ll x1, y1, x2, y2; cin >> x1 >> y1 >> x2 >> y2;

    if(x1<=x2) {
        if(x1+a>x2) {
            ll my = min(y1,y2);
            ll d = max(y1,y2)-(my+b);
            dbg(my,d);
            if(d%b==0) {
                cout << "Yes\n";
            } else {
                cout << "No\n";
            }
            return;
        } else {
            dbg((max(x1,x2)-(min(x1,x2)+a)) % a);
            if((max(x1,x2)-(min(x1,x2)+a)) % a == 0) {
                cout << "Yes\n";
            } else {
                if(min(y1,y2)+b>max(y1,y2)) {
                    cout << "No\n";
                } else {
                    if((max(y1,y2)-(min(y1,y2)+b)) % b == 0) {
                        cout << "Yes\n";
                    } else {
                        cout << "No\n";
                    }
                }
            }
            return;
        }
    }

    dbg("here");
    swap(x1,x2);
    swap(y1,y2);
    if(x1<=x2) {
        if(x1+a>x2) {
            ll my = min(y1,y2);
            ll d = max(y1,y2)-(my+b);
            dbg(my,d);
            if(d%b==0) {
                cout << "Yes\n";
            } else {
                cout << "No\n";
            }
            return;
        } else {
            dbg((max(x1,x2)-(min(x1,x2)+a)) % a);
            if((max(x1,x2)-(min(x1,x2)+a)) % a == 0) {
                cout << "Yes\n";
            } else {
                if(min(y1,y2)+b>max(y1,y2)) {
                    cout << "No\n";
                } else {
                    if((max(y1,y2)-(min(y1,y2)+b)) % b == 0) {
                        cout << "Yes\n";
                    } else {
                        cout << "No\n";
                    }
                }
            }
            return;
        }
    }

}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


