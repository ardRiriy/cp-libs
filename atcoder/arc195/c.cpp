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
    int r, b; cin >> r >> b;

    vector<tuple<char, int, int>> ans;

    if(b==0) {   
        if(r%2==1) {
            // checked
            cout << "No\n";
            return;
        }

        //checked
        int t = r / 2;
        // (1, 0)からtだけ下へ
        rep(i, t) {
            ans.push_back({'R', 1, -i});
        }
        per(i, t) {
            ans.push_back({'R', 0, -i});
        }
    } else if(b==1) {
        if(r%2==1) {
            // checked
            cout << "No\n";
            return;
        }
        // checked
        ans.push_back({'B', 0, 0});
        int tr = r / 2;
        rep(i, tr) {
            ans.push_back({'R', 1, -i-1});
        }
        per(i, tr) {
            ans.push_back({'R', 0, -i-1});
        }

    } else if(b%2==0) {
        int t = b / 2;
        rep(i, t) {
            ans.push_back({'B', -i, i});
        }
        per(i, t) {
            ans.push_back({'B', -i+1, i+1});
        }
        if(r%2==0) {
            //(2, 0からスタート)
            // 多分OK
            int tr = r / 2;
            rep(i, tr) {
                ans.push_back({'R', 2, -i});
            }
            per(i, tr) {
                ans.push_back({'R', 1, -i});
            }
        } else {
            // 何をみてOKっていったんですか？？？？
            cout << "No\n";
            return;
            // 多分OK
            // ans.push_back({'R', 0, 1});
            // int tr = (r-1) / 2;
            // rep(i, tr) {
            //     ans.push_back({'R', 1, -i-1});
            // }
            // per(i, tr) {
            //     ans.push_back({'R', 0, -i-1});
            // }
        }
    } else {
        // bが奇数(一般)
        int tb = (b-1)/2;
        rep(i, tb) {
            ans.push_back({'B', -i, i});
        }
        if(r%2==0 && r>=2) {
            // checked
            per(i, tb) {
                ans.push_back({'B', -i+1, i+1});
            }
            ans.push_back({'B', 2, 0});
            
            int tr = r/2;
            rep(i, tr) {
                ans.push_back({'R', 1, -i-1});
            }
            per(i, tr) {
                ans.push_back({'R', 0, -i-1});
            }
        } else {
            cout << "No\n";
            return;
            // 以下はDead Code
            // バグっているのは俺の頭の方
            // ans.push_back({'R', -tb, tb});
            // per(i, tb) {
            //     ans.push_back({'B', -i, i+1});
            // }
            // int tr = (r-1)/2;
            // rep(i, tr) {
            //     ans.push_back({'R', 2, -i-1});
            // }
            // per(i, tr) {
            //     ans.push_back({'R', 1, -i-1});
            // }
        }
    }

    int base = 10000000;
    base = 0;
    cout << "Yes\n";
    for(auto[c, x, y]: ans) {
        cout << c << " " << x + base << " " << y + base << " " << "\n";
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


