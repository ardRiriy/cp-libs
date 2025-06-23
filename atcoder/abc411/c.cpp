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

// thanks: https://satanic0258.github.io/snippets/data-structure/SegmentMap.html
// Description: 区間をsetで管理するデータ構造(なお実装はmap)．各クエリO(log区間数)．
 
// #### attention! : [l, r] ( include r, not [l, r) )
class SegmentMap : public std::map<signed, signed> {
private:
    bool flagToMergeAdjacentSegment;
public:
    // if merge [l, c] and [c+1, r], set flagToMergeAdjacentSegment to true
    SegmentMap(bool flagToMergeAdjacentSegment) :
        flagToMergeAdjacentSegment(flagToMergeAdjacentSegment) {}
    // __exist -> iterator pair(l, r) (contain p)
    // noexist -> map.end()
    auto get(signed p) const {
        auto it = upper_bound(p);
        if (it == begin() || (--it)->second < p) return end();
        return it;
    }
    // insert segment [l, r]
    void insert(signed l, signed r) {
        auto itl = upper_bound(l), itr = upper_bound(r + flagToMergeAdjacentSegment);
        if (itl != begin()) {
            if ((--itl)->second < l - flagToMergeAdjacentSegment) ++itl;
        }
        if (itl != itr) {
            l = std::min(l, itl->first);
            r = std::max(r, std::prev(itr)->second);
            erase(itl, itr);
        }
        (*this)[l] = r;
    }
    // remove segment [l, r]
    void remove(signed l, signed r) {
        auto itl = upper_bound(l), itr = upper_bound(r);
        if (itl != begin()) {
            if ((--itl)->second < l) ++itl;
        }
        if (itl == itr) return;
        int tl = std::min(l, itl->first), tr = std::max(r, std::prev(itr)->second);
        erase(itl, itr);
        if (tl < l) (*this)[tl] = l - 1;
        if (r < tr) (*this)[r + 1] = tr;
    }
    // Is p and q in same segment?
    bool same(signed p, signed q) const {
        const auto&& it = get(p);
        return it != end() && it->first <= q && q <= it->second;
    }
};

void solve() {
    int n, q; cin >> n >> q;
    auto a = i64_vec_IN(q);

    SegmentMap mp(true);
    vector<bool> isBlack(n, false);
    vll ans;
    for(auto ai: a) {
        ai--;

        if(isBlack[ai]) {
            mp.remove(ai,ai);
        } else {
            mp.insert(ai,ai);
        }
        ans.emplace_back(mp.size());
        isBlack[ai] = !isBlack[ai];
    }

    rep(i,q) cout << ans[i] << "\n\n"[i+1==q];
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


