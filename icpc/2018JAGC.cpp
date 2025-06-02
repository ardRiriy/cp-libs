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

class Node {
    public:
        int type; // 0: 式, 1: 値
        Node* l;
        Node* r;
        char cs;
        char op;
        Node(int t) :type(t) {}

        void debug(int nest) {
            string base = "";
            rep(_,nest) {
                base+="  ";
            }
            cerr << base << "{\n";
            cerr << base << "  type: " << type << '\n';
            if(type==0) {
                cerr << base << "  op: " << op << '\n';
                cerr << base << "  l: " << '\n';
                l->debug(nest+1);
                cerr << base << "  r: " << '\n';
                r->debug(nest+1);
            } else {
                cerr << base << "  cs: " << cs << '\n';
            }
            cerr << base << "  }\n";
        }

        int hash(vector<int>& v) {
            if(type==1) {
                return v[cs-'a'];
            } else {
                int lv = l->hash(v);
                int rv = r->hash(v);
                if(op=='*') return lv&rv;
                else if(op=='+') return lv|rv;
                else return lv^rv;
            }
            assert(false);
        }
};

Node* gen(int& cur, string& s) {
    if(s[cur] == '[') {
        cur++;
        char op = s[cur++];
        Node* res = new Node(0);
        res->op = op;

        res->l = gen(cur,s);
        res->r = gen(cur,s);

        if(s[cur]==']') cur++;

        return res;
    } else {
        Node* res = new Node(1);
        res->cs = s[cur++];
        return res;
    }
}

int solve(){
    string s, p; cin >> s >> p;
    if(s==".")return 1;
    int cur = 0;
    Node* n = gen(cur,s);
    
    vector<int> v;
    for(char c: p) {
        v.push_back(c-'0');
    }
    int hash = n->hash(v);

    int ans = 0;
    v.clear();
    rep(i,10) {
        v.push_back(i);
        rep(j,10) {
            v.push_back(j);
            rep(k,10) {
                v.push_back(k);
                rep(l,10) {
                    v.push_back(l);
                    int res = n->hash(v);
                    if(res==hash) ans++;
                    v.pop_back();                    
                }
                v.pop_back();
            }
            v.pop_back();
        }
        v.pop_back();
    }

    cout << hash << " " << ans << '\n';
    return 0;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}


