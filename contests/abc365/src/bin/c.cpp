#include <bits/stdc++.h>
#include <atcoder/all>
using namespace std;
using namespace atcoder;
typedef long long ll;
#define rep(i, s, n) for (int i = (s); i < (n); i++)
#define P pair<int, int>
#define V vector<int>
#define M map<int, int>
#define S set<int>
#define mp(a, b) make_pair(a, b)
const int INF = 1e9;

int main() {
    ll n, m;
    cin >> n >> m;
    vector<ll> a(n);
    rep(i, 0 ,n) cin >> a[i];

    ll ok = 0, ng = m+1;

    while(abs(ng - ok) > 1) {
        ll x = (ok + ng) / 2;
        ll paid_sum = 0;

        // 支払う金額を計算
        rep(i, 0, n) {
            if(a[i] < x) {
                paid_sum += a[i];
            } else {
                paid_sum += x;
            }
        }

        if (paid_sum <= m) {
            ok = x;
        } else {
            ng = x;
        }
    }

    // 全員が上限額をもらっていないなら、無限に支払っても良い
    bool check = true;
    rep(i, 0, n) {
        if(a[i] > ok) {
            check = false;
            break;
        }
    }

    if(check) {
        cout << "infinite" << endl;
    } else {
        cout << ok << endl;
    }
        
    return 0;
}