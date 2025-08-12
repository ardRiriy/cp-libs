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
        Node* lhs = nullptr;
        Node* rhs = nullptr;
        ll   val  = inf;
        Node() {}
};

string to_str(const Node* n) {
    if (!n) return "";
    return "(" + to_str(n->lhs) + ")[" + to_string(n->val) + "](" + to_str(n->rhs) + ")";
}

Node* tokenize(string &s, int &i, int d) {
    if(s[i] == '(') {
        i++;
        Node* res = new Node();
        if(s[i] == ')') {
            i++;
        } else {
            res->lhs = tokenize(s,i,d+1);
        }
        assert(s[i] == '[');
        i++;

        ll val = 0;
        while(s[i] >= '0' && s[i] <= '9') {
            val = val * 10 + (s[i] - '0');
            i++;
        }
        res->val = val;
        assert(s[i] == ']');
        i++;
        assert(s[i] == '(');
        i++;

        if(s[i] == ')') {
            i++;
        } else {
            res->rhs = tokenize(s,i, d+1);
        }

        i++;
        return res;
    } else {
        dbg(s[i]);
        assert(false);
    }
}

Node* create(Node* a, Node* b) {
    if(!a || !b) return nullptr;
    Node* res = new Node();
    res->val = a->val + b->val;
    res->lhs = create(a->lhs, b->lhs);
    res->rhs = create(a->rhs, b->rhs);
    return res;
}


int solve(){
    string a, b;
    cin >> a >> b;
    int i = 0;
    Node* a_nodes = tokenize(a,i,0);

    i = 0;
    Node* b_nodes = tokenize(b,i, 0);

    Node* ans = create(a_nodes, b_nodes);
    dbg(ans->val);
    cout << to_str(ans) << '\n';
    return 0;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    solve();
    // while(!solve());
}


