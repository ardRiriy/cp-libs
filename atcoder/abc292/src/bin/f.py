# translated by claude 3.7 sonnet
import math

def solve():
    a, b = map(float, input().split())
    if a > b:
        a, b = b, a
    
    upper = 0.0
    lower = math.pi / 6.0
    
    for _ in range(1000000):
        mid = (upper + lower) / 2.0
        t = a * math.cos(mid) / math.cos(30.0 * math.pi / 180.0 - mid)
        
        if t <= b:
            lower = mid
        else:
            upper = mid
    
    print(f"{a / math.cos(math.pi / 6.0 - upper):.15f}")

def main():
    solve()

if __name__ == "__main__":
    main()

"""
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

void solve() {
    long double a, b; cin >> a >> b;
    if(a>b) swap(a, b);
    long double upper = 0.0, lower = PI / 6.0;

    rep(_, 1e6) {
        long double mid = (upper + lower) / 2.0;
        long double t = a * cos(mid) / cos(30.0-mid);
        if(t <= b) {
            lower = mid;
        }  else {
            upper = mid;
        }
    }
    cerr << fixed << setprecision(15) << upper << ' ' << lower << '\n';
    cout << fixed << setprecision(15) << a / cos(PI / 6.0 - upper) << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


"""