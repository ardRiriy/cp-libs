#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;

// using namespace atcoder;

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

#define rep(i,n) for (ll i=0;i<(n);++i)
#define per(i,n) for(ll i=(n)-1;i>=0;--i)
#define rep2(i,a,n) for (ll i=(a);i<(n);++i)
#define per2(i,a,n) for (ll i=(n)-1;i>=(a);--i)


template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};

void solve() {
    int n, k; cin >> n >> k;
    auto a = i64_vec_IN(n);

    if(n==2&&a[0]>a[1]&&a[0]-a[1]<k){
        cout<<"No\n";
        return;
    }

    vll ans;
    rep(i,n-1){

        if(a[i]>a[i+1]){
            ll d=a[i]-a[i+1];
            if(d>=k) {
                // swapして終わり
                a[i+1]+=k;
                swap(a[i],a[i+1]);
                ans.emplace_back(i+1);
                while(i>0&&a[i-1]>a[i]){
                    ans.emplace_back(i+1);
                    ans.emplace_back(i+1);
                    a[i]+=k;
                    a[i+1]+=k;
                }
            } else {
                if(i==0){
                    ans.emplace_back(i+2);
                    ans.emplace_back(i+2);
                    a[i+1]+=k;
                    a[i+2]+=k;                    
                } else {
                    ans.emplace_back(i);
                    ans.emplace_back(i);
                    a[i-1]+=k;
                    a[i]+=k;

                    ans.emplace_back(i+1);
                    a[i+1]+=k;

                    swap(a[i], a[i+1]);

                    
                    while(a[i-1]>a[i]){
                        ans.emplace_back(i+1);
                        ans.emplace_back(i+1);
                        a[i]+=k;
                        a[i+1]+=k;
                    }
                }
            }
        }
    }
    cout << "Yes\n";
    cout << ans.size() << '\n';
    rep(i,ans.size()) cout << ans[i] << ((i+1==ans.size()?'\n':' '));
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


