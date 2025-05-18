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

string s;
vector<int> o_positions;

bool dfs(int idx, int left, int last, bitset<101>& bs) {
    vector<int> updated; 

    if(idx==0) {
        per(j, s.size()-left) {
            if(!bs[j] || bs[left+j]) continue;
            bs[left+j] = true;
            updated.push_back(left+j);
        }
        
        bool res = true;
        for(auto i: o_positions) res = res && bs[i];

        for(auto pos : updated) {
            bs[pos] = false;
        }

        return res;
    }

    bool res = false;

    rep2(i,last,left+1) {
        if(left-i<i) break;
        per(j, s.size() - i) {
            if(!bs[j] || bs[i+j]) continue;
            bs[i+j] = true;
            updated.push_back(i+j);
        }

        res = dfs(idx-1, left-i, i, bs);
        if(res) break;
        
        for(auto pos : updated) {
            bs[pos] = false;
        }
        updated.clear();
    }
    return res;
}

void solve(int n) {
    cin >> s;
    o_positions.clear();
    rep(i, s.size()) if(s[i]=='o') o_positions.push_back(i);

    bitset<101> bs;
    bs[0] = true;
    rep(i,6){
        auto ret = dfs(i, n, 1, bs);
        if(ret) {
            cout << i+1 << '\n';
            return;
        }
    }
    cout << "7\n";
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int n;
    cin >> n;
    while(n) {
        solve(n);
        cin >> n;
    }
}


