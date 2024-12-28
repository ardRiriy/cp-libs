#include <bits/stdc++.h>
#define int long long
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define revrep(i, n) for (int i = (int)(n); i >= 0; i--)
#define itrep(itr, stl) for (auto itr = stl.begin(); itr != stl.end(); itr++)
#define Vec2D(type, n, m, val) vector<vector<type>>(n, vector<type>(m, val))
#define print(x) cout << x << endl
#define all(a) a.begin(), a.end()
const int INF = LLONG_MAX;
const int N_INF = LLONG_MIN;
using namespace std;
class UnionFind {
   private:
    map<int, int> uf;

   public:
    int root(int n) {
        if (uf.find(n) == uf.end()) uf[n] = -1;
        if (uf[n] < 0)
            return n;
        else
            return uf[n] = root(uf[n]);
    }
    bool connected(int a, int b) { return root(a) == root(b); }
    void marge(int a, int b) {
        int root_a = root(a);
        int root_b = root(b);
        if (root_a != root_b) {
            if (uf[root_a] > uf[root_b]) swap(root_a, root_b);
            uf[root_a] += uf[root_b];
            uf[root_b] = root_a;
        }
    }
    int size(int n) { return -uf[root(n)]; }
};
bool chmin(int &a, int b) {
    if (a > b) {
        a = b;
        return true;
    }
    return false;
}
bool chmax(int &a, int b) {
    if (a < b) {
        a = b;
        return true;
    }
    return false;
}
int power(int x, int n) {
    int result = 1;
    while (n > 0) {
        if ((n & 1) == 1) {
            result *= x;
        }
        x *= x;
        n >>= 1;
    }
    return result;
} /*x^nを計算*/
int b_search(vector<int> &v, int k) {
    int ng = -1, ok = v.size();
    while (abs(ng - ok) > 1) {
        int mid = ok + (ng - ok) / 2;
        if (v[mid] >= k)
            ok = mid;
        else
            ng = mid;
    }
    return ok;
}

void solve() {
    // hogehoge
    int h, w;
    cin >> h >> w;
    
    vector<string> s(h), t(h);
    rep(i, h) {
        cin >> s[i];
    }
    rep(i, h) {
        cin >> t[i];
    }

    map<string, int> cnts, cntt;

    rep(j, w) {
        string ss = "";
        string tt = "";
        rep(i, h) {
            ss += s[i][j];
            tt += t[i][j];
        }

        cnts[ss] += 1;
        cntt[tt] += 1;
    }

    itrep(itr, cntt) {
        if(itr->second != cnts[itr->first]) {
            print("No");
            return;
        } 
    }
    print("Yes");

}

signed main() {
    std::cout << std::fixed;
    std::cout << std::setprecision(20);
    std::cin.tie(0)->sync_with_stdio(0);
    int times = 1;
    // cin >> times;
    while (times--) solve();
    return 0;
}