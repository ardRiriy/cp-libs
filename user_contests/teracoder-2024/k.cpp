#include <bits/stdc++.h>
#define int long long
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define revrep(i, n) for (int i = (int)(n); i >= 0; i--)
#define itrep(itr, stl) for (auto itr = stl.begin(); itr != stl.end(); itr++)
#define Vec2D(type, n, m, val) vector<vector<type>>(n, vector<type>(m, val))
#define print(x) cout << x << endl
#define all(a) a.begin(), a.end()
using namespace std;

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
} 

void dfs(vector<vector<int>> &g, int p, vector<int> &history, vector<bool> &seen, vector<bool> &fin, set<int> &ans, vector<bool> &state) {
    seen[p] = true;
    history.push_back(p);

    for(int ni: g[p]) {
        if (state[p] && fin[ni]) {
            for(int i = history.size()-1; i >= 0; i--) {
                if(ans.find(history[i]) != ans.end()) break;
                ans.insert(history[i]);
                state[history[i]] = true;
            }
            continue;
        }
        if(seen[ni] && !fin[ni]) {
            for(int i = history.size()-1; i >= 0; i--) {
                ans.insert(history[i]);
                state[history[i]] = true;
                if (history[i] == ni) break;
            }
            continue;
        }
        if (state[p]) {
            state[ni] = true;
        }
        dfs(g, ni, history, seen, fin, ans, state);
    }
    fin[p] = true;
    history.pop_back();
    return;
}

void solve() {
    int n, m;
    cin >> n >> m;

    vector<vector<int>> g(n);

    rep(i, m) {
        int u, v;
        cin >> u >> v;
        u--; v--;
        g[u].push_back(v);
    }

    set<int> ans;
    vector<bool> seen(n, false), fin(n, false), state(n, false);
    vector<int> his;

    dfs(g, 0, his, seen, fin, ans, state);
    if (ans.size() == 0) cout << "Happy" << endl;
    else cout << ans.size() << endl;
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