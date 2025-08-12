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

vector<int> input(int n) {
    vector<int> res;

    rep(i,n) {
        string s1, s2; cin >> s1 >> s2;
        int a = 0;
        if(s1[0]=='#') a = a | 1;
        if(s1[1]=='#') a = a | (1<<1);   
        if(s2[0]=='#') a = a | (1<<2);
        if(s2[1]=='#') a = a | (1<<3);
        res.emplace_back(a);
    }
    
    return res;
}

int left(int k) {
    if((k&1)==1 || ((k>>2)&1)==1) return -1;
    int res = 0;
    if(((k>>1)&1)==1) res = res | 1;
    if(((k>>3)&1)==1) res = res | (1<<2);
    return res;
}

int right(int k) {
    if(((k>>1)&1)==1 || ((k>>3)&1)==1) return -1;
    int res = 0;
    if(((k>>0)&1)==1) res = res | (1<<1);
    if(((k>>2)&1)==1) res = res | (1<<3);
    return res;
}

int up(int k) {
    if(((k>>0)&1)==1 || ((k>>1)&1)==1) return -1;
    int res = 0;
    if(((k>>2)&1)==1) res = res | (1<<0);
    if(((k>>3)&1)==1) res = res | (1<<1);
    return res; 
}

int down(int k) {
    if(((k>>2)&1)==1 || ((k>>3)&1)==1) return -1;
    int res = 0;
    if(((k>>0)&1)==1) res = res | (1<<2);
    if(((k>>1)&1)==1) res = res | (1<<3);
    return res;
}

// その層で干渉しない true / false
// k1: 下段
bool check(vector<int>& s, int i, int k1, int k2) {
    return ((s[i] | k1) == (s[i] ^ k1)) && ((s[i+1] | k2) == (s[i+1] ^ k2));
}

void merge(vector<int>& s, int i, int k1, int k2) {
    // assert(check(s,i,k1,k2));
    s[i] = s[i] | k1;
    s[i+1] = s[i+1] | k2;
}

int solve(){
    int h, n; cin >> h >> n;
    if(h==0) return 1;

    vector<int> s = input(h);
    vector<int> pl = input(2*n);
    dbg(s.size());
    dbg(pl.size());
    int base = s.size() + pl.size();

    rep(i,n) {
        if(pl[i*2]==0) swap(pl[i*2], pl[i*2+1]);
    }

    dbg(pl);
    
    // 空の段を2つ持っておく
    s.emplace_back(0);
    s.emplace_back(0);

    auto dfs = [&](auto self, int i, vector<int> state) -> int {
        if(i==pl.size()) return 0;
        int res = -1;
        for(auto x: {-1, 0, 1}) for(auto y: {-1, 0, 1}) {
            int k1 = (x==0) ? pl[i] : ((x==-1) ? left(pl[i]) : right(pl[i]));
            k1 = (y==0) ? k1 : ((y==-1) ? up(k1) : down(k1));

            int k2 = (x==0) ? pl[i+1] : ((x==-1) ? left(pl[i+1]) : right(pl[i+1]));
            k2 = (y==0) ? k2 : ((y==-1) ? up(k2) : down(k2));
            if(k1 == -1 || k2 == -1) continue;
            int cur = state.size()-1;
            
            while(cur>0 && check(state, cur-1, k1, k2)) cur--;

            // curにmerge
            auto nxt_state = state;
            dbg(i,x,y);
            dbg(k1,k2);
            merge(nxt_state, cur, k1, k2);

            int cnt = 0;
            if(nxt_state[cur+1]==15) nxt_state.erase(nxt_state.begin()+(cur+1)), cnt++;
            if(nxt_state[cur]==15) nxt_state.erase(nxt_state.begin()+cur), cnt++;

            while(nxt_state.size()<2 || !(nxt_state[nxt_state.size()-1] == 0 && nxt_state[nxt_state.size()-2] == 0)) nxt_state.emplace_back(0);
            
            // for(auto val: nxt_state) cerr << bitset<4>(val) << '\n';
            // cerr << "\n===============\n";

            chmax(res, self(self, i+2, nxt_state) + cnt);
        }
        return res;
    };

    int ans = dfs(dfs, 0, s);
    cout << ans << '\n';
    return 0;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}


