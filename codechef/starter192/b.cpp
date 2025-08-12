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


// ty: https://algo-method.com/descriptions/84
// N の約数をすべて求める関数

vector<long long> calc_divisors(long long N) {

    // 答えを表す集合

    vector<long long> res;



    // 各整数 i が N の約数かどうかを調べる

    for (long long i = 1; i * i <= N; ++i) {

        // i が N の約数でない場合はスキップ

        if (N % i != 0) continue;



        // i は約数である

        res.push_back(i);



        // N ÷ i も約数である (重複に注意)

        if (N / i != i) res.push_back(N / i);

    }



    // 約数を小さい順に並び替えて出力

    sort(res.begin(), res.end());

    return res;

}

void solve() {
    ll x, y, z, c; cin >> x >> y >> z >> c;
    if(x>y) swap(x,y);

    if(x==y) {
        cout << "0\n";
        return;
    }

    ll ans = c*2;

    for(auto ci: calc_divisors(y)){
        dbg(ci);
        dbg(gcd(ci,x));
        dbg(lcm(ci,y));
        if(lcm(ci,x)==gcd(ci,y)) {
            dbg(ci);
            chmin(ans,abs(z-ci)+c);
        }
    }
    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


