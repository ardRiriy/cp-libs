import os
import re
import sys

def process_rust_file(source_file_name, cps_dir='../../cps/src/'):
    # カレントディレクトリからの相対パスでファイルパスを構築
    current_dir = os.getcwd()
    source_file_path = os.path.join(current_dir, 'src/bin', f'{source_file_name}.rs')
    cps_dir = os.path.join(current_dir, cps_dir)

    # ファイルを読み込む
    with open(source_file_path, 'r') as file:
        lines = file.readlines()

    # 元のファイルを../temporaries ディレクトリに保存
    temp_dir = os.path.join(current_dir, '../temporaries')
    os.makedirs(temp_dir, exist_ok=True)
    temp_file_path = os.path.join(temp_dir, f'{source_file_name}.rs')
    with open(temp_file_path, 'w') as file:
        file.writelines(lines)


    # 処理するための変数を初期化
    new_lines = []
    modules_to_append = []

    # 各行を確認して、必要な変更を行う
    for line in lines:
        if line.startswith('use cps::'):
            if line == 'use cps::lg;':
                # lgマクロはdebugに内包されているのでcontinueしておく
                continue
            # モジュール名と関数名を抽出
            match = re.match(r'use cps::(\w+)::.*;', line)
            if match:
                mod_name, = match.groups()
                mod_file_path = os.path.join(cps_dir, f"{mod_name}.rs")
                # モジュールファイルが存在するか確認
                if os.path.exists(mod_file_path):
                    with open(mod_file_path, 'r') as mod_file:
                        modules_to_append.append(mod_file.read())
                # 元の行は追加しない
                continue
        else:
            # 通常の行はそのまま追加
            new_lines.append(line)

    # モジュールの内容をファイルの末尾に追加
    new_lines.extend(modules_to_append)

    # 新しい内容を元のファイルに上書き保存
    with open(source_file_path, 'w') as file:
        file.writelines(new_lines)

    print(f"File {source_file_path} has been updated.")

# 実行時引数からファイル名を取得
if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python script.py <file_name_without_extension>")
        sys.exit(1)
    file_name = sys.argv[1]
    process_rust_file(file_name)
