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
    int n, m;
    int si, sj;
    cin >> n >> m >> si >> sj;

    vector<pair<int, int>> home;
    rep(i, n) {
        int x, y;
        cin >> x >> y;
        home.push_back({x, y});
    }

    vector<pair<char, int>> move;
    rep(i, m) {
        char c;
        int d;
        cin >> c >> d;
        move.push_back({c, d});
    }

    map<int, set<int>> home_i;
    map<int, set<int>> home_j;
    rep(i, n) {
        home_i[home[i].first].insert(home[i].second);
        home_j[home[i].second].insert(home[i].first);
    }

    int ans = 0;
    int now_i = si;
    int now_j = sj;

    rep(i, m) {
        char c = move[i].first;
        int d = move[i].second;
        if (c == 'L') {
            int target = now_i - d;
            auto itr = home_j[now_j].lower_bound(target);
            if (itr == home_j[now_j].end()) {
                now_i = target;
                continue;
            } else {
                while(itr != home_j[now_j].end() && *itr <= now_i) {
                    ans++;
                    home_i[*itr].erase(now_j);
                    itr = home_j[now_j].erase(itr);
                }
                now_i = target;
            }
        } else if (c == 'R') {
            int target = now_i + d;
            auto itr = home_j[now_j].lower_bound(now_i);            
            if (itr == home_j[now_j].end()) {
                now_i = target;
            } else {
                while(itr != home_j[now_j].end() && *itr <= target) {
                    ans++;
                    home_i[*itr].erase(now_j);
                    itr = home_j[now_j].erase(itr);
                }
                now_i = target;
            }
        } else if (c == 'U') {
            int target = now_j + d;
            auto itr = home_i[now_i].lower_bound(now_j);
            if (itr == home_i[now_i].end()) {
                now_j = target;
            } else {
                while(itr != home_i[now_i].end() && *itr <= target) {
                    ans++;
                    home_j[*itr].erase(now_i);
                    itr = home_i[now_i].erase(itr);
                }
                now_j = target;
            }
        } else if (c == 'D') {
            int target = now_j - d;
            auto itr = home_i[now_i].lower_bound(target);
            if (itr == home_i[now_i].end()) {
                now_j = target;
            } else {
                while(itr != home_i[now_i].end() && *itr <= now_j) {
                    ans++;
                    home_j[*itr].erase(now_i);
                    itr = home_i[now_i].erase(itr);
                }
                now_j = target;
            }
        }
    }
    cout << now_i << " " << now_j << " " << ans << endl;
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