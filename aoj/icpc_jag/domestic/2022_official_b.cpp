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

void solve(int n) {
    vector<set<int>> vpq(n);
    set<int> left;
    int cnt = 0;
    rep(i, n) {
        int a, b; cin >> a >> b;
        if(a==b) {
            cnt++;
            continue;
        }
        vpq[i].insert(a);
        vpq[i].insert(b);
        left.insert(i);
    }
    
    int idx = 0;
    int ans = 0;
    while(!left.empty()) {
        if(vpq[idx].empty()) {
            left.erase(idx);
            idx = (idx+1)%n;
            continue;
        }
        if(!vpq[(idx+1)%n].empty()) ans++;
        
        int card = *vpq[idx].begin();
        vpq[idx].erase(card);
        if(vpq[idx].empty()) left.erase(idx);
        if(vpq[(idx+1)%n].find(card)==vpq[(idx+1)%n].end()){
            vpq[(idx+1)%n].insert(card);
        } else {
            vpq[(idx+1)%n].erase(card);
        } 
        idx = (idx+1)%n;
    }

    cout << ans << '\n'; 
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int n; cin >> n;
    while(n>0){
        solve(n);
        cin >> n;
    }
}
