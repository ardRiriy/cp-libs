#include <bits/stdc++.h>
#include "input.hpp"
#include "persistent_array.hpp"

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

#define rep(i,n) for (ll i=0;i<n;++i)
#define per(i,n) for(ll i=n-1;i>=0;--i)
#define rep2(i,a,n) for (ll i=a;i<n;++i)
#define per2(i,a,n) for (ll i=n-1;i>=a;--i)


template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};

vll solve(int q, vector<pair<string, int>>& queries) {
    cerr << "solve output start\n\n";

    vll ans(q, -1);
    string s;
    int val;

    map<int, pair<int,int>> mp;

    int size = 0;
    PersistentArray<int> v;
    rep(i,q) {
        auto [s, val] = queries[i];
        if(s=="ADD") {
            v.set(size++, val);
        } else if(s=="DELETE") {
            if(size>0) size--;
            v.set(size, 1<<30);
        } else if(s=="SAVE") {
            if(v.parents.size()>0) mp[val] = {v.parents.size()-1, size}; // このディビジョンを書き込む
        } else {
            if(mp.find(val)==mp.end()) {
                size = 0;
                v.set(0, 1<<30);
            } else {
                size = mp[val].second;
                v.parents.push_back(v.parents[mp[val].first]);
            }
        }
        ans[i] = ((size>0)?v.get(size-1, v.parents.size()-1):-1);
    }
    return ans;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    int q; cin >> q;
    vector<pair<string, int>> queries(q);
    rep(i,q) {
        string s;
        cin >> s;
        int val;
        if(s!="DELETE") cin >> val;
        queries[i] = {s, val};
    }
    auto res1 = solve(q, queries);
    rep(i, q) cout << res1[i] << ((i+1==q)?'\n':' ');
}


