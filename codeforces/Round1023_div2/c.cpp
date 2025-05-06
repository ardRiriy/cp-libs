#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;

// using namespace atcoder;

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
    ll n, k; cin >> n >> k;
    string s; cin >> s;
    auto a = i64_vec_IN(n);

    {
        bool isAllOne=true;
        rep(i,n) isAllOne = isAllOne&&s[i]=='1';

        if(isAllOne){
            ll cs=0;
            ll maxCs=-inf;
            rep(i,n){
                cs+=a[i];
                if(cs<0)cs=0;
                chmax(maxCs,cs);
            }
            if(maxCs==k){
                cout<<"Yes\n";
                rep(i,n) {
                    cout<<a[i]<<((i+1==n)?'\n':' ');
                }
            } else {
                cout << "No\n";
            }
            return;
        }
    }

    ll cs = 0;
    rep(i,n){
        if(s[i]=='0') {
            cs=0;
        } else {
            cs+=a[i];
            cs=max(cs,0ll);
        }
        if(cs>k){
            cout<<"No\n";
            return;
        }
    }

    cout << "Yes\n";
    bool isMade=false;

    rep(i,n){
        if(s[i]=='1') continue;
        if(isMade) {
            a[i] = -1e16;
            continue;
        }

        // 左
        ll leftCs=0;
        ll leftMaxCs=0;
        per2(j,0,i){
            leftCs+=a[j];
            chmax(leftMaxCs,leftCs);
        }

        // 右
        ll rightCs=0;
        ll rightMaxCs=0;
        rep2(j,i+1,n){
            if(s[j]=='0') break;
            rightCs+=a[j];
            chmax(rightMaxCs,rightCs);
        }
        a[i]=k-(leftMaxCs+rightMaxCs);
        isMade=true;
    }
    rep(i,n)cout << a[i] << ((i+1==n)?'\n':' ');
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


