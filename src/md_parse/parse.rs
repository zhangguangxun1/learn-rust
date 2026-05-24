// #### （1）题型 S-3.1.3.1【已知字母的值求代数式的值】
//
// ##### 母题 1
//
// 当 $x = 2$ 时，代数式 $2x + 1$ 的值是（   ）
//
// A．3　　B．5　　C．4　　D．6
//
// | 难度  | 适用学期 | 题目类型   |
// | :---- | :------- | :--------- |
// | 【1】 | 【71】   | 【选择题】 |
//
// **涉及知识点：** 【求代数式的值】
//
// **参考答案：** 【B】
//
// **【分析】** 本题考查了代数式求值。将 $x = 2$ 代入代数式 $2x + 1$ 直接计算即可。
//
// **【详解】** 解：∵ $x = 2$，
// ∴ $2x + 1 = 2 \times 2 + 1 = 4 + 1 = 5$。
// 故选：B。
//
// ---
//
// ##### 变式 1
//
// 当 $x = -1$ 时，代数式 $2x - 2$ 的值为（   ）
//
// A．2　　B．$-2$　　C．4　　D．$-4$
//
// | 难度  | 适用学期 | 题目类型   |
// | :---- | :------- | :--------- |
// | 【1】 | 【71】   | 【选择题】 |
//
// **涉及知识点：** 【求代数式的值】
//
// **参考答案：** 【D】
//
// **【分析】** 本题考查了求代数式的值，将 $x = -1$ 代入所求代数式计算即可得解。
//
// **【详解】** 解：当 $x = -1$ 时，代数式 $2x - 2 = 2 \times (-1) - 2 = -2 - 2 = -4$。
// 故选：D。
//
// ---

// Start: Heading { level: H5, id: None, classes: [], attrs: [] }
// Other: Text(Borrowed("母题 1"))
// End: Heading(H5)
// Start: Paragraph
// Other: Text(Borrowed("当 $x = 2$ 时，代数式 $2x + 1$ 的值是（\u{a0}\u{a0}\u{a0}）"))
// End: Paragraph
// Start: Paragraph
// Other: Text(Borrowed("A．3\u{3000}\u{3000}B．5\u{3000}\u{3000}C．4\u{3000}\u{3000}D．6"))
// End: Paragraph
// Start: Paragraph
// Other: Text(Borrowed("| 难度  | 适用学期 | 题目类型   |"))
// Other: SoftBreak
// Other: Text(Borrowed("| :---- | :------- | :--------- |"))
// Other: SoftBreak
// Other: Text(Borrowed("| 【1】 | 【71】   | 【选择题】 |"))
// End: Paragraph
// Start: Paragraph
// Start: Strong
// Other: Text(Borrowed("涉及知识点："))
// End: Strong
// Other: Text(Borrowed(" 【求代数式的值】"))
// End: Paragraph
// Start: Paragraph
// Start: Strong
// Other: Text(Borrowed("参考答案："))
// End: Strong
// Other: Text(Borrowed(" 【B】"))
// End: Paragraph
// Start: Paragraph
// Start: Strong
// Other: Text(Borrowed("【分析】"))
// End: Strong
// Other: Text(Borrowed(" 本题考查了代数式求值。将 $x = 2$ 代入代数式 $2x + 1$ 直接计算即可。"))
// End: Paragraph
// Start: Paragraph
// Start: Strong
// Other: Text(Borrowed("【详解】"))
// End: Strong
// Other: Text(Borrowed(" 解：∵ $x = 2$，"))
// Other: SoftBreak
// Other: Text(Borrowed("∴ $2x + 1 = 2 \times 2 + 1 = 4 + 1 = 5$。"))
// Other: SoftBreak
// Other: Text(Borrowed("故选：B。"))
// End: Paragraph
// Other: Rule

#[cfg(test)]
mod tests {
    use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
    use regex::Regex;

    #[derive(Debug)]
    struct ParsedQuestion {
        id: usize,
        parent_id: Option<usize>, // 母题为None，变式为Some(mother_id)
        title: String,
        stem: String,
        choices: Vec<String>,
        table: Vec<Vec<String>>,
        knowledge: String,
        answer: String,
        analysis: String,
        detail: String,
        raw_markdown: String,
    }

    #[derive(Debug)]
    struct Mother {
        question: ParsedQuestion,
        variations: Vec<ParsedQuestion>,
    }

    // 一级：分母题
    fn split_mothers(text: &str) -> Vec<String> {
        let mut mothers = Vec::new();
        let mut buf = String::new();
        let mut started = false;
        for line in text.lines() {
            if line.trim_start().starts_with("##### 母题") {
                if started && !buf.trim().is_empty() {
                    mothers.push(buf.trim().to_string());
                    buf.clear();
                }
                started = true;
            }
            if started {
                buf.push_str(line);
                buf.push('\n');
            }
        }
        if !buf.trim().is_empty() {
            mothers.push(buf.trim().to_string());
        }
        mothers
    }

    // 二级：分所有H5标题（母题+变式）
    fn split_main_and_variations(block: &str) -> Vec<(String, String)> {
        let mut res = Vec::new();
        let mut buf = String::new();
        let mut title = String::new();
        for line in block.lines() {
            if line.trim_start().starts_with("##### ") {
                if !buf.is_empty() {
                    res.push((title.clone(), buf.trim().to_string()));
                    buf.clear();
                }
                title = line
                    .trim_start()
                    .trim_start_matches("#####")
                    .trim()
                    .to_string();
            }
            if !line.trim().is_empty() || line.trim_start().starts_with("##### ") {
                buf.push_str(line);
                buf.push('\n');
            }
        }
        if !buf.trim().is_empty() {
            res.push((title.clone(), buf.trim().to_string()));
        }
        res
    }

    // 题干&选项切割正则，兼容全角半角
    fn extract_choices_and_stem(text: &str) -> (String, Vec<String>) {
        let re = Regex::new(r"[A-D](?:\.|．)[^A-D　\n]+").unwrap();
        let choices: Vec<String> = re
            .find_iter(text)
            .map(|m| m.as_str().trim().to_string())
            .collect();

        let stem = re.replace_all(text, "").to_string();
        let stem = stem
            .trim()
            .replace("（  　）", "（    ）")
            .trim()
            .to_string();
        (stem, choices)
    }

    #[derive(PartialEq, Debug)]
    enum Section {
        None,
        Head5,
        Table,
        Knowledge,
        Answer,
        Analysis,
        Detail,
    }

    // 主Markdown->结构化题的解析
    fn parse_question(
        id: usize,
        parent_id: Option<usize>,
        title: String,
        markdown: &str,
    ) -> ParsedQuestion {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        let parser = Parser::new_ext(markdown, options);

        let mut state = Section::None;
        let mut head5 = String::new();
        let mut main_content = String::new();
        let mut table_cells: Vec<Vec<String>> = Vec::new();
        let mut current_row = Vec::new();
        let mut in_tablerow = false;

        let mut knowledge = String::new();
        let mut answer = String::new();
        let mut analysis = String::new();
        let mut detail = String::new();
        let mut in_strong = false;

        for event in parser {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    if level == pulldown_cmark::HeadingLevel::H5 {
                        state = Section::Head5;
                    }
                }
                Event::End(TagEnd::Heading { .. }) => {
                    if state == Section::Head5 {
                        state = Section::None;
                    }
                }
                Event::Start(Tag::Table(_)) => {
                    state = Section::Table;
                }
                Event::End(TagEnd::Table) => {
                    state = Section::None;
                }
                Event::Start(Tag::TableRow) => {
                    in_tablerow = true;
                    current_row = Vec::new();
                }
                Event::End(TagEnd::TableRow) => {
                    in_tablerow = false;
                    if !current_row.is_empty() {
                        table_cells.push(current_row.clone());
                    }
                }
                Event::Start(Tag::Strong) => {
                    in_strong = true;
                }
                Event::End(TagEnd::Strong) => {
                    in_strong = false;
                }
                Event::Text(t) => {
                    let s = t.trim();
                    if in_strong {
                        if s == "涉及知识点：" {
                            state = Section::Knowledge;
                            continue;
                        } else if s == "参考答案：" {
                            state = Section::Answer;
                            continue;
                        } else if s == "【分析】" {
                            state = Section::Analysis;
                            continue;
                        } else if s == "【详解】" {
                            state = Section::Detail;
                            continue;
                        }
                    }
                    match state {
                        Section::Head5 => head5.push_str(&format!(" {}", s)),
                        Section::Table => {
                            if in_tablerow {
                                current_row.push(s.to_string());
                            }
                        }
                        Section::Knowledge => knowledge.push_str(s),
                        Section::Answer => answer.push_str(s),
                        Section::Analysis => analysis.push_str(s),
                        Section::Detail => detail.push_str(s),
                        Section::None => {
                            main_content.push_str(&format!(" {}", s));
                        }
                    }
                }
                _ => {}
            }
        }
        let (stem, choices) = extract_choices_and_stem(&main_content);

        ParsedQuestion {
            id,
            parent_id,
            title: if title.is_empty() {
                head5.trim().to_string()
            } else {
                title
            },
            stem,
            choices,
            table: table_cells,
            knowledge: knowledge.trim().to_string(),
            answer: answer.trim().to_string(),
            analysis: analysis.trim().to_string(),
            detail: detail.trim().to_string(),
            raw_markdown: markdown.trim().to_string(),
        }
    }

    #[test]
    fn read() {
        let content = r#"##### 母题 1

当 $x = 2$ 时，代数式 $2x + 1$ 的值是（   ）

A．3　　B．5　　C．4　　D．6

| 难度  | 适用学期 | 题目类型   |
| :---- | :------- | :--------- |
| 【1】 | 【71】   | 【选择题】 |

**涉及知识点：** 【求代数式的值】

**参考答案：** 【B】

**【分析】** 本题考查了代数式求值。将 $x = 2$ 代入代数式 $2x + 1$ 直接计算即可。

**【详解】** 解：∵ $x = 2$，
∴ $2x + 1 = 2 \times 2 + 1 = 4 + 1 = 5$。
故选：B。

---

##### 变式 1

当 $x = -1$ 时，代数式 $2x - 2$ 的值为（   ）

A．2　　B．$-2$　　C．4　　D．$-4$

| 难度  | 适用学期 | 题目类型   |
| :---- | :------- | :--------- |
| 【1】 | 【71】   | 【选择题】 |

**涉及知识点：** 【求代数式的值】

**参考答案：** 【D】

**【分析】** 本题考查了求代数式的值，将 $x = -1$ 代入所求代数式计算即可得解。

**【详解】** 解：当 $x = -1$ 时，代数式 $2x - 2 = 2 \times (-1) - 2 = -2 - 2 = -4$。
故选：D。

---

##### 变式 2

当 $a = 3$，$b = -2$ 时，代数式 $a^{2} - b^{2}$ 的值是（   ）

A．4　　B．13　　C．2　　D．5

| 难度  | 适用学期 | 题目类型   |
| :---- | :------- | :--------- |
| 【2】 | 【71】   | 【选择题】 |

**涉及知识点：** 【求代数式的值】

**参考答案：** 【D】

**【分析】** 本题考查了代数式求值。直接代入给定数值计算代数式的值。

**【详解】** 解：当 $a = 3$，$b = -2$ 时，$a^{2} - b^{2} = 3^{2} - (-2)^{2} = 9 - 4 = 5$。
故选：D。

---

##### 变式 3

已知 $a$ 是最大的负整数，$b$ 是最小的正整数，则 $a^{2026} - b^{2027}$ 的结果是（   ）

A．$-1$　　B．0　　C．1　　D．2

| 难度  | 适用学期 | 题目类型   |
| :---- | :------- | :--------- |
| 【2】 | 【71】   | 【选择题】 |

**涉及知识点：** 【求代数式的值】

**参考答案：** 【B】

**【分析】** 此题考查了正整数、负整数，有理数的混合运算。根据题意，$a = -1$；$b = 1$，然后计算 $a$ 的 2026 次方和 $b$ 的 2027 次方，最后求差即可。

**【详解】** 解：∵ $a$ 是最大的负整数，
∴ $a = -1$；
∵ $b$ 是最小的正整数，
∴ $b = 1$；
∴ $a^{2026} - b^{2027} = (-1)^{2026} - 1^{2027} = 1 - 1 = 0$。
故选：B。

---

##### 变式 4

若 $|a| = 5$，$b = -2$，且 $ab > 0$，则 $a + b$ 的值是（    ）

A．3　　B．$-3$　　C．7　　D．$-7$

| 难度  | 适用学期 | 题目类型   |
| :---- | :------- | :--------- |
| 【3】 | 【71】   | 【选择题】 |

**涉及知识点：** 【求代数式的值】

**参考答案：** 【D】

**【分析】** 本题考查了有理数的乘法运算，有理数的加法运算，求一个数的绝对值。由 $|a| = 5$ 知 $a = \pm 5$，结合 $b = -2$，$ab > 0$，可知 $a = -5$，进而求 $a + b$ 的值。

**【详解】** 解：∵ $|a| = 5$
∴ $a = \pm 5$，
∵ $b = -2$，$ab > 0$，
∴ $a$ 与 $b$ 同号，即 $a = -5$
∴ $a + b = -5 + (-2) = -7$。
故选：D。

---

##### 变式 5

若当 $x = -1$，$y = 1$ 时，代数式 $4x^{2} - y^{2}$ 的值是 \_\_\_\_\_\_ 。

| 难度  | 适用学期 | 题目类型   |
| :---- | :------- | :--------- |
| 【2】 | 【71】   | 【填空题】 |

**涉及知识点：** 【求代数式的值】

**参考答案：** 【3】

**【分析】** 本题考查代数式的求值运算，掌握有理数的混合运算规则是解题关键。

**【详解】** 解：当 $x = -1$，$y = 1$ 时，
原式 $= 4 \times (-1)^{2} - 1^{2} = 4 \times 1 - 1 = 4 - 1 = 3$。
故答案为：3。

---

##### 变式 6

当 $m = 9$，$n = -3$ 时，代数式 $m^{2} - mn$ 的值是 \_\_\_\_\_\_ 。

| 难度  | 适用学期 | 题目类型   |
| :---- | :------- | :--------- |
| 【2】 | 【71】   | 【填空题】 |

**涉及知识点：** 【求代数式的值】

**参考答案：** 【108】

**【分析】** 本题主要考查了代数式求值，直接将 $m = 9$ 和 $n = -3$ 代入代数式 $m^{2} - mn$ 进行计算即可。

**【详解】** 解：当 $m = 9$，$n = -3$ 时，
$m^{2} - mn = 9^{2} - 9 \times (-3) = 81 - (-27) = 81 + 27 = 108$。
故答案为：108。

---

##### 变式 7

当 $x = 2$、$y = -3$ 时，求代数式 $2x^{2} - \frac{1}{2}xy - \frac{1}{3}y^{2}$ 的值。

| 难度  | 适用学期 | 题目类型   |
| :---- | :------- | :--------- |
| 【2】 | 【71】   | 【解答题】 |

**涉及知识点：** 【求代数式的值】

**参考答案：** 【$8$】

**【分析】** 本题考查了代数式求值。把 $x = 2$，$y = -3$ 直接代入代数式计算求值即可。

**【详解】** 解：当 $x = 2$，$y = -3$ 时，

$$
\begin{aligned}
& 2x^{2} - \frac{1}{2}xy - \frac{1}{3}y^{2} \\
&= 2 \times 2^{2} - \frac{1}{2} \times 2 \times (-3) - \frac{1}{3} \times (-3)^{2} \\
&= 8 + 3 - 3 \\
&= 8.
\end{aligned}
$$

---

##### 变式 8

摄氏度 $(℃)$ 与华氏度 $(^{\circ}\text{F})$ 是两种常用的温度计量单位，摄氏度 $C(℃)$ 和华氏度 $F(^{\circ}\text{F})$ 之间的关系为 $F = \frac{9}{5}C + 32$，那么将 $35℃$ 转换为华氏度为（　　）

A．95　　B．85　　C．90　　D．105

| 难度  | 适用学期 | 题目类型   |
| :---- | :------- | :--------- |
| 【2】 | 【71】   | 【选择题】 |

**涉及知识点：** 【代数式的书写规范等知识】

**参考答案：** 【A】

**【分析】** 本题考查代数式求值，直接将 $C = 35$ 代入 $F = \frac{9}{5}C + 32$ 进行计算即可。

**【详解】** ∵ $F = \frac{9}{5}C + 32$，$C = 35$，
∴ $F = \frac{9}{5} \times 35 + 32 = 9 \times 7 + 32 = 63 + 32 = 95$，
∴ $35℃$ 转换为华氏度为 $95^{\circ}F$。
故选 A。

---

##### 变式 9

已知梯形的面积 $S = \frac{1}{2}(a + b)h$，其中 $a$ 为上底长，$b$ 为下底长，$h$ 为梯形的高。当 $a = 7\text{cm}$，$b = 10\text{cm}$，$h = 4\text{cm}$ 时，梯形的面积为（　　）

A．$68\text{cm}^{2}$　　B．$51\text{cm}^{2}$　　C．$34\text{cm}^{2}$　　D．$17\text{cm}^{2}$

| 难度  | 适用学期 | 题目类型   |
| :---- | :------- | :--------- |
| 【2】 | 【71】   | 【选择题】 |

**涉及知识点：** 【求代数式的值】

**参考答案：** 【C】

**【分析】** 本题考查代数式的求值，将对应的数代入梯形面积公式计算即可。

**【详解】** 解：∵ $a = 7\text{cm}$，$b = 10\text{cm}$，$h = 4\text{cm}$，
∴ $a + b = 7\text{cm} + 10\text{cm} = 17\text{cm}$，
∴ $(a + b) \cdot h = 17\text{cm} \times 4\text{cm} = 68\text{cm}^{2}$，
∴ $S = \frac{1}{2} \times 68\text{cm}^{2} = 34\text{cm}^{2}$。
故选 C。

---

##### 变式 10

若储蓄的本息和为 $W$。本金为 $a$，利率为 $b$，存款期数为 $T$（按单利计算）。

（1）写出 $W$ 与 $a$，$b$，$T$ 之间的关系式为 \_\_\_\_\_\_ ；
（2）若一年定期存款年利率为 $4.14\%$，现存入银行 $1000$ 元，则明年的今日可得本息和为 \_\_\_\_\_\_ 元。

| 难度  | 适用学期 | 题目类型   |
| :---- | :------- | :--------- |
| 【3】 | 【71】   | 【选择题】 |

**涉及知识点：** 【求代数式的值】

**参考答案：** 【$W = a + abT$，$1041.4$】

**【分析】** 本题考查了列代数式，代数式求值。

（1）根据本息和＝本金+利息＝本金+本金×利率×存期列式即可；
（2）将 $a = 1000$，$b = 4.14\%$，$T = 1$ 代入（1）中所求式子，计算即可。

**【详解】** 解：（1）根据题意可得：$W = a + abT$；
（2）∵ $W = a + abT$，
∴ 当 $a = 1000$，$b = 4.14\%$，$T = 1$ 时，
$W = 1000 + 1000 \times 4.14\% \times 1 = 1041.4$（元），
则明年的今日可得本息和为 $1041.4$ 元。
故答案为：$W = a + abT$；$1041.4$。

---

##### 母题 2

已知 $|m - 3| + |n + 2| = 0$，则 $n - m =$（    ）

A．1　　B．$-1$　　C．$-5$　　D．5

| 难度  | 适用学期 | 题目类型   |
| :---- | :------- | :--------- |
| 【3】 | 【71】   | 【选择题】 |

**涉及知识点：** 【求代数式的值】

**参考答案：** 【C】

**【分析】** 本题考查了绝对值的非负性，代数式求值。利用绝对值的非负性，两个非负数的和为零，则每个数都为零，得出 $m,n$ 的值，代入代数式即可求解。

**【详解】** 解：∵ $|m - 3| + |n + 2| = 0$，$|m - 3| \geq 0$ 且 $|n + 2| \geq 0$，
∴ $|m - 3| = 0$ 且 $|n + 2| = 0$，
∴ $m - 3 = 0$，$n + 2 = 0$，
解得：$m = 3$；$n = -2$；
∴ $n - m = -2 - 3 = -5$。
故选：C。

---

##### 变式 1

若 $|2a + 1| + {(b - 3)}^{2} = 0$，则 $a^{b} =$ \_\_\_\_\_\_ 。

| 难度  | 适用学期 | 题目类型   |
| :---- | :------- | :--------- |
| 【3】 | 【71】   | 【填空题】 |

**涉及知识点：** 【求代数式的值】

**参考答案：** 【$-\frac{1}{8}$】

**【分析】** 本题考查了绝对值与平方数的非负性。根据非负数的性质，绝对值和平方项的和为零，则每个部分都为零，从而求出 $a$ 和 $b$ 的值。

**【详解】** ∵ $|2a + 1| + {(b - 3)}^{2} = 0$，
∴ $2a + 1 = 0$，$b - 3 = 0$，
解得 $a = -\frac{1}{2}$，$b = 3$

$$
a^{b} = \left( -\frac{1}{2} \right)^{3} = -\frac{1}{8}
$$

故答案为：$-\frac{1}{8}$。

---

##### 母题 3

已知 $|a| = 5$，$|b| = 2$，且 $ab < 0$，则 $a + b$ 的值是（　　）

A．$\pm 7$　　B．$-7$　　C．3　　D．$\pm 3$

| 难度  | 适用学期 | 题目类型   |
| :---- | :------- | :--------- |
| 【4】 | 【71】   | 【选择题】 |

**涉及知识点：** 【求代数式的值】

**参考答案：** 【D】

**【分析】** 此题主要考查了代数式求值，有理数的乘法，以及绝对值的含义和求法。根据 $|a| = 5$，$|b| = 2$ 可得：$a = \pm 5$，$b = \pm 2$，再根据 $ab < 0$，可得：$a = -5$，$b = 2$ 或 $a = 5$，$b = -2$，据此求出 $a + b$ 的值即可。

**【详解】** 解：$\because |a| = 5$，$|b| = 2$
$\therefore a = \pm 5$，$b = \pm 2$，
$\because ab < 0$，
$\therefore a = -5$，$b = 2$ 或 $a = 5$，$b = -2$
$\therefore a + b = -5 + 2 = -3$ 或 $a + b = 5 + (-2) = 3$。
故选：D。

---

##### 变式 1

若 $|a| = 3$，$|b| = 1$，且 $a + b > 0$，那么 $a - b$ 的值是（   ）

A．4或2　　B．$-4$或$-2$　　C．4或$-2$　　D．$-4$或2

| 难度  | 适用学期 | 题目类型   |
| :---- | :------- | :--------- |
| 【4】 | 【71】   | 【选择题】 |

**涉及知识点：** 【求代数式的值】

**参考答案：** 【A】

**【分析】** 本题考查绝对值的定义、代数式求值。根据绝对值的定义，结合条件 $a + b > 0$，排除不满足条件的组合，仅保留 $a = 3$，$b = 1$ 和 $a = 3$，$b = -1$ 两种情况，分别计算 $a - b$ 的值即可。

**【详解】** 解：∵ $|a| = 3$，
∴ $a = 3$ 或 $a = -3$；
∵ $|b| = 1$，∴ $b = 1$ 或 $b = -1$。
又∵ $a + b > 0$，
∴ 当 $a = -3$ 时，$a + b$ 最大值为 $-3 + 1 = -2 < 0$，不符合题意，舍去；
∴ $a = 3$，$b = 1$ 或 $a = 3$，$b = -1$，
当 $a = 3$，$b = 1$ 时，$a - b = 3 - 1 = 2$；
当 $a = 3$，$b = -1$ 时，$a - b = 4$；
∴ $a - b$ 的值为4或2。
故选：A。

---

##### 变式 2

若 $|a| = 2$，$b^{2} = 9$，且 $ab > 0$，则 $a + b$ 等于（　　）

A．$\pm 1$　　B．$\pm 5$　　C．1　　D．$-1$

| 难度  | 适用学期 | 题目类型   |
| :---- | :------- | :--------- |
| 【5】 | 【71】   | 【选择题】 |

**涉及知识点：** 【求代数式的值】

**参考答案：** 【B】

**【分析】** 本题考查了绝对值、有理数的乘方、求代数式的值。先根据绝对值和有理数乘方的逆运算求出 $a$ 和 $b$ 的可能值，再分4种情况讨论，结合 $ab > 0$ 找出符合题意的情况，从而计算 $a + b$ 的值即可。

**【详解】** 解：∵ $|a| = 2$，$b^{2} = 9$，
∴ $a = \pm 2$，$b = \pm 3$，
①当 $a = 2$，$b = 3$ 时，$ab = 2 \times 3 = 6 > 0$，符合题意，此时 $a + b = 2 + 3 = 5$；
②当 $a = 2$，$b = -3$ 时，$ab = 2 \times (-3) = -6 < 0$，不符合题意，舍去；
③当 $a = -2$，$b = 3$ 时，$ab = (-2) \times 3 = -6 < 0$，不符合题意，舍去；
④当 $a = -2$，$b = -3$ 时，$ab = (-2) \times (-3) = 6 > 0$，符合题意，此时 $a + b = (-2) + (-3) = -5$；
∴ 综上所述，$a + b = \pm 5$。
故选：B。

---

##### 变式 3

若 $|m - n| = n - m$，且 $|m| = 4, n^{2} = 9$，则 $m + n =$ \_\_\_\_\_\_ 。

| 难度  | 适用学期 | 题目类型   |
| :---- | :------- | :--------- |
| 【4】 | 【71】   | 【填空题】 |

**涉及知识点：** 【求代数式的值】

**参考答案：** 【$-1$ 或 $-7$】

**【分析】** 本题主要考查绝对值，乘方，代数式求值。先根据绝对值和乘方的性质得出 $m$、$n$ 的值，再代入计算即可。

**【详解】** 解：$\because |m - n| = n - m$，
∴ $n - m \geq 0$，即 $m \leq n$，
$\because |m| = 4, n^{2} = 9$，
当 $m = 4$ 时，$n$ 无取值，故舍去，
∴ $m = -4$，$n = 3$ 或 $n = -3$，
当 $m = -4$，$n = 3$ 时，$m + n = -4 + 3 = -1$；
当 $m = -4$，$n = -3$ 时，$m + n = -4 + (-3) = -7$；
故答案为：$-1$ 或 $-7$。

---

##### 变式 4

已知 $|a| = 6$，$|b| = 4$。

(1) 若 $a > 0$，且 $ab > 0$，求 $a - b$ 的值；
(2) 若 $a > b$，求 $ab$ 的值。

| 难度  | 适用学期 | 题目类型   |
| :---- | :------- | :--------- |
| 【4】 | 【71】   | 【解答题】 |

**涉及知识点：** 【求代数式的值】

**参考答案：** 【(1) $a - b = 2$，(2) $ab = 24$ 或 $ab = -24$。】

**【分析】** 本题考查绝对值，代入求值。

（1）由绝对值求出 $a,b$ 的值，取符合 $a > 0$ 且 $ab > 0$ 的要求的解代入计算即可；
（2）由绝对值求出 $a,b$ 的值，取符合 $a > b$ 的要求的解代入计算即可。

**【详解】** （1）解：∵ $|a| = 6$，$|b| = 4$，
∴ $a = 6$ 或 $-6$，$b = 4$ 或 $-4$。
∵ $a > 0$，且 $ab > 0$，
∴ $a = 6$，$b = 4$，
∴ $a - b = 6 - 4 = 2$；
（2）解：由（1）知 $a = 6$ 或 $-6$，$b = 4$ 或 $-4$。
∵ $a > b$，
∴ $a = 6$，$b = 4$ 或 $a = 6$，$b = -4$，
∴ $ab = 6 \times 4 = 24$ 或 $ab = 6 \times (-4) = -24$。

---"#;
        // 母题分组
        let mothers_blocks = split_mothers(content);
        let mut all_questions = Vec::new();

        let mut global_id = 1;
        for mother_block in mothers_blocks {
            let subs = split_main_and_variations(&mother_block);
            if subs.is_empty() {
                continue;
            }
            // 第一项为母题
            let (mother_title, mother_md) = &subs[0];
            let mother_id = global_id;
            let mother_struct = parse_question(mother_id, None, mother_title.clone(), mother_md);
            global_id += 1;

            // 变式
            let mut var_vec = Vec::new();
            for (title, md) in subs.iter().skip(1) {
                let var = parse_question(global_id, Some(mother_id), title.clone(), md);
                var_vec.push(var);
                global_id += 1;
            }

            all_questions.push(Mother {
                question: mother_struct,
                variations: var_vec,
            });
        }

        // 输出结构
        for mother in all_questions {
            println!(
                "\n=== 母题(id:{}) 标题：{} ===",
                mother.question.id, mother.question.title
            );
            println!("题干: {}", mother.question.stem);
            println!("选项: {:?}", mother.question.choices);
            println!("参考答案: {}", mother.question.answer);
            println!("知识点: {}", mother.question.knowledge);
            println!("分析: {}", mother.question.analysis);
            println!("详解: {}", mother.question.detail);
            println!("表格: {:?}", mother.question.table);
            for v in &mother.variations {
                println!(
                    "  -- 变式(id:{}, parent_id:{}) 标题：{}",
                    v.id,
                    v.parent_id.unwrap(),
                    v.title
                );
                println!("     题干: {}", v.stem);
                println!("     选项: {:?}", v.choices);
                println!("     参考答案: {}", v.answer);
                println!("     知识点: {}", v.knowledge);
                println!("     分析: {}", v.analysis);
                println!("     详解: {}", v.detail);
                println!("     表格: {:?}", v.table);
            }
        }
    }
}
