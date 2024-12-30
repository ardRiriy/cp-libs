#include <bits/stdc++.h>
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

/* BIT: 区間和の更新や計算を行う構造体
    初期値は a_1 = a_2 = ... = a_n = 0
    ・add(i,x): a_i += x とする
    ・sum(i): a_1 + a_2 + ... + a_i を計算する
    計算量は全て O(logn)
*/
template <typename T>
struct BIT {
    int n;          // 配列の要素数(数列の要素数+1)
    vector<T> bit;  // データの格納先
    BIT(int n_) : n(n_ + 1), bit(n, 0) {}

    void add(int i, T x) {
        for (int idx = i; idx < n; idx += (idx & -idx)) {
            bit[idx] += x;
        }
    }

    T sum(int i) {
        T s(0);
        for (int idx = i; idx > 0; idx -= (idx & -idx)) {
            s += bit[idx];
        }
        return s;
    }
};

void solve() {
    int n;
    cin >> n;

    vector<int> count(2*n+2, 0);
    BIT<int> bit(2*n+2);

    int l[n], r[n];
    rep(i, n) {
        cin >> l[i] >> r[i];
        if(l[i] == r[i]) {
            count[l[i]] += 1;
            if(count[l[i]] == 1) {
                bit.add(l[i], 1);
            }
        }
    }


    char ans[n];

    rep(i, n) {
        if(l[i] == r[i]) {
            if (count[l[i]] == 1) ans[i] = '1';
            else ans[i] = '0';
            continue;
        }
        int k = bit.sum(r[i]) - bit.sum(l[i]-1);
        if(k >= r[i] - l[i] + 1) {
            ans[i] = '0';
        } else {
            ans[i] = '1';
        }
    }

    rep(i, n) {
        cout << ans[i];
    }
    cout << "\n";
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);

    int t;
    cin >> t;
    while(t--) solve();
}
