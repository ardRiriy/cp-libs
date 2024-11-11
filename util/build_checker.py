import os
import subprocess
import sys

# 対象のディレクトリを指定
root_dir = "../contests/"

# 再帰的にすべてのディレクトリを探索し、Cargo.tomlが存在するディレクトリを見つける
for dirpath, _, filenames in os.walk(root_dir):
    if "Cargo.toml" in filenames:
        print(f"Building in directory: {dirpath}")
        try:
            # `cargo build` コマンドを実行
            result = subprocess.run(
                ["cargo", "build"],
                cwd=dirpath,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                check=True
            )
            print(f"Build succeeded in: {dirpath}")
        except subprocess.CalledProcessError as e:
            # ビルドが失敗した場合、そのディレクトリを標準エラー出力に出力
            sys.stderr.write(f"Build failed in directory: {dirpath}\n")
