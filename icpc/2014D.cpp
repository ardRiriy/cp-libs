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
    if(s=="#") return 1;

    set<string> ans;

    vector<bool> isUsed(26,false);
    auto dfs = [&](auto self, int i, int& left, string& cur) {
        if(i==s.size()) {
            ans.insert(cur);
            left--;
            return;
        }

        if(s[i] == 'a') {
            if(isUsed[0]) {
                cur += s[i];
                self(self,i+1,left,cur);
                cur.pop_back();
            } else {
                cur += s[i];
                // 変えない
                self(self,i+1,left,cur);
                cur.pop_back();
                if(left>0) {
                    cur += s[i]+1;
                    isUsed[s[i]-'a'] = true;
                    self(self,i+1,left,cur);
                    isUsed[s[i]-'a'] = false;
                    cur.pop_back();
                }
            }
        } else if(s[i] == 'z') {
            if(isUsed['y'-'a']) {
                cur += s[i];
                self(self,i+1,left,cur);
                cur.pop_back();
            } else {
                return;
            }
        } else {
            if(isUsed[s[i]-'a']) {
                // 変えてはならない
                cur += s[i];
                self(self,i+1,left,cur);
                cur.pop_back();
            } else {
                if(isUsed[s[i]-'a'-1]) {
                    cur += s[i];
                    self(self,i+1,left,cur);
                    cur.pop_back();
                }
                if(left==0) return;
                cur += s[i]+1;
                isUsed[s[i] - 'a']  = true;
                self(self,i+1,left,cur);
                isUsed[s[i] - 'a']  = false;
                cur.pop_back();
            }
        }
        return;
    };
    auto dfs2 = [&](auto self, int i, int& left, string& cur) {
        if(i==s.size()) {
            ans.insert(cur);
            left--;
            return;
        }

        if(s[i] == 'a') {
            if(isUsed[0]) {
                cur += s[i];
                self(self,i+1,left,cur);
                cur.pop_back();
            } else {
                cur += s[i]+1;
                isUsed[s[i]-'a'] = true;
                self(self,i+1,left,cur);
                isUsed[s[i]-'a'] = false;
                cur.pop_back();
                if(left==0) return;
                cur += s[i];
                // 変えない
                self(self,i+1,left,cur);
                cur.pop_back();
            }
        } else if(s[i] == 'z') {
            if(isUsed['y'-'a']) {
                cur += s[i];
                self(self,i+1,left,cur);
                cur.pop_back();
            } else {
                return;
            }
        } else {
            if(isUsed[s[i]-'a']) {
                // 変えてはならない
                cur += s[i];
                self(self,i+1,left,cur);
                cur.pop_back();
            } else {
                cur += s[i]+1;
                isUsed[s[i] - 'a']  = true;
                self(self,i+1,left,cur);
                isUsed[s[i] - 'a']  = false;
                cur.pop_back();
                if(left==0) return;

                if(isUsed[s[i]-'a'-1]) {
                    cur += s[i];
                    self(self,i+1,left,cur);
                    cur.pop_back();
                }

            }
        }
        return;
    };

    int l = 5;
    string cur;
    dfs(dfs,0,l,cur);

    l = 5;
    cur = "";
    
    dfs2(dfs2, 0, l, cur);

    map<vector<bool>,ll> memo;

    auto cnt = [&](auto self, int i) -> ll {
        if(i==s.size()) {
            return 1;
        }
        if(memo.contains(isUsed)) return memo[isUsed];
        ll res = 0;
        if(s[i] == 'a') {
            if(isUsed[0]) {
                res += self(self,i+1);
            } else {
                res += self(self,i+1);
                isUsed[s[i]-'a'] = true;
                res += self(self,i+1);
                isUsed[s[i]-'a'] = false;
            }
        } else if(s[i] == 'z') {
            if(isUsed['y'-'a']) {
                res += self(self,i+1);
            }
        } else {
            if(isUsed[s[i]-'a']) {
                // 変えてはならない
                res += self(self,i+1);
            } else {
                if(isUsed[s[i]-'a'-1]) {
                    res += self(self,i+1);
                }
                isUsed[s[i] - 'a']  = true;
                res += self(self,i+1);
                isUsed[s[i] - 'a']  = false;
            }
        }

        return memo[isUsed] = res;
    };

    cout << cnt(cnt,0) << '\n';
    for(auto ss: ans) {
        cout << ss << '\n';
    }
    return 0;
}


int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}


