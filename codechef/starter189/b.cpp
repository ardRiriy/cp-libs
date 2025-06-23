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
std::vector<int> Eratosthenes( const int N )
{
    std::vector<bool> is_prime( N + 1 );
    for( int i = 0; i <= N; i++ )
    {
        is_prime[ i ] = true;
    }
    std::vector<int> p;
    for( int i = 2; i <= N; i++ )
    {
        if( is_prime[ i ] )
        {
            for( int j = 2 * i; j <= N; j += i )
            {
                is_prime[ j ] = false;
            }
            p.emplace_back( i );
        }
    }
    return p;
}

vector<int> primes = Eratosthenes(1e6);

void solve() {
    int n; cin >> n;
    if(n==1) {
        cout << "1\n";
    } else if(n==2) {
        cout << "1 2\n";
    } else if(n==3) {
        cout << "2 1 3\n";
    } else if(n==4) {
        // f(P)=1
        cout << "2 1 3 4\n";
    } else if(n==5) {
        cout << "3 1 4 5 2\n";
    } else {
        vll ans(n, -inf);
        ans[0] = 2;
        ans[1] = 1;
        ans[2] = 3;
        ans[3] = 5;
        ans[4] = 4;

        set<ll> pool;
        rep2(i, 6, n+1) {
            pool.insert(i);
        }
        
        ll cur = 3;
        ll idx = 5;
        while(primes[cur] <= n) {
            assert(idx<n);
            ans[idx] = primes[cur];
            pool.erase(primes[cur]);
            cur++;
            idx+=2;
        }

        rep(i,n) {
            if(ans[i] != -inf) continue;
            ans[i] = *pool.begin();
            pool.erase(ans[i]);
        }

        rep(i,n) cout << ans[i] << " \n"[i+1==n];
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


