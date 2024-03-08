cargo compete new $1
cd $1

# add opt-level to Cargo.toml
echo '
[profile.dev]
opt-level = 3' >> Cargo.toml

# cargo compete test a
