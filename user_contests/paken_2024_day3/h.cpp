#include<bits/stdc++.h>
//#include <atcoder/all>
using namespace std;
//using namespace atcoder;

#define all(v) v.begin(),v.end()
using ll = long long;
using ull = unsigned long long;
using lll = __int128;
using vll=vector<ll>;
using vvll = vector<vector<ll>>;
using P = pair<ll,ll>;
using vp=vector<pair<ll, ll>>;
//using mint=modint1000000007;
//using mint=modint998244353;

const ll INF=1ll<<60;
ll mod10=1e9+7;
ll mod99=998244353;
const double PI = acos(-1);

#define rep(i,n) for (ll i=0;i<n;++i)
#define per(i,n) for(ll i=n-1;i>=0;--i)
#define rep2(i,a,n) for (ll i=a;i<n;++i)
#define per2(i,a,n) for (ll i=a;i>=n;--i)

template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

bool solve(){
	ll N;cin>>N;
	vll A(N);rep(i,N) cin>>A[i];
	vll cnt(100);
	rep(i,N){
		rep(j,63) if(A[i]>>j&1) cnt[j]++;
	}
	
	ll ind=-1;
	rep(i,100) if(cnt[i]%2==1) ind=i;
	if(ind==-1){
		cout << "Draw" << endl;
		return 0;
	}
	if(cnt[ind]==1){
		cout << "Alice" << endl;
		return 0;
	}
	if(cnt[ind]==N){
		cout<<(cnt[ind]%4==1?"Alice":"Bob")<<endl;
		return 0;
	}
	cout << (((N-1)%2==1)?"Alice":"Bob") << endl;
	return 0;
}




int main(){
   cin.tie(0);
   ios::sync_with_stdio(false);
   ll T=1;cin>>T;
   rep(i,T) solve();
}
	
