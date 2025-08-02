use crate::{ExtractType, FetchType, extract_text, html_helper};
use anyhow::anyhow;
use headless_chrome::protocol::cdp::Target::CreateTarget;
use headless_chrome::{Browser, LaunchOptions, Tab};
use reqwest::Client;
use reqwest::header::HeaderMap;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use sysinfo::{ProcessesToUpdate, System};

const SEARCH_API: &str = "https://cn.bing.com/search?q=";
const DEFAULT_PROMPT: &str = r#"你是一个网页正文提取器。请从提供的网页内容中提取正文，并返回纯文本的Markdown格式内容：
要求：
1.仅提取网页核心正文，排除广告、导航、评论等无关内容
2.输出纯文本Markdown，严禁包含任何HTML标签
2.保持原文结构和格式（标题、列表、引用、加粗等）
3.不要添加任何解释、总结或额外内容
4.无法提取时返回空字符串
5.只需返回提取到的正文内容，不要包含任何其他文字:"#;

static BROWSER: OnceLock<Browser> = OnceLock::new();
pub struct WebSearch;

#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResult {
    pub title: String,
    pub url: String,
    pub content: Option<String>,
}

impl WebSearch {
    pub async fn search(keyword: &str, count: Option<u32>) -> anyhow::Result<Vec<SearchResult>> {
        let count = count.unwrap_or(10);
        if count > 100 {
            return Err(anyhow::anyhow!("Maximum search count supported is 100"));
        }
        let mut m = count / 10;
        if m % 10 != 0 {
            m += 10;
        }
        let mut result = vec![];
        for i in 0..m {
            let url = &format!(
                "{}{} -site:www.zhihu.com&first={}",
                SEARCH_API,
                keyword,
                i * 10 + 1
            );
            log::debug!("search url: {}", url);
            let client = Self::get_client()?;
            let html = client
                .get(url)
                .headers(Self::get_headers())
                .send()
                .await?
                .text()
                .await?;

            let document = Html::parse_document(&html);

            let h2_selector = Selector::parse("h2").map_err(|e| anyhow!(e.to_string()))?;
            let a_selector = Selector::parse("a").map_err(|e| anyhow!(e.to_string()))?;
            let b_algo_selector = Selector::parse(".b_algo").map_err(|e| anyhow!(e.to_string()))?;
            let b_caption_selector =
                Selector::parse(".b_caption").map_err(|e| anyhow!(e.to_string()))?;
            for element in document.select(&b_algo_selector) {
                let url = element
                    .select(&h2_selector)
                    .next()
                    .and_then(|h2| h2.select(&a_selector).next())
                    .and_then(|a| a.attr("href"));
                if url.is_none() {
                    continue;
                }
                let title = element
                    .select(&h2_selector)
                    .next()
                    .and_then(|h2| h2.select(&a_selector).next())
                    .map(|a| a.text().collect::<String>())
                    .unwrap_or_default();
                if title.is_empty() {
                    continue;
                }
                let summary = element
                    .select(&b_caption_selector)
                    .next()
                    .and_then(|summary| Some(summary.text().collect::<String>()))
                    .unwrap_or_default();

                result.push(SearchResult {
                    title,
                    url: url.unwrap().to_string(),
                    summary: html_helper::clean_text(&summary),
                });
            }
        }

        let result = result
            .iter()
            .take(count as usize)
            .cloned()
            .collect::<Vec<_>>();
        Ok(result)
    }

    fn get_client() -> anyhow::Result<Client> {
        let client = Client::builder().build()?;
        Ok(client)
    }
    fn get_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36".parse().unwrap());
        headers.insert(reqwest::header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8".parse().unwrap());
        headers.insert(
            reqwest::header::CACHE_CONTROL,
            "private, max-age=0".parse().unwrap(),
        );
        headers.insert("Cookie", "MUID=3129A596BF106E7A31F1B3ACBE536FC3; MUIDB=3129A596BF106E7A31F1B3ACBE536FC3; _EDGE_S=F=1&SID=36F263CB7E156FAF39FB75F17F566E1B; _EDGE_V=1; SRCHD=AF=NOFORM; SRCHUID=V=2&GUID=21E0253EA21745D199B2525A3B46BCA6&dmnchg=1; MUIDB=3129A596BF106E7A31F1B3ACBE536FC3; _UR=QS=0&TQS=0&Pn=0; BFBUSR=BFBHP=0; _HPVN=CS=eyJQbiI6eyJDbiI6MSwiU3QiOjAsIlFzIjowLCJQcm9kIjoiUCJ9LCJTYyI6eyJDbiI6MSwiU3QiOjAsIlFzIjowLCJQcm9kIjoiSCJ9LCJReiI6eyJDbiI6MSwiU3QiOjAsIlFzIjowLCJQcm9kIjoiVCJ9LCJBcCI6dHJ1ZSwiTXV0ZSI6dHJ1ZSwiTGFkIjoiMjAyNS0wNy0zMVQwMDowMDowMFoiLCJJb3RkIjowLCJHd2IiOjAsIlRucyI6MCwiRGZ0IjpudWxsLCJNdnMiOjAsIkZsdCI6MCwiSW1wIjoyLCJUb2JuIjowfQ==; _Rwho=u=d&ts=2025-07-31; ipv6=hit=1753941088924&t=4; USRLOC=HS=1&ELOC=LAT=30.42397117614746|LON=103.80491638183594|N=%E6%96%B0%E6%B4%A5%E5%8C%BA%EF%BC%8C%E5%9B%9B%E5%B7%9D%E7%9C%81|ELT=4|; _RwBf=r=0&ilt=2&ihpd=1&ispd=1&rc=3&rb=0&rg=200&pc=0&mtu=0&rbb=0&clo=0&v=2&l=2025-07-30T07:00:00.0000000Z&lft=0001-01-01T00:00:00.0000000&aof=0&ard=0001-01-01T00:00:00.0000000&rwdbt=0&rwflt=0&rwaul2=0&g=&o=2&p=&c=&t=0&s=0001-01-01T00:00:00.0000000+00:00&ts=2025-07-31T04:51:32.9664576+00:00&rwred=0&wls=&wlb=&wle=&ccp=&cpt=&lka=0&lkt=0&aad=0&TH=&cid=0&gb=; _SS=SID=36F263CB7E156FAF39FB75F17F566E1B&R=3&RB=0&GB=0&RG=200&RP=0; dsc=order=BingPages; SRCHHPGUSR=SRCHLANG=zh-Hans&PREFCOL=0&BRW=HTP&BRH=S&CW=902&CH=695&SCW=1164&SCH=4056&DPR=1.3&UTC=480&PV=19.0.0&HV=1753937495&HVE=CfDJ8ONjmqURwN9Bgygetv6pz_npYH7dJLzIBHb9W2tYZeCN59yN2HqJutW8ZuPV4rC3tHcFeCAEPXiSBZrUlCMhCoTF5mqj4BCiC1dta6dnlstY-r4b6Osg6xEeXvUcKETG-6PnNBYlDU5W0BjSeACb5Mx4jES6lX5OlR2dfsVfjoI2Q-xGHXMfZwhQWQQHsxyGtg&BZA=0&PRVCW=1536&PRVCH=695&B=0&EXLTT=1; SRCHUSR=DOB=20250731&DS=1".parse().unwrap());
        headers
    }

    pub async fn get_raw_content_with_chrome(tab: &Arc<Tab>) -> anyhow::Result<(String, String)> {
        log::info!("fetching html");
        // 等待body元素加载完成
        tab.wait_for_element_with_custom_timeout("body", Duration::from_secs(5))?;

        // 给页面更多时间加载内容
        tokio::time::sleep(Duration::from_secs(1)).await;

        // 尝试获取body内容，最多重试10次
        for attempt in 1..=10 {
            match tab.find_element("body") {
                Ok(body_element) => {
                    let html = body_element.get_content()?;
                    // 清理
                    let content = html_helper::clean_text(&html);
                    // 压缩
                    let content = html_helper::compress_html_for_extraction(&content);
                    // 检查内容是否有效
                    if body_element.get_inner_text()?.trim().chars().count() > 20 || attempt == 10 {
                        log::info!("html parsed, {} chars", html.chars().count());
                        return Ok((tab.get_title()?, content));
                    }
                }
                Err(e) if attempt == 5 => {
                    return Err(anyhow::anyhow!("Failed to get body content: {}", e));
                }
                _ => {}
            }

            log::info!("Retrying to get body content...");
            // 等待后重试
            tokio::time::sleep(Duration::from_millis(300 * attempt)).await;
        }

        // 默认返回
        let body = tab.find_element("body")?.get_content()?;
        // 清理
        let content = html_helper::clean_text(&body);
        // 压缩
        let content = html_helper::compress_html_for_extraction(&content);

        Ok((tab.get_title()?, content))
    }
    pub async fn get_raw_content_with_static(url: &str) -> anyhow::Result<(String, String)> {
        let client = Self::get_client()?;
        let response = client.get(url).send().await?;
        let html = response.text().await?;
        log::info!("html parsed, {} chars", html.chars().count());

        //从html中提取标题
        let title = html_helper::extract_title_from_html(&html);

        // 清理
        let content = html_helper::clean_text(&html);
        // 压缩
        let content = html_helper::compress_html_for_extraction(&content);

        Ok((title, content))
    }

    pub async fn extract(
        url: &str,
        fetch_type: FetchType,
        extract_type: ExtractType,
    ) -> anyhow::Result<PageResult> {
        log::info!(
            "extracting url: {}, fetch_type: {:?}, extract_type: {:?}",
            url,
            fetch_type,
            extract_type
        );

        let (title, raw_content) = match fetch_type {
            FetchType::Static => Self::get_raw_content_with_static(url).await?,
            FetchType::Dynamic => {
                let browser = BROWSER.get_or_init(|| {
                    let current_dir = env::current_exe().unwrap();
                    let current_dir = current_dir.parent().unwrap();
                    let path = current_dir.join("resources").join("chrome");
                    let browser = Browser::new(LaunchOptions {
                        headless: env::var("CHROME_HEADLESS")
                            .unwrap_or("true".to_string())
                            .parse::<bool>()
                            .unwrap(),
                        sandbox: false,
                        window_size: Some((1080, 720)),
                        idle_browser_timeout: Duration::from_secs(60 * 60 * 24),
                        #[cfg(windows)]
                        path: path.join("chrome").into(),
                        #[cfg(unix)]
                        path: path.join("chrome.exe").into(),
                        args: vec![
                            "--blink-settings=imagesEnabled=false".as_ref(),
                            "--disable-images".as_ref(),
                        ],
                        ..Default::default()
                    })
                    .unwrap();
                    browser
                });
                browser.get_process_id();
                let tab = browser.new_tab_with_options(CreateTarget {
                    url: url.to_string(),
                    width: None,
                    height: None,
                    browser_context_id: None,
                    enable_begin_frame_control: None,
                    new_window: None,
                    background: None,
                    for_tab: None,
                })?;
                let content = Self::get_raw_content_with_chrome(&tab).await?;
                tab.close_target()?;
                content
            }
        };
        log::info!(
            "raw html cleaned and compressed, {} chars",
            raw_content.chars().count()
        );

        log::debug!("compressed html: {}", raw_content);

        // 提取正文
        let content = match extract_type {
            ExtractType::Algorithm => html_helper::extract_content_with_probability(&raw_content),
            ExtractType::AI => extract_text::extra(DEFAULT_PROMPT.to_string(), raw_content)
                .await
                .map_err(|e| {
                    log::error!("extract failed: {}", e.to_string());
                    anyhow!("extract failed")
                })?
                .unwrap_or_default(),
            ExtractType::Mix => {
                let html = html_helper::extract_content_element_with_probability(&raw_content);
                log::info!("get main content, {} chars", html.chars().count());
                log::debug!("main content: {}", html);
                extract_text::extra(DEFAULT_PROMPT.to_string(), html)
                    .await
                    .map_err(|e| {
                        log::error!("extract failed: {}", e.to_string());
                        anyhow!("extract failed")
                    })?
                    .unwrap_or_default()
            }
        };

        let content = content
            .replace("```markdown", "")
            .replace("```", "")
            .trim_matches('\n')
            .to_string();
        let content = if content.starts_with("markdown") {
            content.replace("markdown", "").to_string()
        } else {
            content
        };

        let result = PageResult {
            title,
            url: url.to_string(),
            content: Some(content),
        };
        Ok(result)
    }

    pub async fn close() {
        if let Some(browser) = BROWSER.get() {
            if let Some(pid) = browser.get_process_id() {
                log::info!("kill browser process: {}", pid);
                let mut system = System::new_all();
                system.refresh_processes(ProcessesToUpdate::All, true);
                if let Some(process) = system.process(sysinfo::Pid::from_u32(pid)) {
                    process.kill();
                }
            }
        }
    }
}

#[tokio::test]
async fn test_search() {
    let content = WebSearch::search("释永信", Some(10)).await.unwrap();
    println!("[web-search]content: {:#?}", content);
}

#[tokio::test]
async fn test_extract() {
    common::init_log();
    let content =
        WebSearch::extract("https://news.sina.com.cn/c/2025-07-27/doc-infhxkpq5795183.shtml")
            .await
            .unwrap();
    println!("[web-search]content: {:#?}", content);
}
