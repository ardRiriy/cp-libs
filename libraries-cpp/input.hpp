#ifndef INPUT_HPP
#define INPUT_HPP
#include <iostream>
#include <vector>
#include <string>
using namespace std;
vector<long long> i64_vec_IN(int n) {vector<long long> res(n); for(int i=0; i<n; i++) { cin >> res[i]; } return res; }
vector<string> str_vec_IN(int n) { vector<string> res(n); for(int i=0; i<n; i++) { cin >> res[i]; } return res; }
vector<vector<int>> graph_IN(int n, int m) { vector<vector<int>> g(n); int u, v; for(int i=0; i<m; i++) { cin >> u >> v; u--; v--; g[u].emplace_back(v); g[v].emplace_back(u); } return g; }
vector<vector<pair<int, long long>>> weighted_graph_IN(int n, int m) { vector<vector<pair<int, long long>>> g(n); int u, v; long long w; for(int i=0; i<m; i++) { cin >> u >> v >> w; u--; v--; g[u].push_back({v, w}); g[v].push_back({u, w}); } return g; }

#endif
