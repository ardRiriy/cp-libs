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

const ll inf=1ll<<30;
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

template<int MOD>
struct ModInt {
    int value;

    ModInt() : value(0) {}
    ModInt(int value) : value(value % MOD) {
        if (this->value < 0) this->value += MOD;
    }

    ModInt operator+(const ModInt& other) const {
        return ModInt((value + other.value) % MOD);
    }

    ModInt operator-(const ModInt& other) const {
        return ModInt((value - other.value + MOD) % MOD);
    }

    ModInt operator*(const ModInt& other) const {
        return ModInt((1LL * value * other.value) % MOD);
    }

    ModInt operator/(const ModInt& other) const {
        return *this * other.inv();
    }

    ModInt inv() const {
        int a = value, b = MOD, u = 1, v = 0;
        while (b) {
            int t = a / b;
            a -= t * b; std::swap(a, b);
            u -= t * v; std::swap(u, v);
        }
        return ModInt(u);
    }
};

template<class T>
auto comp_idx(T* ptr){
  return [ptr](int l_idx, int r_idx){
    return ptr[l_idx] < ptr[r_idx];
  };
}

void solve() {
    using mint = ModInt<998244353>;
    int n, q;
    cin >> n >> q;

    vector<int> idx_a(n), idx_b(n), a(n), b(n);
    rep(i, n) {
        idx_a[i] = i;
        idx_b[i] = i;
        cin >> a[i];
    }
    rep(i, n) {
        cin >> b[i];
    }
    a.push_back(inf);
    b.push_back(inf);

    sort(all(idx_a), [&a](size_t i1, size_t i2) {
        return a[i1] < a[i2];
    });

    sort(all(idx_b), [&b](size_t i1, size_t i2) {
        return b[i1] < b[i2];
    });

    mint p = 1;
    rep(i, n) {
        p = p * min(a[idx_a[i]], b[idx_b[i]]);
    }
    cout << p.value << " ";

    int x, i, new_val;
    int ok, ng, mid;
    rep(qq, q) {
        cin >> x >> i;
        i--;
        if(x==1){
            new_val = a[idx_a[i]] + 1;
            ok = n;
            ng = idx_a[i]-1;
            while(ok-ng>1){
                mid = (ok+ng)/2;
                if(a[idx_a[mid]] >= new_val) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            cerr << "ok:" << ok << endl;
            if(ok != 0 && ok != n) {
                p = p / min(b[idx_b[ok-1]], a[idx_a[ok-1]]) / min(a[idx_a[i]], b[idx_b[i]]);
                a[idx_a[i]] += 1;
                swap(idx_a[i], idx_a[ok-1]);
                p = p * min(b[idx_b[ok-1]], a[idx_a[ok-1]]) * min(a[idx_a[i]], b[idx_b[i]]);
            } else {
                p = p / min(b[idx_b[i]], a[idx_a[i]]);
                a[idx_a[i]] += 1;
                p = p * min(a[idx_a[i]], b[idx_b[i]]);
            }
        } else {
            new_val = b[idx_b[i]] + 1;
            ok = n;
            ng = idx_b[i]-1;
            while(ok-ng>1){
                mid = (ok+ng)/2;
                if(b[idx_b[mid]] >= new_val) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }

            cerr << "ok:" << ok << endl;
            if(ok != 0 && ok != n) {
                p = p / min(b[idx_b[ok-1]], a[idx_a[ok-1]]) / min(a[idx_a[i]], b[idx_b[i]]);
                b[idx_b[i]] += 1;
                swap(idx_b[i], idx_b[ok-1]);
                p = p * min(b[idx_b[ok-1]], a[idx_a[ok-1]]) * min(a[idx_a[i]], b[idx_b[i]]);
            } else {
                p = p / min(b[idx_b[i]], a[idx_a[i]]);
                b[idx_b[i]] += 1;
                p = p * min(a[idx_a[i]], b[idx_b[i]]);
            }
        }


        cout << p.value << ((qq == q-1) ? "\n" : " ");
    }
}

int main() {
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr); std::cout.tie(nullptr);

    int t; cin >> t;
    while(t--) solve();

}
