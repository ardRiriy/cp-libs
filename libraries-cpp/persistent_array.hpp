#ifndef PERSISTENT_ARRAY_HPP
#define PERSISTENT_ARRAY_HPP
#include <vector>
#include <string>
#include <cstring>
#include <cassert>

using namespace std;

constexpr int DIV = 20; // 分割数

template<typename T>
class PersistentArray {
private:
    struct Node {
        T value;
        Node* ch[DIV] = {};
        Node() {}
        Node(T val) : value(val) {}
    };
        
    T _get(int idx, Node* t) {
        if(!t) assert(false);
        if(idx == 0) {
            return t->value;
        } else {
            return _get(idx/DIV, t->ch[idx%DIV]);
        }
    }

    Node* _set(int idx, T val, Node* t) {
        /* t: 過去のデータ */
        Node* res = new Node();
        if(t) {
            // 過去データが存在する場合はコピーしておく
            memcpy(res->ch, t->ch, sizeof(t->ch));
            res->value = t->value;
        }
        
        if(idx == 0) {
            res->value = val;
        } else {
            res->ch[idx%DIV] = _set(idx/DIV, val, t ? t->ch[idx%DIV] : nullptr);
        }
        
        return res;
    }
    
public:
    vector<Node*> parents;

    // デフォルトコンストラクタ
    PersistentArray() {}
    
    // 配列から構築するコンストラクタ
    PersistentArray(vector<T>& a) {
        // 配列Aの永続配列を作成
        Node* p = nullptr;
        for(size_t i = 0; i < a.size(); i++) {
            p = _set(i, a[i], p);
        }
        if(p) {
            parents.push_back(p);
        }
    }
    
    void set(int idx, T val) {
        Node* ret = _set(idx, val, (parents.size())?parents[parents.size()-1]:nullptr);
        parents.push_back(ret);
    }

    /* 時刻Tにおけるidxの値を返却する */
    T get(int idx, int t) {
        assert(t<parents.size());
        return _get(idx, parents[t]);
    }
};

# endif // PERSISTENT_ARRAY_HPP