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

using mint = modint1000000007;

// thanks: https://qiita.com/seal_qiita/items/a0e6fa4def1a5b66b3d2
mint fact(ll n) {
    if(n==0) return 1;

    if(n>=1000000007){
        return 0;
    }

    long long catche_list[11]={1,
        927880474, //100000000!%mod
        933245637, //200000000!%mod
        668123525, //300000000!%mod
        429277690, //400000000!%mod
        733333339, //500000000!%mod
        724464507, //600000000!%mod
        957939114, //700000000!%mod
        203191898, //800000000!%mod
        586445753, //900000000!%mod
        698611116, //1000000000!%mod
    };
    	//nを10^8で割った商がcatche_listのインデックスになり起点が求まる
	long long start = n/100000000;
	mint ans = catche_list[start];

	//起点から階乗の計算を始める
	for(long long i=start*100000000+1;i<=n;i++){
		ans = ans * i;
	}
    return ans;
}

void solve() {
    ll a, b, k; cin >> a >> b >> k;
    mint h = mint(k) * (a-1) + 1;
    mint w = fact(h.val())/(fact(h.val())*k) * (b-1) + 1;
    cout << h.val() << " " << w.val() << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


