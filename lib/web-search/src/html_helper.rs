use scraper::{ElementRef, Html, Selector};

pub(crate) fn clean_text(text: &str) -> String {
    let mut cleaned = text.to_string();

    // 移除各种 Unicode 空格字符
    cleaned = cleaned
        .replace('\u{2002}', "") // EN SPACE
        .replace('\u{2003}', "") // EM SPACE
        .replace('\u{2009}', "") // THIN SPACE
        .replace('\u{00A0}', " ") // NON-BREAKING SPACE
        .replace('\u{200B}', "") // ZERO WIDTH SPACE
        .replace('\u{200C}', "") // ZERO WIDTH NON-JOINER
        .replace('\u{200D}', "") // ZERO WIDTH JOINER
        .replace('\u{2060}', "") // WORD JOINER
        .replace('\u{FEFF}', ""); // ZERO WIDTH NO-BREAK SPACE

    // 移除多余的空白字符（多个空格、换行符等）
    let whitespace_regex = regex::Regex::new(r"\s+").unwrap();
    cleaned = whitespace_regex.replace_all(&cleaned, " ").to_string();

    // 移除 HTML 注释
    let comment_regex = regex::Regex::new(r"<!--.*?-->").unwrap();
    cleaned = comment_regex.replace_all(&cleaned, "").to_string();

    // 移除内联样式和脚本
    let style_regex = regex::Regex::new(r"<style[^>]*>.*?</style>").unwrap();
    cleaned = style_regex.replace_all(&cleaned, "").to_string();

    let script_regex = regex::Regex::new(r"<script[^>]*>.*?</script>").unwrap();
    cleaned = script_regex.replace_all(&cleaned, "").to_string();

    // 移除 SVG 内容
    let svg_regex = regex::Regex::new(r"<svg[^>]*>.*?</svg>").unwrap();
    cleaned = svg_regex.replace_all(&cleaned, "").to_string();

    // 移除 base64 数据 URI
    let base64_regex = regex::Regex::new(r#"data:image/[^;]*;base64,[^'\"\s]*"#).unwrap();
    cleaned = base64_regex.replace_all(&cleaned, "").to_string();

    // 移除 class, id, style 等属性
    let class_regex = regex::Regex::new(r#"\s+class\s*=\s*['\"][^'\"]*['\"]"#).unwrap();
    cleaned = class_regex.replace_all(&cleaned, "").to_string();

    let id_regex = regex::Regex::new(r#"\s+id\s*=\s*['\"][^'\"]*['\"]"#).unwrap();
    cleaned = id_regex.replace_all(&cleaned, "").to_string();

    let style_attr_regex = regex::Regex::new(r#"\s+style\s*=\s*['\"][^'\"]*['\"]"#).unwrap();
    cleaned = style_attr_regex.replace_all(&cleaned, "").to_string();

    // 移除 target="_blank" 属性
    let target_blank_regex = regex::Regex::new(r#"\s+target\s*=\s*['\"]_blank['\"]"#).unwrap();
    cleaned = target_blank_regex.replace_all(&cleaned, "").to_string();

    // 移除所有 data-* 属性
    let data_attr_regex =
        regex::Regex::new(r#"\s+data-[a-zA-Z0-9_-]+\s*=\s*['\"][^'\"]*['\"]"#).unwrap();
    cleaned = data_attr_regex.replace_all(&cleaned, "").to_string();

    // 移除 alt 属性
    let alt_regex = regex::Regex::new(r#"\s+alt\s*=\s*['\"][^'\"]*['\"]"#).unwrap();
    cleaned = alt_regex.replace_all(&cleaned, "").to_string();

    // 移除 href 属性中的 javascript 代码
    let js_href_regex = regex::Regex::new(r#"\s+href\s*=\s*['\"]javascript:[^'\"]*['\"]"#).unwrap();
    cleaned = js_href_regex.replace_all(&cleaned, "").to_string();

    // 移除 onclick 等事件属性
    let event_attr_regex = regex::Regex::new(r#"\s+on[a-zA-Z]+\s*=\s*['\"][^'\"]*['\"]"#).unwrap();
    cleaned = event_attr_regex.replace_all(&cleaned, "").to_string();

    // 移除 srcset 属性
    let srcset_regex = regex::Regex::new(r#"\s+srcset\s*=\s*['\"][^'\"]*['\"]"#).unwrap();
    cleaned = srcset_regex.replace_all(&cleaned, "").to_string();

    // 移除 sizes 属性
    let sizes_regex = regex::Regex::new(r#"\s+sizes\s*=\s*['\"][^'\"]*['\"]"#).unwrap();
    cleaned = sizes_regex.replace_all(&cleaned, "").to_string();

    // 移除 loading 属性
    let loading_regex = regex::Regex::new(r#"\s+loading\s*=\s*['\"][^'\"]*['\"]"#).unwrap();
    cleaned = loading_regex.replace_all(&cleaned, "").to_string();

    // 移除常见的空标签
    let empty_div_regex = regex::Regex::new(r#"<div[^>]*>\s*</div>"#).unwrap();
    cleaned = empty_div_regex.replace_all(&cleaned, "").to_string();

    let empty_span_regex = regex::Regex::new(r#"<span[^>]*>\s*</span>"#).unwrap();
    cleaned = empty_span_regex.replace_all(&cleaned, "").to_string();

    let empty_p_regex = regex::Regex::new(r#"<p[^>]*>\s*</p>"#).unwrap();
    cleaned = empty_p_regex.replace_all(&cleaned, "").to_string();

    let empty_li_regex = regex::Regex::new(r#"<li[^>]*>\s*</li>"#).unwrap();
    cleaned = empty_li_regex.replace_all(&cleaned, "").to_string();

    // 移除多余的空白和整理结果
    cleaned = cleaned.trim().to_string();

    // 移除 meta 标签
    let meta_regex = regex::Regex::new(r#"<meta[^>]*/?>"#).unwrap();
    cleaned = meta_regex.replace_all(&cleaned, "").to_string();

    // 移除 link 标签
    let link_regex = regex::Regex::new(r#"<link[^>]*/?>"#).unwrap();
    cleaned = link_regex.replace_all(&cleaned, "").to_string();

    // 移除 noscript 标签及其内容
    let noscript_regex = regex::Regex::new(r#"<noscript[^>]*>.*?</noscript>"#).unwrap();
    cleaned = noscript_regex.replace_all(&cleaned, "").to_string();

    // 移除 iframe 标签及其内容
    let iframe_regex = regex::Regex::new(r#"<iframe[^>]*>.*?</iframe>"#).unwrap();
    cleaned = iframe_regex.replace_all(&cleaned, "").to_string();

    // 移除 object 和 embed 标签及其内容
    let object_regex = regex::Regex::new(r#"<object[^>]*>.*?</object>"#).unwrap();
    cleaned = object_regex.replace_all(&cleaned, "").to_string();

    let embed_regex = regex::Regex::new(r#"<embed[^>]*/?>"#).unwrap();
    cleaned = embed_regex.replace_all(&cleaned, "").to_string();

    // 移除 track 标签
    let track_regex = regex::Regex::new(r#"<track[^>]*/?>"#).unwrap();
    cleaned = track_regex.replace_all(&cleaned, "").to_string();

    // 移除 source 标签
    let source_regex = regex::Regex::new(r#"<source[^>]*/?>"#).unwrap();
    cleaned = source_regex.replace_all(&cleaned, "").to_string();

    // 移除 canvas 标签及其内容
    let canvas_regex = regex::Regex::new(r#"<canvas[^>]*>.*?</canvas>"#).unwrap();
    cleaned = canvas_regex.replace_all(&cleaned, "").to_string();

    // 再次清理多余的空白字符
    cleaned = whitespace_regex.replace_all(&cleaned, " ").to_string();

    cleaned
}

pub(crate) fn compress_html_for_extraction(html: &str) -> String {
    let mut compressed = html.to_string();

    // 1. 移除或简化不必要的属性
    let attrs_to_remove = [
        r#"aria-[a-zA-Z0-9_-]+="[^"]*""#,
        r#"role="[^"]*""#,
        r#"contenteditable="[^"]*""#,
        r#"target="[^"]*""#,
        r#"aria-label="[^"]*""#,
    ];

    for attr_pattern in &attrs_to_remove {
        let regex = regex::Regex::new(attr_pattern).unwrap();
        compressed = regex.replace_all(&compressed, "").to_string();
    }

    // 2. 移除装饰性标签和内容
    let tags_to_remove = [
        r#"<button[^>]*>.*?</button>"#,
        r#"<nav[^>]*>.*?</nav>"#,
        r#"<script[^>]*>.*?</script>"#,
        r#"<style[^>]*>.*?</style>"#,
        r#"<svg[^>]*>.*?</svg>"#,
        r#"<img[^>]*>"#,
        r#"<link[^>]*/?>"#,
        r#"<meta[^>]*/?>"#,
    ];

    for tag_pattern in &tags_to_remove {
        let regex = regex::Regex::new(tag_pattern).unwrap();
        compressed = regex.replace_all(&compressed, "").to_string();
    }

    // 3. 合并嵌套的div标签 - 修复死循环问题
    let nested_div_regex = regex::Regex::new(r#"<div[^>]*>(.*?)</div>"#).unwrap();
    let max_iterations = 10; // 设置最大迭代次数防止死循环
    let mut iterations = 0;

    loop {
        iterations += 1;
        let original_len = compressed.chars().count();
        compressed = nested_div_regex.replace_all(&compressed, "$1").to_string();

        // 如果没有更多变化，或者达到最大迭代次数，则停止
        if compressed.chars().count() == original_len || iterations >= max_iterations {
            break;
        }
    }

    // 4. 移除空标签
    let empty_tags = [r#"<div>\s*</div>"#, r#"<span>\s*</span>"#, r#"<p>\s*</p>"#];

    for tag_pattern in &empty_tags {
        let regex = regex::Regex::new(tag_pattern).unwrap();
        compressed = regex.replace_all(&compressed, "").to_string();
    }

    // 5. 简化表格结构为文本
    // let table_regex = regex::Regex::new(r#"<table[^>]*>(.*?)</table>"#).unwrap();
    // compressed = table_regex.replace_all(&compressed, |caps: &regex::Captures| {
    //     let table_content = &caps[1];
    //     format!("[表格数据: {}]", Self::simplify_table_content(table_content))
    // }).to_string();

    // 6. 清理多余空白字符
    let whitespace_regex = regex::Regex::new(r"\s+").unwrap();
    compressed = whitespace_regex.replace_all(&compressed, " ").to_string();

    // 7. 修复可能损坏的HTML结构
    compressed = fix_html_structure(&compressed);

    compressed.trim().to_string()
}

/// 修复可能损坏的HTML结构
pub(crate) fn fix_html_structure(html: &str) -> String {
    // 确保标题标签正确闭合
    let mut result = html.to_string();

    let headers = ["h1", "h2", "h3", "h4", "h5", "h6"];
    for header in &headers {
        let open_tag = format!("<{}", header);
        let close_tag = format!("</{}>", header);

        // 如果有打开标签但没有闭合标签，则添加闭合标签
        if result.contains(&open_tag) && !result.contains(&close_tag) {
            let header_regex =
                regex::Regex::new(&format!(r"<{}[^>]*>(.*?)(?=<|$)", header)).unwrap();
            result = header_regex
                .replace_all(&result, |caps: &regex::Captures| {
                    format!("{}{}{}", open_tag, &caps[1], close_tag)
                })
                .to_string();
        }
    }

    result
}

/// 从HTML中提取页面标题
pub fn extract_title_from_html(html: &str) -> String {
    let document = Html::parse_document(html);

    // 首先尝试从<title>标签提取
    if let Ok(title_selector) = Selector::parse("title") {
        if let Some(title_element) = document.select(&title_selector).next() {
            let title = title_element.text().collect::<String>().trim().to_string();
            if !title.is_empty() {
                return title;
            }
        }
    }

    // 如果<title>标签不存在或为空，尝试从<h1>标签提取
    if let Ok(h1_selector) = Selector::parse("h1") {
        if let Some(h1_element) = document.select(&h1_selector).next() {
            let h1_text = h1_element.text().collect::<String>().trim().to_string();
            if !h1_text.is_empty() {
                return h1_text;
            }
        }
    }

    // 如果还是没有找到标题，尝试查找最大的标题标签(h1-h6)
    for i in 1..=6 {
        let selector_str = format!("h{}", i);
        if let Ok(selector) = Selector::parse(&selector_str) {
            if let Some(header_element) = document.select(&selector).next() {
                let header_text = header_element.text().collect::<String>().trim().to_string();
                if !header_text.is_empty() {
                    return header_text;
                }
            }
        }
    }

    // 如果所有方法都失败，返回空字符串
    String::new()
}

/// HTML 转 Markdown
pub(crate) struct HtmlToMdConverter;
impl HtmlToMdConverter {
    /// 从HTML中提取正文并转换为Markdown格式，保持原文结构
    pub fn extract_markdown_from_html(html: &str) -> String {
        let document = Html::parse_document(html);
        let mut markdown = String::new();

        // 使用body选择器来获取主要内容区域，如果没有body则使用整个文档
        let content_selectors = [
            "article",
            "main",
            "[role='main']",
            "[id='main']",
            "[id='content']",
            "[class*='content']",
            "[class*='main']",
            "[role='article']",
            "[id='article']",
            "[class*='article']",
            ".post",
            ".entry",
            ".story",
            "body",
        ];
        let root_element = content_selectors
            .iter()
            .find_map(|&selector_str| {
                Selector::parse(selector_str)
                    .ok()
                    .and_then(|selector| document.select(&selector).next())
            })
            .unwrap_or_else(|| {
                // 如果没有找到特定的内容区域，创建一个包含所有子元素的虚拟根元素
                // 这里我们直接使用document作为根
                // 注意：ElementRef没有公开构造函数，所以我们需要另一种方式处理
                // 我们将直接遍历document的子元素
                // 为了简化处理，我们直接处理整个文档
                // 这里我们用一个特殊标记来表示需要处理整个文档
                return document.root_element();
            });

        // 如果root_element是整个文档，则我们需要特殊处理
        // 否则我们可以遍历其子元素

        // 遍历DOM树，按照文档顺序处理元素
        Self::process_element_recursive(root_element, &mut markdown, 0);

        // 清理多余的空行
        let empty_line_regex = regex::Regex::new(r"\n\s*\n\s*\n").unwrap();
        markdown = empty_line_regex.replace_all(&markdown, "\n\n").to_string();

        markdown.trim().to_string()
    }

    /// 递归处理元素以保持文档结构
    fn process_element_recursive(
        element: scraper::ElementRef,
        markdown: &mut String,
        depth: usize,
    ) {
        let tag_name = element.value().name();

        match tag_name {
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                let level = tag_name[1..].parse::<usize>().unwrap_or(1);
                let text = element
                    .text()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim()
                    .to_string();
                if !text.is_empty() {
                    markdown.push_str(&format!("\n\n{} {}\n\n", "#".repeat(level), text));
                }
            }
            "p" => {
                let text = element
                    .text()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim()
                    .to_string();
                if !text.is_empty() {
                    markdown.push_str(&format!("{}\n\n", text));
                }
            }
            "br" => {
                markdown.push_str("  \n");
            }
            "hr" => {
                markdown.push_str("\n---\n\n");
            }
            "ul" | "ol" => {
                if depth > 0 {
                    // 添加一些缩进
                    markdown.push_str("\n");
                }
                for child in element.children() {
                    match child.value() {
                        scraper::Node::Element(_) => {
                            if let Some(child_ref) = scraper::ElementRef::wrap(child) {
                                Self::process_element_recursive(child_ref, markdown, depth);
                            }
                        }
                        scraper::Node::Text(text) => {
                            let text = text.text.trim();
                            if !text.is_empty() {
                                markdown.push_str(text);
                            }
                        }
                        _ => {}
                    }
                }
                markdown.push_str("\n");
            }
            "li" => {
                let text = element
                    .text()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim()
                    .to_string();
                if !text.is_empty() {
                    let indent = "  ".repeat(depth.saturating_sub(1));
                    markdown.push_str(&format!("{}- {}\n", indent, text));
                }
            }
            "table" => {
                let table_markdown = Self::extract_table_as_markdown(element);
                if !table_markdown.is_empty() {
                    markdown.push_str(&format!("\n{}\n\n", table_markdown));
                }
            }
            "pre" => {
                // 处理代码块
                if let Some(child) = element.children().find(|n| {
                    if let scraper::Node::Element(e) = n.value() {
                        e.name() == "code"
                    } else {
                        false
                    }
                }) {
                    if let Some(code_element) = scraper::ElementRef::wrap(child) {
                        let text = code_element.text().collect::<Vec<_>>().join("\n");
                        if !text.is_empty() {
                            markdown.push_str(&format!("\n\n{}\n\n", text));
                        }
                    }
                } else {
                    // 处理没有code标签的pre元素
                    let text = element.text().collect::<Vec<_>>().join("\n");
                    if !text.is_empty() {
                        markdown.push_str(&format!("\n\n{}\n\n", text));
                    }
                }
            }
            "code" => {
                // 只处理不在<pre>标签内的<code>标签
                let in_pre = element.ancestors().any(|ancestor| {
                    if let scraper::Node::Element(element) = ancestor.value() {
                        element.name() == "pre"
                    } else {
                        false
                    }
                });

                if !in_pre {
                    let text = element.text().collect::<Vec<_>>().join("");
                    if !text.is_empty() {
                        markdown.push_str(&format!("`{}`", text));
                    }
                }
            }
            "strong" | "b" => {
                let text = element
                    .text()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim()
                    .to_string();
                if !text.is_empty() {
                    markdown.push_str(&format!("**{}**", text));
                }
            }
            "em" | "i" => {
                let text = element
                    .text()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim()
                    .to_string();
                if !text.is_empty() {
                    markdown.push_str(&format!("*{}*", text));
                }
            }
            "a" => {
                let text = element
                    .text()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim()
                    .to_string();
                if let Some(href) = element.value().attr("href") {
                    if !text.is_empty() && href.starts_with("http") {
                        markdown.push_str(&format!("[{}]({})", text, href));
                    } else if !text.is_empty() {
                        markdown.push_str(&text); // 如果没有有效的链接，只显示文本
                    }
                } else if !text.is_empty() {
                    markdown.push_str(&text);
                }
            }
            "div" | "span" | "section" | "article" | "main" | "body" | "html" => {
                // 容器元素 - 递归处理子元素
                for child in element.children() {
                    match child.value() {
                        scraper::Node::Element(_) => {
                            if let Some(child_ref) = scraper::ElementRef::wrap(child) {
                                Self::process_element_recursive(child_ref, markdown, depth);
                            }
                        }
                        scraper::Node::Text(text_node) => {
                            let text = text_node.text.trim();
                            if !text.is_empty() {
                                markdown.push_str(text);
                            }
                        }
                        _ => {}
                    }
                }

                // 在块级元素后添加换行（根据需要）
                if matches!(tag_name, "div" | "section" | "article") {
                    if !markdown.ends_with("\n\n") && !markdown.is_empty() {
                        markdown.push_str("\n");
                    }
                }
            }
            _ => {
                // 处理其他元素 - 递归处理子元素
                for child in element.children() {
                    match child.value() {
                        scraper::Node::Element(_) => {
                            if let Some(child_ref) = scraper::ElementRef::wrap(child) {
                                Self::process_element_recursive(child_ref, markdown, depth);
                            }
                        }
                        scraper::Node::Text(text_node) => {
                            let text = text_node.text.trim();
                            if !text.is_empty() {
                                markdown.push_str(text);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// 将HTML表格转换为Markdown表格
    fn extract_table_as_markdown(table_element: scraper::ElementRef) -> String {
        let mut markdown_table = String::new();

        // 提取表头
        if let Ok(thead_selector) = Selector::parse("thead tr") {
            for row in table_element.select(&thead_selector) {
                if let Ok(th_selector) = Selector::parse("th") {
                    let headers: Vec<String> = row
                        .select(&th_selector)
                        .map(|th| th.text().collect::<Vec<_>>().join(" ").trim().to_string())
                        .collect();

                    if !headers.is_empty() {
                        markdown_table.push_str(&format!("| {} |\n", headers.join(" | ")));
                        // 添加分隔行
                        markdown_table.push_str(&format!(
                            "|{}|\n",
                            headers
                                .iter()
                                .map(|_| " --- ")
                                .collect::<Vec<_>>()
                                .join("|")
                        ));
                    }
                }
            }
        }

        // 提取表体行
        let tbody_row_selectors = ["tbody tr", "tr"];
        let mut found_rows = false;

        for selector_str in &tbody_row_selectors {
            if let Ok(row_selector) = Selector::parse(selector_str) {
                for row in table_element.select(&row_selector) {
                    // 跳过表头行（如果已经在thead中处理过）
                    if *selector_str == "tr" && !found_rows {
                        // 检查是否是表头行
                        if let Ok(th_selector) = Selector::parse("th") {
                            if row.select(&th_selector).count() > 0 {
                                continue; // 跳过表头行
                            }
                        }
                    }

                    if let Ok(td_selector) = Selector::parse("td") {
                        let cells: Vec<String> = row
                            .select(&td_selector)
                            .map(|td| td.text().collect::<Vec<_>>().join(" ").trim().to_string())
                            .collect();

                        if !cells.is_empty() {
                            markdown_table.push_str(&format!("| {} |\n", cells.join(" | ")));
                            found_rows = true;
                        }
                    }
                }
            }
        }

        markdown_table
    }
}

/// 基于概率分布的HTML正文提取器
pub struct ImprovedContentExtractor;
impl ImprovedContentExtractor {
    /// 使用概率分布方法提取网页正文内容
    pub fn extract_content_element_with_probability(html: &str) -> String {
        let document = Html::parse_document(html);
        let mut elements_with_scores: Vec<(ElementRef, f64)> = Vec::new();

        // 按文档顺序收集所有可评分的元素并计算得分
        Self::collect_elements_in_order(document.root_element(), &mut elements_with_scores);

        // 过滤出得分高于10的元素
        let high_score_elements: Vec<(ElementRef, f64)> = elements_with_scores
            .into_iter()
            .filter(|(_, score)| *score > 10.)
            .collect();

        // 如果有得分高于5的元素，过滤掉被包含的元素，只保留顶级元素
        if !high_score_elements.is_empty() {
            let top_level_elements = Self::filter_top_level_elements(high_score_elements);

            let mut combined_html = String::new();

            // 按文档顺序添加所有顶级高分元素
            for element in top_level_elements {
                combined_html.push_str(&Self::element_to_html_string(element));
            }

            return combined_html;
        }

        // 备用方案：启发式查找
        Self::extract_element_heuristic(&document).unwrap_or(html.to_string())
    }

    /// 过滤出顶级元素，移除被其他元素包含的元素
    /// 过滤出顶级元素，移除被其他元素包含的元素
    fn filter_top_level_elements(elements_with_scores: Vec<(ElementRef, f64)>) -> Vec<ElementRef> {
        let mut top_level = Vec::new();

        // 按得分排序，得分高的优先考虑
        let mut sorted_elements = elements_with_scores;
        sorted_elements.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        for (current_element, _) in sorted_elements {
            let mut is_contained = false;

            // 检查当前元素是否被已选中的元素包含
            for &selected_element in &top_level {
                if Self::is_element_contained_by(current_element, selected_element) {
                    is_contained = true;
                    break;
                }
            }

            // 如果未被包含，则添加到结果中
            if !is_contained {
                top_level.push(current_element);
            }
        }

        // 按文档顺序重新排序
        top_level.sort_by(|&a, &b| {
            // 通过比较元素在原始文档中的位置来排序
            Self::compare_element_position(a, b)
        });

        top_level
    }

    /// 检查element是否被container元素包含
    fn is_element_contained_by(element: ElementRef, container: ElementRef) -> bool {
        // 检查container是否是element的祖先
        let mut current = element;
        while let Some(parent) = current.parent() {
            if let Some(parent_element) = ElementRef::wrap(parent) {
                if std::ptr::eq(
                    parent_element.value() as *const _,
                    container.value() as *const _,
                ) {
                    return true;
                }
                current = parent_element;
            } else {
                break;
            }
        }
        false
    }

    /// 比较两个元素在文档中的位置
    fn compare_element_position(a: ElementRef, b: ElementRef) -> std::cmp::Ordering {
        // 使用元素的唯一标识来比较文档顺序
        // 由于scraper库没有直接提供比较文档顺序的方法，
        // 我们可以通过比较元素在DOM树中的路径来确定顺序

        // 获取元素a的路径
        let a_path = Self::get_element_path(a);
        let b_path = Self::get_element_path(b);

        // 比较路径来确定文档顺序
        a_path.cmp(&b_path)
    }

    /// 获取元素在DOM树中的路径（用于比较文档顺序）
    fn get_element_path(element: ElementRef) -> Vec<usize> {
        let mut path = Vec::new();
        let mut current = element;

        // 向上遍历到根节点，记录路径
        loop {
            // 获取当前元素在其父元素中的索引位置
            if let Some(parent) = current.parent() {
                if let Some(parent_element) = ElementRef::wrap(parent) {
                    let mut index = 0;
                    let mut found = false;

                    // 查找当前元素在父元素子元素中的位置
                    for (i, child) in parent_element.children().enumerate() {
                        if let Some(child_element) = ElementRef::wrap(child) {
                            if std::ptr::eq(
                                child_element.value() as *const _,
                                current.value() as *const _,
                            ) {
                                index = i;
                                found = true;
                                break;
                            }
                        }
                    }

                    if found {
                        path.push(index);
                    }

                    current = parent_element;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        // 反转路径，使其从根节点开始
        path.reverse();
        path
    }

    /// 按文档顺序收集元素并计算得分
    fn collect_elements_in_order<'a>(
        element: ElementRef<'a>,
        elements_with_scores: &mut Vec<(ElementRef<'a>, f64)>,
    ) {
        // 只收集有ID或可能包含内容的元素
        if element.value().id().is_some() || Self::is_content_container(element.value().name()) {
            let score = Self::calculate_element_score(&element);
            elements_with_scores.push((element, score));
        }

        // 按文档顺序递归处理子元素
        for child in element.children() {
            if let Some(child_element) = ElementRef::wrap(child) {
                Self::collect_elements_in_order(child_element, elements_with_scores);
            }
        }
    }

    /// 将ElementRef转换为HTML字符串，保持原始结构
    fn element_to_html_string(element: ElementRef) -> String {
        let mut html = String::new();

        // 构建开始标签
        html.push('<');
        html.push_str(element.value().name());

        // 添加所有属性
        for (name, value) in element.value().attrs() {
            html.push(' ');
            html.push_str(name);
            html.push('=');
            html.push('"');
            html.push_str(value);
            html.push('"');
        }

        html.push('>');

        // 递归添加子元素，保持文档顺序
        for child in element.children() {
            match child.value() {
                scraper::Node::Text(text) => {
                    html.push_str(&text.text);
                }
                scraper::Node::Element(_) => {
                    if let Some(child_element) = ElementRef::wrap(child) {
                        html.push_str(&Self::element_to_html_string(child_element));
                    }
                }
                _ => {}
            }
        }

        // 添加结束标签
        html.push_str("</");
        html.push_str(element.value().name());
        html.push('>');

        html
    }

    /// 判断元素是否可能是内容容器
    fn is_content_container(tag_name: &str) -> bool {
        matches!(
            tag_name,
            "div" | "article" | "section" | "main" | "p" | "td" | "li"
        )
    }

    /// 计算单个元素的内容得分
    fn calculate_element_score(element: &ElementRef) -> f64 {
        let mut score = 0.0;

        // 基于文本长度的得分
        let text_content = Self::extract_text_without_links(element);
        let text_length = text_content.chars().filter(|c| !c.is_whitespace()).count();
        score += text_length as f64 * 0.5;

        // 基于标签类型的权重
        let tag_weight = match element.value().name() {
            "article" => 3.0,
            "main" => 2.5,
            "div" => 1.0,
            "section" => 1.5,
            "p" => 2.,
            "td" => 0.8,
            "li" => 0.7,
            "nav" | "header" | "footer" | "aside" | "script" | "style" => 0.2,
            _ => 1.0,
        };
        score *= tag_weight;

        // 基于CSS类和ID的得分调整
        let class_attr = element.value().attr("class").unwrap_or("");
        let id_attr = element.value().attr("id").unwrap_or("");

        let positive_indicators = [
            "content", "main", "article", "post", "entry", "text", "story",
        ];
        let negative_indicators = [
            "nav",
            "menu",
            "sidebar",
            "ad",
            "ads",
            "advertisement",
            "header",
            "footer",
            "comment",
            "meta",
            "social",
        ];

        for indicator in &positive_indicators {
            if class_attr.contains(indicator) || id_attr.contains(indicator) {
                score *= 2.0;
            }
        }

        for indicator in &negative_indicators {
            if class_attr.contains(indicator) || id_attr.contains(indicator) {
                score *= 0.3;
            }
        }

        // 链接密度惩罚
        let link_density = Self::calculate_link_density(element);
        if link_density > 0.3 {
            score *= 1.0 - link_density;
        }

        // 段落数量奖励
        let paragraph_count = Self::count_paragraphs(element);
        score += (paragraph_count as f64) * 20.0;

        score
    }

    /// 提取不包含链接的文本内容
    fn extract_text_without_links(element: &ElementRef) -> String {
        let mut text = String::new();

        if let Ok(a_selector) = Selector::parse("a") {
            // 创建一个包含所有链接文本的集合
            let link_texts: Vec<String> = element
                .select(&a_selector)
                .map(|link| link.text().collect::<String>())
                .collect();

            // 获取元素的所有文本
            let all_text = element.text().collect::<String>();

            // 从所有文本中移除链接文本
            text = all_text;
            for link_text in link_texts {
                text = text.replace(&link_text, "");
            }
        } else {
            text = element.text().collect::<String>();
        }

        text
    }

    /// 计算链接密度
    fn calculate_link_density(element: &ElementRef) -> f64 {
        let total_text = element.text().collect::<String>();
        let total_chars = total_text.chars().count();

        if total_chars == 0 {
            return 0.0;
        }

        let mut link_text = String::new();
        if let Ok(a_selector) = Selector::parse("a") {
            for link in element.select(&a_selector) {
                link_text.push_str(&link.text().collect::<String>());
            }
        }

        let link_chars = link_text.chars().count();
        link_chars as f64 / total_chars as f64
    }

    /// 计算段落数量
    fn count_paragraphs(element: &ElementRef) -> usize {
        if let Ok(p_selector) = Selector::parse("p") {
            element.select(&p_selector).count()
        } else {
            0
        }
    }

    /// 启发式元素提取
    fn extract_element_heuristic(document: &Html) -> Option<String> {
        let content_selectors = [
            "article",
            "main",
            "[role='main']",
            "#content",
            "#main",
            ".content",
            ".main-content",
            ".post-content",
            ".entry-content",
            ".article-content",
        ];

        for selector_str in &content_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    return Some(Self::element_to_html_string(element));
                }
            }
        }

        // 如果找不到特定容器，查找文本最密集的区域
        Self::find_text_densest_element(document)
    }

    /// 查找文本最密集的元素
    fn find_text_densest_element(document: &Html) -> Option<String> {
        let mut candidates = Vec::new();

        if let Ok(div_selector) = Selector::parse("div") {
            for element in document.select(&div_selector) {
                let text_content = element.text().collect::<String>();
                let text_length = text_content.chars().filter(|c| !c.is_whitespace()).count();

                if text_length > 100 {
                    candidates.push((element, text_length));
                }
            }
        }

        // 按文本长度排序
        candidates.sort_by(|a, b| b.1.cmp(&a.1));

        // 返回文本最长的元素内容
        if let Some((element, _)) = candidates.first() {
            Some(Self::element_to_html_string(*element))
        } else {
            // 最后手段：返回body元素
            if let Ok(body_selector) = Selector::parse("body") {
                if let Some(body_element) = document.select(&body_selector).next() {
                    return Some(Self::element_to_html_string(body_element));
                }
            }
            None
        }
    }
}

/// 提取网页正文元素
pub(crate) fn extract_content_element_with_probability(html: &str) -> String {
    ImprovedContentExtractor::extract_content_element_with_probability(html)
}
/// 提取网页正文内容
pub fn extract_content_with_probability(html: &str) -> String {
    let html = ImprovedContentExtractor::extract_content_element_with_probability(html);
    HtmlToMdConverter::extract_markdown_from_html(&html)
}
