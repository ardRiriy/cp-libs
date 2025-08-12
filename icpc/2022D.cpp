#include <bits/stdc++.h>
#include "input.hpp"

#include "atcoder/modint.hpp"

using namespace std;

using namespace atcoder;

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

using mint = modint998244353;

vector<mint> fact(20000, 1);
mint nck(int n, int k) {
    if(n<k) return 0;
    return fact[n] / fact[n-k] / fact[k];
}


unordered_map<int, int> mp;
vector<int> s, t, q;
void solve(int n, int k) {
    dbg("===");
    int a;
    
    mp.clear();
    s.clear();
    rep(i,n) {
        cin >> a;
        s.emplace_back(a);
    }

    t.clear();
    rep(i,n) {
        cin >> a;
        t.emplace_back(a);
    }

    rep(i,n) {
        mp[t[i]] = i;
    }

    q.clear();
    rep(i,n){
        q.emplace_back(mp[s[i]]);
    }
    dbg(q);

    vll pos;
    rep(i,n-1) {
        if(q[i] > q[i+1]) {
            pos.emplace_back(i+1);
        }
    }
    pos.emplace_back(n);

    {
        // check
        dbg(pos.size());
        vll indicates(pos.size(),0);
        rep(i,pos.size()-1){
            indicates[i+1] = pos[i];
        }
        
        djks pq;
        rep(i,indicates.size()) {
            pq.push({s[indicates[i]], i});
        }

        ll cur = 0;
        while(!pq.empty()) {
            dbg(pq);
            auto [val, idx] = pq.top();
            pq.pop();
            if(t[cur]==val) cur++;
            else break;

            if(indicates[idx]+1<pos[idx]) {
                indicates[idx]++;
                pq.push({s[indicates[idx]], idx});
            }
        }
        if(cur!=n) {
            cout << "0\n";
            return;
        }
    }

    ll cnt = 0;

    
    if(pos.size()>k){
        cout << "0\n";
        return;
    }
    k -= pos.size();

    ll l = 0;
    for(auto r: pos) {
        ll maximam = s[l];
        rep2(i,l+1,r) {
            if(maximam<s[i]) cnt++;
            chmax(maximam, ll(s[i]));
        }
        l = r;
    }
    
    mint ans = 0;
    rep(i, k+1) {
        ans += nck(cnt, i);
    }
    cout << ans.val() << '\n';

}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);

    rep(i,20000-1) {
        fact[i+1] = fact[i] * (i+1);
    }

    int n, k;
    cin >> n >> k;
    while(n) {
        solve(n, k);
        cin >> n >> k;
    }
}
