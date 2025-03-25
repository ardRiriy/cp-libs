
#include <bits/stdc++.h>
#include <atcoder/all>
using namespace atcoder;
using namespace std;

#define all(v) v.begin(),v.end()
#define resort(v) sort(v.rbegin(),v.rend())
using ll = long long;
using ull = unsigned long long;
using vll=vector<ll>;
using vvll = vector<vector<ll>>;
using P = pair<ll,ll>;
using vp=vector<pair<ll, ll>>;

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


void solve() {
    string S;
    cin >> S;
    int n = S.size();
    bool has_q = false;
    rep(i,n) has_q = has_q || S[i] == '?';

    if(has_q) {
        ll cnt_open = 0, cnt_close = 0;
        rep(i, n) {
            if(S[i] == '(') cnt_open++;
            else if(S[i] == ')') cnt_close++;
        }
        if(abs(cnt_close-cnt_open)!=1) {
            cout << "Second\n";
            return;
        }

        if(cnt_close<cnt_open) {
            // closeを一番右端のところに置く
            per(i, n) {
                if(S[i] == '?') {
                    S[i] = ')';
                    break;
                }
            }
        } else {
            // openをおく
            rep(i, n){
                if(S[i] == '?') {
                    S[i] = '(';
                    break;
                }
            }
        }
    }

    ll turn = 0;
    rep(i,n) {
        if(S[i] == '?') {
            S[i] = ((turn == 0) ? ')' : '(');
            turn = 1-turn;
        }
    }

    ll cnt = 0;
    rep(i, n) {
        if(S[i] == '(') cnt++;
        else cnt--;
        if(cnt < 0) {
            cout << "Second\n";
            return;
        }
    }
    cout << ((cnt == 0) ? "First\n" : "Second\n");

}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    
    while(t--) solve();
}
