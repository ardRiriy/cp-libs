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

// thanks: https://chiwawa-star.hatenablog.com/entry/2016/11/25/015503
std::vector<long long> Eratosthenes( const int N )
{
    std::vector<bool> is_prime( N + 1 );
    for( int i = 0; i <= N; i++ )
    {
        is_prime[ i ] = true;
    }
    std::vector<long long> P;
    for( int i = 2; i <= N; i++ )
    {
        if( is_prime[ i ] )
        {
            for( int j = 2 * i; j <= N; j += i )
            {
                is_prime[ j ] = false;
            }
            P.emplace_back( i );
        }
    }
    return P;
}


vll primes = Eratosthenes(2e7);

void solve() {
    // K=N-1 素数の列挙でOK
    // K<N-1 構成不可能
    int k, n; cin >> n >> k;
    if(k<n-1) {
        cout << "-1\n";
        return;
    }

    
    set<ll> used;
    vll ans(n);
    // 2個を使って、k-(n-2)を達成する必要がある
    ll targ = k-(n-2);
    assert(targ>=1);
    ans[0] = targ;
    ans[1] = targ * 2;
    targ *= 2;
    ll v = 0;
    while(targ>1) {
        bool flag = false;
        while(targ%primes[v] == 0) {
            targ/=primes[v];
            flag = true;
        }
        if(flag) used.insert(primes[v]);
        v++;
    }

    int idx = 0;
    rep2(i,2,n) {
        while(used.contains(primes[idx])) idx++;
        ans[i] = primes[idx++];
        used.insert(ans[i]);
    }

    rep(i,n) cout << ans[i] << ((i+1==n)?'\n':' ');
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


