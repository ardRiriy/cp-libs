// #include <bits/stdc++.h>
// #include "input.hpp"

#include <iostream>
#include <vector>
#include <string>
using namespace std;
// using namespace atcoder;

using ll = long long;
#define rep(i,n) for (ll i=0;i<(n);++i)
#define rep2(i,a,n) for (ll i=(a);i<(n);++i)
#define all(v) v.begin(),v.end()


vector<vector<char>> extend(int h, int w, vector<string>& x) {
    vector<vector<char>> board(h+18, vector<char>(w+18, '.'));

    rep2(i, 9, 9+h) {
        rep2(j, 9, 9+w) {
            board[i][j] = x[i-9][j-9];
        }
    }

    return board;
}

bool gen(int i, int j, vector<string>& a, vector<vector<char>>& state, vector<vector<char>>& x) {
    cerr << "====\n";

    rep(id, a.size()) {
        rep(jd, a[0].size()) {
            if(id+i >= state.size() || jd+j >= state[0].size()) {
                if(a[id][jd] == '#') {
                    return false;
                }
            } else {
                if(a[id][jd]=='#') {
                    if(x[id+i][jd+j]=='.') return false;
                    state[id+i][jd+j] = a[id][jd];
                } else {

                }
            }
        }
    }
    cerr << "====\n";
    return true;
}

void solve() {
    int h1, w1; cin >> h1 >> w1;
    vector<string> a(h1);
    rep(i,h1) cin >> a[i];

    int h2, w2; cin >> h2 >> w2;
    vector<string> b(h2);
    rep(i,h2) cin >> b[i];

    int h3, w3; cin >> h3 >> w3;
    vector<string> x(h3);
    rep(i,h3) cin >> x[i];

    vector<vector<char>> bd = extend(h3,w3,x);

    int n = bd.size();
    int m = bd[0].size();
    cerr << n << " " << m << "\n";
    rep(ii,n) rep(jj,m) {
        cerr << bd[ii][jj] << " \n"[jj+1==m];
    }
    cerr << "===============\n";



    rep(ia,n) rep(ja,m) {

        vector<vector<char>> state(n, vector<char>(m, '.'));
        if(!gen(ia,ja,a,state,bd)) continue;


        rep(ib, n) rep(jb,m) {
            auto state_cp = state;
            if(gen(ib,jb,b,state_cp,bd)) {
                if(bd==state_cp) {
                    cout << "Yes\n";
                    return;
                }
            }
        }
    }
    cout << "No\n";
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


