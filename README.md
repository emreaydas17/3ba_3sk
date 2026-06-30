# 🚀 Crypto Pattern Screener (3BA & 3SK)

![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)
![Binance](https://img.shields.io/badge/API-Binance_Futures-FCD535.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

A high-performance asynchronous crypto screener written in **Rust**. It scans the top cryptocurrencies on Binance using a sliding window algorithm over the last 50 candles to detect **Three White Soldiers (3BA)** and **Three Black Crows (3SK)** candlestick patterns without volume validation.

---

## 🇺🇸 English

### ✨ Features

* **Asynchronous & Parallel:** Scans multiple coins simultaneously using `tokio` and `futures`.
* **Precise Time Tracking:** Displays the exact opening time of the pattern candle converted to **Turkey Time (TRT / UTC+3)** using the `chrono` library.
* **Aggregated Output:** Grouped and clean reporting for both patterns at the end of the execution.
* **One-time execution:** Ideal for running with cron jobs or CI/CD pipelines.

## 🇹🇷 Türkçe

Rust diliyle yazılmış, yüksek performanslı ve asenkron bir kripto para tarayıcı botu. Binance üzerindeki popüler paritelerin son 50 mumunu kayan pencere algoritmasıyla tarayarak hacim onaysız 3 Beyaz Asker (3BA) ve 3 Siyah Karga (3SK) mum formasyonlarını tespit eder.

### ✨ Özellikler

* **Asenkron ve Paralel: tokio ve futures kullanarak birden fazla coini aynı anda (eşzamanlı) tarar.
* **Nokta Atışı Zaman Takibi: Formasyonun gerçekleştiği mumun açılış saatini chrono kütüphanesiyle hatasız bir şekilde Türkiye Saatine (TSİ / UTC+3) çevirir.
* **Toplu Raporlama: Terminal kirliliğini önlemek amacıyla tarama sonunda sonuçları formasyon başlıkları altında toplu gösterir.
* **Tek Seferlik Çalışma: Zamanlanmış görevler veya CI/CD süreçleriyle tetiklenmek için uygundur.