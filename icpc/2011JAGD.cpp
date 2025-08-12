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

vll calc(vll& a, vll& b, char c) {
    vll res;
    for(auto ai: a) {
        for(auto bi: b) {
            if(c=='+') res.emplace_back(ai+bi);
            else if(c=='-') res.emplace_back(ai-bi);
            else if(c=='*') res.emplace_back(ai*bi);
            else if(bi != 0) {
                bool isMinus = (ai<0) ^ (bi<0);
                res.emplace_back((abs(ai)/abs(bi)) * ((isMinus)?-1:1));
                
            }
        }
    }

    sort(all(res));
    res.erase(unique(all(res)), res.end());
    return res;
}

vll parse(string& s, int& i) {
    assert(s[i]=='(');
    i++;

    vvll values;
    vector<char> inst;

    while(i<s.size()) {
        if(s[i] == '(') {
            auto ret = parse(s,i);
            values.push_back(ret);            
            if(s[i] != ')') {
                inst.emplace_back(s[i++]);
            }
        }  else if(s[i] == ')') {
            i++;
            break;
        } else {
            ll cur = 0;
            while('0' <= s[i] && s[i] <= '9') {
                cur *= 10;
                cur += s[i]-'0';
                i++;
            }
            values.push_back({cur});
            if(s[i] != ')') inst.emplace_back(s[i++]);
        }

    }
    if(values.size()==1) {
        return values[0];
    }
    vll res;

    // [l,r)を評価したときにあり得る値
    vector<vvll> dp(values.size()+1, vvll(values.size()+1));

    rep(idx,values.size()) {
        dp[idx][idx+1] = values[idx];
    }
    dbg(values);
    dbg(dp);

    rep2(idx,2,values.size()+1) {
        rep(l,values.size()) {
            // [l,l+idx)を評価したときにあり得る値の個数
            if(l+idx>values.size()) continue;
            rep2(k, l+1, l+idx) {
                // [l,k), [k,l+idx)
                auto res = calc(dp[l][k], dp[k][l+idx], inst[k-1]);
                for(auto x: res) dp[l][l+idx].emplace_back(x);
            }
            sort(all(dp[l][l+idx]));
            dp[l][l+idx].erase(unique(all(dp[l][l+idx])),dp[l][l+idx].end());
        }
    }
    return dp[0][values.size()];
    // rep(idx, 1<<inst.size()) {
    //     vvll cur;
    //     cur.push_back(values[0]);

    //     rep(j,inst.size()) {
    //         if(((idx>>j)&1ll)!=1) {
    //             // 0ならinstを評価
    //             cur[cur.size()-1] = calc(cur[cur.size()-1],values[j+1],inst[j]);
    //         } else {
    //             cur.push_back(values[j+1]);
    //         }
    //     }

    //     vll result = cur[0];
    //     dbg(result);
    //     int xxx = 1;
    //     rep(j,inst.size()) {
    //         if(((idx>>j)&1ll)==1) {
    //             dbg(xxx,inst[j]);
    //             result = calc(result, cur[xxx++], inst[j]);
    //         }
    //     }
        
    //     dbg(bitset<4>(idx), result);

    //     for(auto x: result) res.emplace_back(x);
    // }

    // sort(all(res));
    // res.erase(unique(all(res)), res.end());
    // dbg(res);
}

int solve(){
    string s; cin >> s;
    if(s=="#") return 1;
    s = '(' + s + ')';
    int i = 0;
    vll ans = parse(s,i);
    cout << ans.size() << '\n';
    return 0;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}
