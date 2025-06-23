#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;
// using namespace atcoder;

#ifdef ADRY
#include <dbg.h>
#else
// DO NOTHING
#define dbg(...)
#endif

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

vll gen() {
    std::random_device seed_gen;
    std::mt19937 engine(seed_gen());
    int n = engine() % 20 + 3;
    vll a(n);
    rep(i,n) a[i] = engine() % 3;
    return a;
}

int cnt[3] = {0,0,0};

bool exp(vll a) {
    int n = a.size();
    rep(i,3) cnt[i] = 0;
    bool found = true;
    rep(i,n) {
        cnt[a[i]]++;
        int c = 0;
        if(a[(n+i-1)%n]<a[i])c++;
        if(a[(i+1)%n]<a[i])c++;
        if(a[i]!=c) {
            found = false;
        }
    }
    // if(found) {
    //     rep(i,3) cout << cnt[i] << " \n"[i+1==3];
    //     rep(i,n) cout << a[i] << " \n"[i+1==n];
    //     cout << "\n====================\n";
    // }
    return found;
}

/*
0の数は十分あれば適当に並べればOKなのでそこまで気にしなくてもいい
1, 2の数で条件づけ出来そう

x=1, y=1, z=1がYESなのか

2の配置はかなり制限される 

2 ? 2とおくとして、それだと?には0しか入らない

2 0 2 0 ... 0 2と埋めるのが良さそう
1の挿入可能位置ってもしかして2 1 0 1 2にするだけ？

自由度が高すぎるので別で代替できることを言いたい

2 0 2 0 2 1 0 1 1 0 1
上の形かな
2 1 0 1 2 1 0 1 2 ...
のほうが良くね？1は省略可能として

間に1単体は入れられないから、
- z-1<=x
が必要
↑を満たしたうえで、
1が残る個数は最小でmax(y-(z-1)*2,0)個, 最大でy個 => left_y
0が残る個数はx-(z-1)個 => left_z

i) z>0の場合: すなわち、左端に2がある場合
1終わりでも0終わりでも良い？

2 1 0 1
2 1 0
うん、いいね
1の残る個数も最小を取るとして、left_y<=left_z+1が必要かな

↑は嘘 ii)の議論参照
奇数個でもいいけど、

left_yき数個
2 1 1 0 1
で、left_y/2<=left_zが必要

left_yが偶数個
2 1 1 0 1 1 0
で、やっぱりleft_y/2<=left_zが必要

おい大嘘じゃねぇか
2 1 0 1 1 0 → 奇数個OK
2 1 0 1 1 0 1 → 偶数個もOK？
境界でミスってる？

ii) z=0の場合
1 0 1

y = 1はNoでOK
1終わりじゃないと駄目

2 4 0
で構成できる場合がある？
1 1 0 1 1 0
1 1 0 1 1 0 1

yが奇数のときって構成できない？無理で良さそう。実験的にはそう言ってるしなぁ
yが偶数かつy<=x*2かな？

iii) どれか２つがない場合:
xけあるパターンは構成可能

1 2 2がYesって何に落ちてるんだ
2 1 0 1 2みたいなパターンがNGか で、これは0が足りないので直したが...

2 1 0 1 2 1 0 1 2
2 1 0 1 2 1 0 1 2 0

2 1 1 0
*/


void solve() {
    ll x, y, z; cin >> x >> y >> z;
    if(z==0){
        if(y%2==0&&y/2<=x) {
            cout << "Yes\n";
        } else {
            cout << "No\n";
        }
    } else {
        if(z-1 <= x) {
            ll left_y = max(y-(z-1)*2,0ll);
            ll left_z = x-(z-1);
            dbg(left_y, left_z);
            if(left_y/2 + left_y%2 <=left_z && left_z > 0) {
                cout << "Yes\n";
            } else {
                cout << "No\n";
            }
        } else {
            cout << "No\n";
            return;
        }
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    // while(t--) exp();
    while(t--)solve();
}


