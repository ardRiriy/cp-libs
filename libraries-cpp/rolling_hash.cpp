// thanks: https://kyoroid.github.io/algorithm/string/rolling_hash.html
class RollingHash{
    private:
        const __uint128_t modulo = (1LL << 62) - 1;
        __uint128_t base;
        vector<__uint128_t> prefix;
        vector<__uint128_t> power;
        int n;

    public:
        RollingHash(string& S, long long base = 0) {
            random_device seed_gen;
            mt19937 engine(seed_gen());

            if(base==0)base = engine();
            n = S.size();

            prefix = vector<__uint128_t>(n+1, 0);
            power = vector<__uint128_t>(n+1, 1);
            for(int i = 0; i<n; i++) {
                int c = S[i];
                prefix[i+1] = (prefix[i] * base + c) % modulo;
                power[i+1] = (power[i] * base) % modulo;
            }
        }
    
        long long hash(int l, int r) {
            __uint128_t res = prefix[r] + modulo - ((power[r-l] * prefix[l]) % modulo);
            res %= modulo;
            return (long long)res;
        }
};

