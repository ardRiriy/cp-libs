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
    string s,t;cin>>s>>t;
    
    vector<pair<char,int>> vs, vt;
    int cnt = 0;
    char prev = '-';
    for(auto c: s){
        if(prev==c) {
            cnt++;
        } else {
            if(prev!='-') {
                vs.push_back({prev, cnt});
            }
            prev=c;
            cnt=1;
        }
    }
    vs.push_back({prev,cnt});

    cnt=0;
    prev='-';
    for(auto c: t){
        if(prev==c) {
            cnt++;
        } else {
            if(prev!='-') {
                vt.push_back({prev, cnt});
            }
            prev=c;
            cnt=1;
        }
    }
    vt.push_back({prev,cnt});

    if(vs.size()!=vt.size()) {
        cout << "No\n";
        return;
    }

    rep(i,vs.size()) {
        auto [sc, cs] = vs[i];
        auto [tc, ct] = vt[i];
        if(sc != tc || cs*2<ct || ct < cs) {
            cout << "No\n";
            return;
        }
    }
    cout << "Yes\n";
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--)solve();
}