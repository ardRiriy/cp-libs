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

void solve() {
    int n; cin >> n;
    vll a = i64_vec_IN(n);
    vll b = i64_vec_IN(n);
    ll as = 0, bs = 0;
    for(auto ai: a) as += ai;
    for(auto bi: b) bs += bi;
    if(as != bs) {
        cout << "No\n";
        return;
    }

    // この場合必ず可能？
    // n>=3なら必ず可能そうではあるが、未証明
    // 3、無理じゃね？終わりだが
    // できそうじゃね？
    if(n==2) {
        bool flag = true;
        rep(i,n) flag = flag && a[i] == b[i];
        if(flag) {
            cout << "Yes\n0\n";
            return;
        }
        flag = true;
        swap(a[0],a[1]);
        a[0]-=1;
        a[1]+=1;
        rep(i,n) flag = flag && a[i] == b[i];
        if(flag) {
            cout << "Yes\n1\n";
            cout << "1 2\n";
            return;
        } else {
            cout << "No\n";
            return;
        }
    }
    vp ans;
    while(b.size()>3) {
        auto targ = *b.rbegin();
        b.pop_back();

        ll cur = 0;
        if(a[cur]+1<=targ) {
            while(a[cur]+1 < targ) {
                if(cur==2) {
                    swap(a[0], a[2]);
                    a[0]--;
                    a[2]++;
                    ans.push_back({0,2});
                    cur = 0;
                } else {
                    swap(a[cur], a[cur+1]);
                    a[cur]--;
                    a[cur+1]++;
                    ans.push_back({cur,cur+1});
                    cur++;
                }
            }
        } else {
            while(a[cur]+1>targ) {
                if(cur==0) {
                    swap(a[0], a[2]);
                    a[0]--;
                    a[2]++;
                    ans.push_back({0,2});
                    cur = 2;
                } else {
                    swap(a[cur], a[cur-1]);
                    a[cur]++;
                    a[cur-1]--;
                    ans.push_back({cur-1, cur});
                    cur--;
                }
            }
        }

        assert(a[cur]+1==targ);
        swap(a[cur], a[b.size()]);
        a[cur]--;
        a[b.size()]++;
        ans.push_back({cur, b.size()});
    }



    dbg(a);
    dbg(b);
    dbg(ans);

    vp add;
    rep(i,3) {
        vll aa = a;
        auto targ = *b.rbegin();
        // b.pop_back();

        ll cur = i;
        if(a[cur]+1<=targ) {
            while(a[cur]+1 < targ) {
                if(cur==2) {
                    swap(a[0], a[2]);
                    a[0]--;
                    a[2]++;
                    add.push_back({0,2});
                    cur = 0;
                } else {
                    swap(a[cur], a[cur+1]);
                    a[cur]--;
                    a[cur+1]++;
                    add.push_back({cur,cur+1});
                    cur++;
                }
            }
        } else {
            while(a[cur]+1>targ) {
                if(cur==0) {
                    swap(a[0], a[2]);
                    a[0]--;
                    a[2]++;
                    add.push_back({0,2});
                    cur = 2;
                } else {
                    swap(a[cur], a[cur-1]);
                    a[cur]++;
                    a[cur-1]--;
                    add.push_back({cur-1, cur});
                    cur--;
                }
            }
        }

        bool flag = true;
        rep(i,3) flag = flag && a[i] == b[i];
        if(flag) {
            break;
        }
        a = aa;
    }


    cout << "Yes\n";
    cout << ans.size() + add.size() << '\n';

    for(auto[u, v]: ans) {
        cout << u+1 << " " << v+1 << '\n';
    }
    for(auto[u, v]: add) {
        cout << u+1 << " " << v+1 << '\n';
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


