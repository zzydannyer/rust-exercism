# Exercism Rust 刷题集

官方初始代码及测试，来源：https://github.com/exercism/rust

同步版本：`cc099ae090a6acbe6f53f1e36325263046ba333e`（2026-10-18，America/Los_Angeles）。共 117 题：106 道 practice、11 道 concept，包含已弃用和 WIP 题；不代表网站当前开放题数。未导入参考答案。

## 开始刷题

推荐使用顺序练习工具（按官方 practice 题目、难度从低到高）：

```sh
cargo ex start      # 从第一道未通过的题开始，展示说明/教程/提示并跑测试
cargo ex watch      # 监听当前题源码变化，保存后自动重测；通过后进入下一题
cargo ex concept    # 单独练习 concept 题（同样展示教程和提示）
cargo ex list       # 列出题目顺序与难度
cargo ex show       # 只展示当前题的说明文档
```

流程：修改当前题的 `src/lib.rs` → 测试通过 → 自动进入下一题。进度会显示 `已完成/总数`。

也可以手动测单题：

```sh
cargo test -p hello_world
cargo test -p reverse_string
# 连同默认被忽略的测试一起运行
cargo test -p reverse_string -- --include-ignored
# 启用题目的扩展挑战
cargo test -p reverse_string --features grapheme -- --include-ignored
```

根目录直接执行 `cargo test` 默认只测 hello-world。也可以进入某题目录运行 `cargo test`。

初始代码故意未实现，测试失败甚至编译失败都可能是正常的（例如 macros）。不建议一开始就 `cargo test --workspace`。文档测试中的 `compile_fail,ignore` 需要按照题目说明移除 `,ignore` 来启用；普通 `#[ignore]` 测试可以用上面的参数运行。仅使用 `--all-targets` 不包含文档测试。

每题有独立 `Cargo.toml`，共享根目录的 `Cargo.lock` 和 `target/`。需要支持 edition 2024 的 Rust 工具链（Rust 1.85 或更高版本）。多数练习是库，没有 main.rs，使用 `cargo test` 而不是 `cargo run`。

## 网站提交

这是官方仓库源码导入，不是账号专属下载，不包含伪造的 `.exercism/metadata.json`。需要提交到个人账号时，用已登录的 Exercism CLI 在另一个目录下载对应题目，再把本地解答文件复制过去并提交。不要用上游重复导入覆盖已经写好的解答。

目录名使用官方 slug（如 hello-world），Cargo 包名以各题 Cargo.toml 为准（通常为 hello_world，但 two-fer 的包名是 twofer）。概念题在上游配置中尚未整体启用，保留作额外练习。

## 题目索引

以下按官方配置顺序列出。普通 practice 未标记 status 的条目不额外标注；deprecated 为已弃用，wip 为尚未完成。

### practice

- [hello-world](exercises/practice/hello-world/.docs/instructions.md) · 难度 1
- [reverse-string](exercises/practice/reverse-string/.docs/instructions.md) · 难度 1
- [gigasecond](exercises/practice/gigasecond/.docs/instructions.md) · 难度 1
- [clock](exercises/practice/clock/.docs/instructions.md) · 难度 4
- [anagram](exercises/practice/anagram/.docs/instructions.md) · 难度 4
- [space-age](exercises/practice/space-age/.docs/instructions.md) · 难度 7
- [sublist](exercises/practice/sublist/.docs/instructions.md) · 难度 7
- [flower-field](exercises/practice/flower-field/.docs/instructions.md) · 难度 7
- [minesweeper](exercises/practice/minesweeper/.docs/instructions.md) · 难度 7 · deprecated
- [luhn](exercises/practice/luhn/.docs/instructions.md) · 难度 7
- [armstrong-numbers](exercises/practice/armstrong-numbers/.docs/instructions.md) · 难度 1
- [beer-song](exercises/practice/beer-song/.docs/instructions.md) · 难度 1 · deprecated
- [bottle-song](exercises/practice/bottle-song/.docs/instructions.md) · 难度 1
- [difference-of-squares](exercises/practice/difference-of-squares/.docs/instructions.md) · 难度 1
- [grains](exercises/practice/grains/.docs/instructions.md) · 难度 1
- [leap](exercises/practice/leap/.docs/instructions.md) · 难度 1
- [nth-prime](exercises/practice/nth-prime/.docs/instructions.md) · 难度 1
- [prime-factors](exercises/practice/prime-factors/.docs/instructions.md) · 难度 1
- [proverb](exercises/practice/proverb/.docs/instructions.md) · 难度 1
- [raindrops](exercises/practice/raindrops/.docs/instructions.md) · 难度 1
- [sum-of-multiples](exercises/practice/sum-of-multiples/.docs/instructions.md) · 难度 1
- [bob](exercises/practice/bob/.docs/instructions.md) · 难度 1
- [high-scores](exercises/practice/high-scores/.docs/instructions.md) · 难度 1
- [matching-brackets](exercises/practice/matching-brackets/.docs/instructions.md) · 难度 1
- [collatz-conjecture](exercises/practice/collatz-conjecture/.docs/instructions.md) · 难度 1
- [diffie-hellman](exercises/practice/diffie-hellman/.docs/instructions.md) · 难度 1 · deprecated
- [series](exercises/practice/series/.docs/instructions.md) · 难度 1
- [hexadecimal](exercises/practice/hexadecimal/.docs/instructions.md) · 难度 1 · deprecated
- [nucleotide-codons](exercises/practice/nucleotide-codons/.docs/instructions.md) · 难度 1 · deprecated
- [two-fer](exercises/practice/two-fer/.docs/instructions.md) · 难度 1 · deprecated
- [kindergarten-garden](exercises/practice/kindergarten-garden/.docs/instructions.md) · 难度 1
- [eliuds-eggs](exercises/practice/eliuds-eggs/.docs/instructions.md) · 难度 1
- [acronym](exercises/practice/acronym/.docs/instructions.md) · 难度 4
- [all-your-base](exercises/practice/all-your-base/.docs/instructions.md) · 难度 4
- [allergies](exercises/practice/allergies/.docs/instructions.md) · 难度 4
- [alphametics](exercises/practice/alphametics/.docs/instructions.md) · 难度 4
- [binary-search](exercises/practice/binary-search/.docs/instructions.md) · 难度 4
- [bowling](exercises/practice/bowling/.docs/instructions.md) · 难度 4
- [dot-dsl](exercises/practice/dot-dsl/.docs/instructions.md) · 难度 4
- [etl](exercises/practice/etl/.docs/instructions.md) · 难度 1
- [grade-school](exercises/practice/grade-school/.docs/instructions.md) · 难度 4
- [hamming](exercises/practice/hamming/.docs/instructions.md) · 难度 4
- [isbn-verifier](exercises/practice/isbn-verifier/.docs/instructions.md) · 难度 4
- [isogram](exercises/practice/isogram/.docs/instructions.md) · 难度 4
- [nucleotide-count](exercises/practice/nucleotide-count/.docs/instructions.md) · 难度 4
- [paasio](exercises/practice/paasio/.docs/instructions.md) · 难度 4
- [palindrome-products](exercises/practice/palindrome-products/.docs/instructions.md) · 难度 4
- [pangram](exercises/practice/pangram/.docs/instructions.md) · 难度 4
- [pascals-triangle](exercises/practice/pascals-triangle/.docs/instructions.md) · 难度 4
- [perfect-numbers](exercises/practice/perfect-numbers/.docs/instructions.md) · 难度 4
- [pig-latin](exercises/practice/pig-latin/.docs/instructions.md) · 难度 4
- [queen-attack](exercises/practice/queen-attack/.docs/instructions.md) · 难度 4
- [rna-transcription](exercises/practice/rna-transcription/.docs/instructions.md) · 难度 4
- [run-length-encoding](exercises/practice/run-length-encoding/.docs/instructions.md) · 难度 4
- [saddle-points](exercises/practice/saddle-points/.docs/instructions.md) · 难度 4
- [say](exercises/practice/say/.docs/instructions.md) · 难度 4
- [scrabble-score](exercises/practice/scrabble-score/.docs/instructions.md) · 难度 4
- [sieve](exercises/practice/sieve/.docs/instructions.md) · 难度 4
- [simple-linked-list](exercises/practice/simple-linked-list/.docs/instructions.md) · 难度 4
- [spiral-matrix](exercises/practice/spiral-matrix/.docs/instructions.md) · 难度 4
- [tournament](exercises/practice/tournament/.docs/instructions.md) · 难度 4
- [triangle](exercises/practice/triangle/.docs/instructions.md) · 难度 4
- [two-bucket](exercises/practice/two-bucket/.docs/instructions.md) · 难度 4
- [variable-length-quantity](exercises/practice/variable-length-quantity/.docs/instructions.md) · 难度 4
- [robot-simulator](exercises/practice/robot-simulator/.docs/instructions.md) · 难度 7
- [robot-name](exercises/practice/robot-name/.docs/instructions.md) · 难度 4
- [protein-translation](exercises/practice/protein-translation/.docs/instructions.md) · 难度 7
- [wordy](exercises/practice/wordy/.docs/instructions.md) · 难度 4
- [custom-set](exercises/practice/custom-set/.docs/instructions.md) · 难度 4
- [accumulate](exercises/practice/accumulate/.docs/instructions.md) · 难度 4
- [affine-cipher](exercises/practice/affine-cipher/.docs/instructions.md) · 难度 4
- [atbash-cipher](exercises/practice/atbash-cipher/.docs/instructions.md) · 难度 4
- [crypto-square](exercises/practice/crypto-square/.docs/instructions.md) · 难度 4
- [diamond](exercises/practice/diamond/.docs/instructions.md) · 难度 4
- [largest-series-product](exercises/practice/largest-series-product/.docs/instructions.md) · 难度 4
- [luhn-from](exercises/practice/luhn-from/.docs/instructions.md) · 难度 4
- [luhn-trait](exercises/practice/luhn-trait/.docs/instructions.md) · 难度 4
- [list-ops](exercises/practice/list-ops/.docs/instructions.md) · 难度 4
- [phone-number](exercises/practice/phone-number/.docs/instructions.md) · 难度 4
- [rail-fence-cipher](exercises/practice/rail-fence-cipher/.docs/instructions.md) · 难度 4
- [roman-numerals](exercises/practice/roman-numerals/.docs/instructions.md) · 难度 4
- [rotational-cipher](exercises/practice/rotational-cipher/.docs/instructions.md) · 难度 4
- [simple-cipher](exercises/practice/simple-cipher/.docs/instructions.md) · 难度 4
- [word-count](exercises/practice/word-count/.docs/instructions.md) · 难度 4
- [secret-handshake](exercises/practice/secret-handshake/.docs/instructions.md) · 难度 4
- [knapsack](exercises/practice/knapsack/.docs/instructions.md) · 难度 4
- [yacht](exercises/practice/yacht/.docs/instructions.md) · 难度 4
- [matrix](exercises/practice/matrix/.docs/instructions.md) · 难度 4
- [fizzy](exercises/practice/fizzy/.docs/instructions.md) · 难度 7
- [pythagorean-triplet](exercises/practice/pythagorean-triplet/.docs/instructions.md) · 难度 7
- [parallel-letter-frequency](exercises/practice/parallel-letter-frequency/.docs/instructions.md) · 难度 10
- [macros](exercises/practice/macros/.docs/instructions.md) · 难度 10
- [pov](exercises/practice/pov/.docs/instructions.md) · 难度 10
- [poker](exercises/practice/poker/.docs/instructions.md) · 难度 10
- [forth](exercises/practice/forth/.docs/instructions.md) · 难度 10
- [ocr-numbers](exercises/practice/ocr-numbers/.docs/instructions.md) · 难度 10
- [react](exercises/practice/react/.docs/instructions.md) · 难度 10
- [circular-buffer](exercises/practice/circular-buffer/.docs/instructions.md) · 难度 10
- [rectangles](exercises/practice/rectangles/.docs/instructions.md) · 难度 10
- [xorcism](exercises/practice/xorcism/.docs/instructions.md) · 难度 10
- [book-store](exercises/practice/book-store/.docs/instructions.md) · 难度 7
- [decimal](exercises/practice/decimal/.docs/instructions.md) · 难度 7
- [grep](exercises/practice/grep/.docs/instructions.md) · 难度 7
- [scale-generator](exercises/practice/scale-generator/.docs/instructions.md) · 难度 7 · deprecated
- [dominoes](exercises/practice/dominoes/.docs/instructions.md) · 难度 10
- [doubly-linked-list](exercises/practice/doubly-linked-list/.docs/instructions.md) · 难度 10

### concept

- [lucians-luscious-lasagna](exercises/concept/lucians-luscious-lasagna/.docs/instructions.md) · 难度 1 · active
- [magazine-cutout](exercises/concept/magazine-cutout/.docs/instructions.md) · 难度 1 · active
- [assembly-line](exercises/concept/assembly-line/.docs/instructions.md) · 难度 1 · active
- [semi-structured-logs](exercises/concept/semi-structured-logs/.docs/instructions.md) · 难度 1 · active
- [resistor-color](exercises/concept/resistor-color/.docs/instructions.md) · 难度 1 · active
- [health-statistics](exercises/concept/health-statistics/.docs/instructions.md) · 难度 1 · active
- [role-playing-game](exercises/concept/role-playing-game/.docs/instructions.md) · 难度 1 · active
- [low-power-embedded-game](exercises/concept/low-power-embedded-game/.docs/instructions.md) · 难度 1 · active
- [short-fibonacci](exercises/concept/short-fibonacci/.docs/instructions.md) · 难度 1 · active
- [rpn-calculator](exercises/concept/rpn-calculator/.docs/instructions.md) · 难度 4 · active
- [csv-builder](exercises/concept/csv-builder/.docs/instructions.md) · 难度 1 · wip
