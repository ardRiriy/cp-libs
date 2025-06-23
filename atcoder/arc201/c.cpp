#include <bits/stdc++.h>
#include "input.hpp"
#include "atcoder/modint.hpp"
using namespace std;
using namespace atcoder;

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

using mint = atcoder::modint998244353;

class Node {
    public:

    int end_count=0;
    int c = 0;

    Node* a = nullptr;
    mint aval = 0;

    Node* b = nullptr;
    mint bval = 0;

    Node() {

    }

    mint add(string& s, int i) {
        if(i < s.size()) {
            c++;
            if(s[i]=='A') {
                if(!a) {
                    a = new Node();
                }
                aval = a->add(s, i+1);
            } else {
                if(!b) {
                    b = new Node();
                }
                bval = b->add(s,i+1);
            }
        } else {
            end_count++;
        }

        return aval * bval + mint(2).pow(c) * end_count;
    }
};




void solve() {
    int n; cin >> n;
    Node base;
    rep(i,n) {
        string s; cin >> s;
        cout << base.add(s,0).val() << '\n';
    }

}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    // cin >> t;
    while(t--)solve();
}


