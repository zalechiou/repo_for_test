# PyO3 Multithread Demo

展示如何用 PyO3 撰寫 Python 擴充模組，解決 GIL 限制與整合 NumPy。

## 安裝

```bash
# 安裝 Rust 開發環境：
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# 安裝 maturin
pip install maturin
# 編譯並安裝 Rust 擴展
maturin develop
# 執行測試
python test.py
```
