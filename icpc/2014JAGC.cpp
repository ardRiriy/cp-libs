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
    string s; cin >> s;
    if(s==".") return 1;
    deque<char> que;
    for(char c: s) {
        if(c==']'){
            string t = "";
            while(!que.empty()) {
                char b = que.back();
                que.pop_back();
                if(b=='[') break;
                t += b;
            }
            dbg(t);

            for(char cc: t) que.push_back(cc);
        } else if(c == '?' || (c>='A' && c<='Z')) {
            char nc = c;
            while(!que.empty()) {
                char pm = que.back();
                que.pop_back();
                if(pm == '+') nc = (nc == 'Z') ? 'A' : nc+1;
                else if(pm == '-') nc = (nc == 'A') ? 'Z' : nc-1;
                else {
                    que.push_back(pm);
                    break;
                }
            }
            que.push_back((c=='?'? '?' : nc));
        } else {
            que.push_back(c);
        }
    }
    string ans = "";
    while(!que.empty()) {
        char c = que.front();
        que.pop_front();
        ans += (c=='?')?'A':c;
    }
    cout << ans << '\n';

    return 0;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}


