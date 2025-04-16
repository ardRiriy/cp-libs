#include <bits/stdc++.h>
#include <atcoder/all>

using namespace std;
using namespace atcoder;

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

enum Types {
    Plus,
    Minus,
    Mul,
    Div,
    Num,
};

struct Node {
    Types type;
    string num = "UNDEFIND";
    vector<Node*> child;
    Node(Types t) {
        type = t;
    }  

    void set_num(string& s){
        num = s;
    }

    void append(Node* c) {
        child.emplace_back(c);
    }

    void to_str() {
        if(type==Num) {
            cout << num;
        } else {
            char sep = '+';
            switch(type) {
                case Plus: break;
                case Minus: sep = '-'; break;
                case Mul: sep = '*'; break;
                case Div: sep = '/'; break;
                default: break;
            }
            cout << "(";
            child[0]->to_str();
            rep2(i, 1, child.size()) {
                cout << sep;
                child[i]->to_str();
            }
            cout << ")";
        }
    }
};

int cur = 0;

Node* tokenize(string& s) {
    if(cur >= s.size()) assert(false);
    if('0' <= s[cur] && s[cur] <= '9') {
        // 数字なら, numとしてparseして返す
        string num = "";
        while(cur < s.size() && '0' <= s[cur] && s[cur] <= '9') {
            num += s[cur];
            cur++;
        }
        Node* res = new Node(Num);
        res->set_num(num);
        return res;
    } else {
        Types t;
        switch(s[cur]) {
            case '+': t = Plus; break;
            case '-': t = Minus; break;
            case '*': t = Mul; break;
            case '/': t = Div; break;
            default: break;
        }
        Node* res = new Node(t);
        cur += 2; // 記号+開括弧で2token
        while(cur < s.size()) {
            Node* ret = tokenize(s);
            res->append(ret);
            if(s[cur]==')') break;
            else cur++;
        }
        cur++; // 閉じ括弧から一つ進めておく
        return res;
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    
    string s; cin >> s;
    Node* n = tokenize(s);
    n->to_str();
    cout << "\n";
}

