#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;

// using namespace atcoder;

#ifdef ADRY
#include <dbg.h>
#else
// DO NOTHING
#define dbg(x)
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

const int inf=1ll<<30;
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


int solve(){
    string s1,s2;
    cin >> s1;
    if(s1==".") return 1;
    cin >> s2;

    vector<string> s1v, s2v;
    string cur;
    for(auto c: s1) {
        if(c == '"') {
            s1v.push_back(cur);
            cur = "";
        } else {
            cur += c;
        }
    }
    s1v.push_back(cur);
    cur = "";
    for(auto c: s2) {
        if(c == '"') {
            s2v.push_back(cur);
            cur = "";
        } else {
            cur += c;
        }
    }
    s2v.push_back(cur);

    if(s1v.size() != s2v.size()) {
        cout << "DIFFERENT\n";
        return 0;
    }

    int cnt = 0;
    rep(i,s1v.size()) {
        if(i%2==0 && s1v[i] != s2v[i]) {
            cout << "DIFFERENT\n";
            return 0;
        }
        if(i%2==1 && s1v[i] != s2v[i]) {
            cnt++;
        }
    }
    if(cnt==0) {
        cout << "IDENTICAL\n";
    } else if(cnt==1) {
        cout << "CLOSE\n";
    } else {
        cout << "DIFFERENT\n";
    }
    return 0;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}


