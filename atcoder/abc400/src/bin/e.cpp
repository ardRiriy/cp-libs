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

std::vector<ll> Eratosthenes( const int N )
{
    std::vector<bool> is_prime( N + 1, true );

    std::vector<ll> P;
    for( ll i = 2; i <= N; i++ )
    {
        if( is_prime[ i ] )
        {
            for( ll j = 2 * i; j <= N; j += i )
            {
                is_prime[ j ] = false;
            }
            P.emplace_back( i );
        }
    }
    return P;
}

vll create(ll n) {
    vll primes = Eratosthenes(n);
    vll res;

    rep(i, primes.size()) {
        rep2(j,i+1, primes.size()) {
            ll cur = primes[i] * primes[j];
            if(cur > n) break;

            rep(_, 1000000) {
                ll x = cur;
                rep(__, 10000000) {
                    res.push_back(x);
                    x *= primes[j];
                    if(x>n)break;
                }
                cur *= primes[i];
                if(cur > n) break;
            }
        }
    }
    return res;
}

vector<ll> primes_sq = create(1000000+1);
void solve() {
    ll a; cin >> a;
    ll ans = 0;

    // a以下で最大のtarg
    ll ok = 0, ng = primes_sq.size();
    while(ng-ok > 1) {
        ll mid = (ok+ng)/2;
        if(primes_sq[mid] * primes_sq[mid] <= a) ok = mid;
        else ng = mid;
    }
    cout << primes_sq[ok]*primes_sq[ok] << '\n';
}

int main() {
    sort(all(primes_sq));
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


