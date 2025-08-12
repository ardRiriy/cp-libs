#include <bits/stdc++.h>

using namespace std;

void solve() {
    int n; cin >> n;
    vector<char> ans(n);
    for(int i=0; i<n; i+=2) {
        cout << "? 2 " << i+1 << " " << i+2 << endl;
        int res; cin >> res;
        if (res == 0) {
            ans[i] = ')';
            ans[i+1] = '(';
        } else {
            ans[i+1] = ')';
            ans[i] = '(';
        }
    }
    
    if(n%2==1) {
        cout << "? 2 " << n-1 << " " << n << endl;
        int res; cin >> res;
        ans[n-1] = (res == 1) ? ')' : '(';
    }

    cout << "! ";
    for(int i=0; i<n; i++) cout << ans[i];
    cout << endl;
}

int main() {
    int t=1;
    cin >> t;
    while(t--)solve();
}
