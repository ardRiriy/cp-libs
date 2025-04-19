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

#define rep(i,n) for (ll i=0;i<n;++i)
#define per(i,n) for(ll i=n-1;i>=0;--i)
#define rep2(i,a,n) for (ll i=a;i<n;++i)
#define per2(i,a,n) for (ll i=n-1;i>=a;--i)


template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};


/* thank you! : https://nyaannyaan.github.io/library/segment-tree/persistent-segment-tree.hpp.html */
template <typename T, typename F, int NODES = 20000000>
struct PersistentSegmentTree {
  using ll = long long;
  struct Node {
    T data;
    Node *l, *r;
    Node() {}
    Node(const T &_data) : data(_data), l(nullptr), r(nullptr) {}
  };

  Node *pool;
  int pid;
  ll N;
  const F f;
  const T ID;
  Node *nil;
  vector<Node *> roots;

  PersistentSegmentTree(const vector<T> &v, const F &f_, const T &ID_)
      : pid(0), f(f_), ID(ID_), nil(nullptr) {
    pool = new Node[NODES];
    nil = my_new(ID);
    nil->l = nil->r = nil;
    roots.reserve(262144);
    roots.push_back(build(v));
  }

  PersistentSegmentTree(const ll &N_, const F &f_, const T &ID_)
      : pid(0), N(N_), f(f_), ID(ID_), nil(nullptr) {
    pool = new Node[NODES];
    nil = my_new(ID);
    nil->l = nil->r = nil;
    roots.reserve(262144);
    roots.push_back(nil);
  }

  Node *my_new(const T &data) {
    pool[pid].data = data;
    pool[pid].l = pool[pid].r = nil;
    return &(pool[pid++]);
  }

  Node *merge(Node *l, Node *r) {
    pool[pid].data = f(l->data, r->data);
    pool[pid].l = l;
    pool[pid].r = r;
    return &(pool[pid++]);
  }

  Node *build(const vector<T> &v) {
    N = (ll)v.size();
    return build(0, (ll)v.size(), v);
  }

  Node *build(ll l, ll r, const vector<T> &v) {
    if (l + 1 == r) return my_new(v[l]);
    ll m = (l + r) >> 1;
    return merge(build(l, m, v), build(m, r, v));
  }

 private:
  Node *update_(ll a, const T &x, Node *n, ll l, ll r) {
    if (l + 1 == r) return my_new(x);
    ll m = (l + r) >> 1;
    if (a < m) return merge(update_(a, x, n->l, l, m), n->r);
    return merge(n->l, update_(a, x, n->r, m, r));
  }
  Node *add_(ll a, const T &x, Node *n, ll l, ll r) {
    if (l + 1 == r) return my_new(f(x, n->data));
    ll m = (l + r) >> 1;
    if (a < m) return merge(add_(a, x, n->l, l, m), n->r);
    return merge(n->l, add_(a, x, n->r, m, r));
  }
  T query_(ll a, ll b, Node *n, ll l, ll r) {
    if (n == nil) return ID;
    if (r <= a or b <= l) return ID;
    if (a <= l and r <= b) return n->data;
    ll m = (l + r) >> 1;
    return f(query_(a, b, n->l, l, m), query_(a, b, n->r, m, r));
  }

 public:
  Node *update(Node *n, ll k, const T &x) {
    Node *root = update_(k, x, n, 0, N);
    roots.push_back(root);
    return root;
  }
  Node *update(int t, ll k, const T &x) {
    Node *root = update_(k, x, roots[t], 0, N);
    roots.push_back(root);
    return root;
  }
  Node *update(ll k, const T &x) {
    Node *root = update_(k, x, roots.back(), 0, N);
    roots.push_back(root);
    return root;
  }

  Node *add(Node *n, ll k, const T &x) {
    Node *root = add_(k, x, n, 0, N);
    roots.push_back(root);
    return root;
  }
  Node *add(int t, ll k, const T &x) {
    Node *root = add_(k, x, roots[t], 0, N);
    roots.push_back(root);
    return root;
  }
  Node *add(ll k, const T &x) {
    Node *root = add_(k, x, roots.back(), 0, N);
    roots.push_back(root);
    return root;
  }

  T query(Node *n, ll a, ll b) { return query_(a, b, n, 0, N); }
  T query(int t, ll a, ll b) { return query_(a, b, roots[t], 0, N); }
  T query(ll a, ll b) { return query_(a, b, roots.back(), 0, N); }

  Node *new_tree() { return nil; }
};

/**
 * @brief 永続セグメント木
 * @docs docs/segment-tree/persistent-segtree.md
 */

void solve() {
    int n; cin >> n;
    auto a = i64_vec_IN(n);
    
    PersistentSegmentTree<ll, function<ll(ll,ll)>, 6000000> seg(n, [](ll a, ll b) { return a+b;}, 0);
    
    vll indicates(n); iota(all(indicates),0);
    sort(all(indicates), [&](const ll&i, const ll&j) {
        return a[i]<a[j];
    });

    map<ll,ll> mp; // val => 最終更新位置
    rep(i,n) {
        seg.update(indicates[i], a[indicates[i]]);
        mp[a[indicates[i]]] = i;
    }

    ll b = 0;
    int q; cin >> q;
    ll alpha, beta, gannma;
    rep(_, q) {
        cin >> alpha >> beta >> gannma;
        ll l = alpha ^ b;
        ll r = beta ^ b;
        ll x = gannma ^ b;

        // x以下で最大の値をとる
        ll ok = -1;
        ll ng = n;
        while(abs(ok-ng)>1){
            ll mid = (ok+ng)>>1;
            if(a[indicates[mid]]<=x){
                ok = mid;
            } else {
                ng = mid;
            }
        }
        if(ok==-1){
            b=0;
        } else {
            ll time = mp[a[indicates[ok]]];
            b = seg.query(time+1, l-1, r);
        }
        cout << b << '\n';
    }
}

int main() {
    // cin.tie(0);
    // ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


