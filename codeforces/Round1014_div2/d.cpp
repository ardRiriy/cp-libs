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

void solve() {
    // L : 0, I: 1, T: 2
    int n; cin >> n;
    // string s; cin >> s;
    string t = "LIT";
    // vll cnt(3, 0);
    // rep(i, n) {
    //     rep(j, 3) {
    //         if(s[i] == t[j]) cnt[j]++;
    //     }
    // }

    // vll ans;
    // while(cnt[0] != cnt[1] || cnt[1] != cnt[2] || cnt[0] != cnt[2]) {
    //     ll min_i = -1;
    //     ll val = inf;
    //     rep(i, 3) if(chmin(val, cnt[i])) min_i = i;

    //     bool flag = true;
    //     rep(i, s.size()-1) {
    //         if(s[i] != s[i+1] && s[i] != t[min_i] && s[i+1] != t[min_i]) {
    //             s.insert(s.begin()+i+1, t[min_i]);
    //             ans.push_back(i+1);
    //             cnt[min_i]++;
    //             flag = false;
    //             break;
    //         }
    //     }
        
    //     if(flag) {
    //         cout << "-1\n";
    //         return;
    //     }
    // }

    // if(ans.size() > 2*n) {
    //     cout << "-1\n";
    //     return;
    // }else{
    //     cout << ans.size() << '\n';
    //     for(auto v: ans) {
    //         cout << v << '\n';
    //     }
    // }


   // cerr << s << '\n';
    map<char, int> indi;
    indi['L'] = 0;
    indi['I'] = 1;
    indi['T'] = 2;

    string s; cin>>s;
    bool check = true;
    rep(i, n-1) {
        check = check && s[i] == s[i+1];
    }
    if(check) {
        cout << "-1\n";
        return;
    }

    // Sに含まれるもののcnt
    vll cnt(3, 0);
    for(char c: s) {
        cnt[indi[c]]++;
    }

    if(cnt[0] == cnt[1] && cnt[1] == cnt[2]) {
        cout << "0\n";
        return;
    }

    // 増加可能数
    vll increase_cnt(3, 0);
    rep(i, n-1) {
        int idx = -1;
        rep(j, 3) {
            if(s[i] != t[j] && s[i+1] != t[j]) idx = j;
        }
        increase_cnt[idx] ++;
    }

    vll ans;

    {
        vll cnt_indicate(3, 0);
        iota(all(cnt_indicate), 0);
        sort(all(cnt_indicate), [&](const ll& i, const ll& j) {
            return cnt[i] < cnt[j];
        });


        
    }

    //     ll d1 = 0;
    //     if(cnt[cnt_indicate[0]] + increase_cnt[cnt_indicate[0]] < cnt[cnt_indicate[1]]) {
    //         cerr << "cnt_indicate[0]=" << " " << cnt_indicate[0]; 
    //         cout << "-1\n";
    //         return;
    //     } else {
    //         d1 = cnt[cnt_indicate[1]] - cnt[cnt_indicate[0]];
    //         ans += d1;
    //     }
    //     ll d2 = cnt[cnt_indicate[2]] - cnt[cnt_indicate[1]];
    //     ans += 2*d2;

    //     if(ans <= 2*n) {
    //         cout << ans << '\n';
    //         per(i, n-1) {
    //             if(d1>0 && s[i] != t[cnt_indicate[0]] && s[i+1] != t[cnt_indicate[0]] && s[i] != s[i+1]) {
    //                 cout << i+1 << '\n';
    //                 d1--;

    //                 if(s[i] == t[cnt_indicate[2]]) {
    //                     // i側に追加
    //                     rep(j, 2*d2) {
    //                         cout << i+1 << '\n';
    //                     }
                        
    //                 } else {
    //                     // i+1
    //                     rep(j, 2*d2) {
    //                         cout << i+j+2 << '\n';
    //                     }
    //                 }
    //                 d2 = 0;
    //             }
    //             if(d2>0 && !((s[i] == t[cnt_indicate[0]] && s[i+1] == t[cnt_indicate[1]]) || (s[i+1] == t[cnt_indicate[0]] && s[i] == t[cnt_indicate[1]])) && s[i] != s[i+1]) {
    //                 rep(i, 2*d2) {
    //                     cout << i+1 << '\n';
    //                 }
    //                 d2=0;
    //             }
    //         }
    //     } else {
    //         cout << "-1\n";
    //         return;
    //     }
    // }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--)solve();
}
