use chrono::{FixedOffset, LocalResult, TimeZone, Utc};
use futures::future::join_all;

// --- VERİ YAPILARI (MODELS) ---

#[derive(Debug, Clone)]
pub struct Candle {
    pub open_time: u64, // Mumun açılış zamanı (Timestamp)
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Clone)]
pub struct PatternMatch {
    pub symbol: String,
    pub candle_ago: usize,
    pub formatted_time: String,
}

// --- İNDİKATÖR MANTIĞI (HACİM ONAYSIZ) ---

pub fn is_three_white_soldiers(sub_candles: &[Candle]) -> bool {
    let c = |index: usize| -> &Candle { &sub_candles[index] };
    (c(3).close > c(3).open)
        && (c(2).close > c(2).open)
        && (c(1).close > c(1).open)
        && (c(0).low > c(2).high)
        && (c(1).low > c(3).high)
        && (c(2).low > c(4).high)
}

pub fn is_three_black_crows(sub_candles: &[Candle]) -> bool {
    let c = |index: usize| -> &Candle { &sub_candles[index] };
    (c(3).open > c(3).close)
        && (c(2).open > c(2).close)
        && (c(1).open > c(1).close)
        && (c(2).low > c(0).high)
        && (c(3).low > c(1).high)
        && (c(4).low > c(2).high)
}

fn format_timestamp_tsi(ms: u64) -> String {
    match Utc.timestamp_millis_opt(ms as i64) {
        LocalResult::Single(naive) => {
            let tsi_offset = FixedOffset::east_opt(3 * 3600).unwrap();

            let tsi_time = naive.with_timezone(&tsi_offset);
            tsi_time.format("%H:%M TSİ").to_string()
        }
        _ => "Bilinmeyen Zaman".to_string(),
    }
}

// --- BINANCE API SERVİSİ ---

async fn fetch_candles(
    client: &reqwest::Client,
    symbol: &str,
    interval: &str,
) -> Result<Vec<Candle>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.binance.com/api/v3/klines?symbol={}&interval={}&limit=50",
        symbol, interval
    );

    let response = client
        .get(&url)
        .send()
        .await?
        .json::<Vec<Vec<serde_json::Value>>>()
        .await?;
    let mut candles = Vec::new();

    for kline in response {
        if kline.len() >= 6 {
            let open_time: u64 = kline[0].as_i64().unwrap_or(0) as u64;
            let open: f64 = kline[1].as_str().unwrap_or("0").parse()?;
            let high: f64 = kline[2].as_str().unwrap_or("0").parse()?;
            let low: f64 = kline[3].as_str().unwrap_or("0").parse()?;
            let close: f64 = kline[4].as_str().unwrap_or("0").parse()?;
            let volume: f64 = kline[5].as_str().unwrap_or("0").parse()?;

            candles.push(Candle {
                open_time,
                open,
                high,
                low,
                close,
                volume,
            });
        }
    }

    Ok(candles)
}

async fn scan_coin(
    client: &reqwest::Client,
    symbol: &str,
    interval: &str,
) -> (Vec<PatternMatch>, Vec<PatternMatch>) {
    let mut ba_matches = Vec::new();
    let mut sk_matches = Vec::new();

    if let Ok(candles) = fetch_candles(client, symbol, interval).await {
        let mut reversed_candles = candles.clone();
        reversed_candles.reverse();

        for i in 0..=reversed_candles.len() - 6 {
            let window = &reversed_candles[i..i + 6];

            let event_time = format_timestamp_tsi(window[0].open_time);

            if is_three_white_soldiers(window) {
                ba_matches.push(PatternMatch {
                    symbol: symbol.to_string(),
                    candle_ago: i,
                    formatted_time: event_time.clone(),
                });
            }

            if is_three_black_crows(window) {
                sk_matches.push(PatternMatch {
                    symbol: symbol.to_string(),
                    candle_ago: i,
                    formatted_time: event_time,
                });
            }
        }
    } else {
        eprintln!("❌ {} verisi çekilemedi.", symbol);
    }

    (ba_matches, sk_matches)
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let symbols = vec![
        "BTCUSDT",
        "ETHUSDT",
        "SOLUSDT",
        "BNBUSDT",
        "XRPUSDT",
        "ADAUSDT",
        "AVAXUSDT",
        "LINKUSDT",
        "DOTUSDT",
        "DOGEUSDT",
        "MATICUSDT",
        "SHIBUSDT",
    ];

    // Zaman dilimi
    let interval = "1h";

    println!("🚀 Kripto Tarayıcı Analizi Başlatıldı...");
    println!(
        "📊 Taranan Mum Sayısı: Son 50 | Zaman Dilimi: {}\n",
        interval
    );

    let mut tasks = vec![];
    for symbol in &symbols {
        tasks.push(scan_coin(&client, symbol, interval));
    }

    let results = join_all(tasks).await;

    let mut all_white_soldiers = Vec::new();
    let mut all_black_crows = Vec::new();

    for (ba, sk) in results {
        all_white_soldiers.extend(ba);
        all_black_crows.extend(sk);
    }

    println!("============================================================");
    println!("🟢 3 BEYAZ ASKER (3BA) RAPORU");
    println!("============================================================");
    if all_white_soldiers.is_empty() {
        println!("❌ Şartlara uyan herhangi bir coin yoktur.");
    } else {
        for m in all_white_soldiers {
            if m.candle_ago == 0 {
                println!(
                    "🚨 {} -> GÜNCEL canlı mumda tespit edildi! (Açılış Saati: {})",
                    m.symbol, m.formatted_time
                );
            } else {
                println!(
                    "⏳ {} -> {} mum önce gerçekleşmiş. (Açılış Saati: {})",
                    m.symbol, m.candle_ago, m.formatted_time
                );
            }
        }
    }

    println!("\n============================================================");
    println!("🔴 3 SİYAH KARGA (3SK) RAPORU");
    println!("============================================================");
    if all_black_crows.is_empty() {
        println!("❌ Şartlara uyan herhangi bir coin yoktur.");
    } else {
        for m in all_black_crows {
            if m.candle_ago == 0 {
                println!(
                    "🚨 {} -> GÜNCEL canlı mumda tespit edildi! (Açılış Saati: {})",
                    m.symbol, m.formatted_time
                );
            } else {
                println!(
                    "⏳ {} -> {} mum önce gerçekleşmiş. (Açılış Saati: {})",
                    m.symbol, m.candle_ago, m.formatted_time
                );
            }
        }
    }

    println!("\n============================================================");
    println!("✅ Tarama tamamlandı. Program sonlandırıldı.");
}
