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

// r, d, l, u
ll dy[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dx[] = {0, 1, 0, -1, -1, 1, 1, -1};
char dc[] = {'R', 'D', 'L', 'U'};

void solve() {
    int n, m; cin >> n >> m;
    vector<string> bd(n); rep(i, n) cin >> bd[i];

    vvll dist_f_zero(n, vll(m, inf));
    dist_f_zero[0][0] = 0;
    int i = 0, j = 0;
    while(i >= 0 && i < n && j >= 0 && j < m) {
        //cerr << i << ' ' << j << '\n';
        if(i==n-1 && j ==m-1) break;

        bool flag = true;
        rep(r, 4){
            if(dc[r] == bd[i][j]) {
                int ni = i + dx[r];
                int nj = j + dy[r];

                if(0 <= ni && ni < n && 0 <= nj && nj < m) {
                    if(dist_f_zero[ni][nj] != inf) {
                        flag = false;
                        break;
                    }
                    dist_f_zero[ni][nj] = dist_f_zero[i][j] + 1;
                }
                i = ni;
                j = nj;
                break;
            }
        }
        if(!flag) break;
    }

    vvll dist_t_nm(n, vll(m, inf));
    dist_t_nm[n-1][m-1] = 0;
    queue<pair<int, int>> que;
    que.push({n-1, m-1});
    while(!que.empty()) {
        auto [pi, pj] = que.front(); que.pop();

        rep(r, 4) {
            int ni = pi + dx[r];
            int nj = pj + dy[r];
            if(0 <= ni && ni < n && 0 <= nj && nj < m) {
                if(dist_t_nm[ni][nj] != inf) continue;
                rep(nr, 4) {
                    if(bd[ni][nj] == dc[nr]) {
                        int nni = ni + dx[nr];
                        int nnj = nj + dy[nr];
                        if(nni == pi && nnj == pj) {
                            dist_t_nm[ni][nj] = dist_t_nm[pi][pj] + 1;
                            que.push({ni, nj});
                        }
                    }
                }
            }
        }
    }

    rep(i,n) {
        rep(j,n) {
            cerr << dist_t_nm[i][j] << ' ';
        }
        cerr << '\n';
    }


    ll ans = 0;
    rep(i, n) rep(j, m) {
        if(dist_f_zero[n-1][m-1] == inf) {
            if(dist_f_zero[i][j] == inf) continue;
            rep(r, 4) {
                if(bd[i][j] == dc[r]) continue;
                int ni = i + dx[r];
                int nj = j + dy[r];
                if(0 <= ni && ni < n && 0 <= nj && nj < m) {
                    if(dist_t_nm[ni][nj] != inf) ans++;
                } 
            }
        } else {
            if(dist_f_zero[i][j] == inf || (i==n-1 && j==m-1)) {
                ans += 3;
                continue;
            }
            rep(r, 4) {
                if(bd[i][j] == dc[r]) continue;
                int ni = i + dx[r];
                int nj = j + dy[r];
                if(0 <= ni && ni < n && 0 <= nj && nj < m) {
                    if(dist_f_zero[ni][nj] == inf) {
                        if(dist_t_nm[ni][nj] != inf) {
                            ans ++;
                        }else {
                        }
                    } else {
                        if(dist_f_zero[i][j] < dist_f_zero[ni][nj]) {
                            ans++;
                        } else {
                        }
                    }
                } else {
                    
                    
                }
            }
        }
    }
    cout << ans << '\n';
    
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


// H~L