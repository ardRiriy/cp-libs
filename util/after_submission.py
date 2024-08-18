import os
import re
import sys

def process_rust_file(source_file_name):
    # カレントディレクトリからの相対パスでファイルパスを構築
    current_dir = os.getcwd()
    source_file_path = os.path.join(current_dir, '../temporaries', f'{source_file_name}.rs')

    # ファイルを読み込む
    with open(source_file_path, 'r') as file:
        lines = file.readlines()

    # src/bin ディレクトリにファイルを上書き保存
    update_file_path = os.path.join(current_dir, 'src/bin', f'{source_file_name}.rs')
    with open(update_file_path, 'w') as file:
        file.writelines(lines)

    # temporaries ディレクトリのファイルを削除
    os.remove(source_file_path)

# 実行時引数からファイル名を取得
if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python script.py <file_name_without_extension>")
        sys.exit(1)
    file_name = sys.argv[1]
    process_rust_file(file_name)
