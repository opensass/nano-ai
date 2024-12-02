<div align="center">

# 🤖 Nano AI

[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/nano-ai.svg)](https://crates.io/crates/nano-ai)
[![Crates.io Downloads](https://img.shields.io/crates/d/nano-ai)](https://crates.io/crates/nano-ai)
[![Docs.rs](https://docs.rs/nano-ai/badge.svg)](https://docs.rs/nano-ai/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/wiseaidev)
[![Netlify Status](https://api.netlify.com/api/v1/badges/21f0f284-a363-42a9-b72a-3a9c82f26f00/deploy-status)](https://nano-wasm.netlify.app/)

![banner](https://github.com/user-attachments/assets/87956e6c-9c9f-428a-8bb6-0b6221b8f6a6)

</div>

## 🌌 Overview

<video src="https://github.com/user-attachments/assets/a57552af-5fe2-4cd4-ad3e-40f8ab264711"></video>

`nano-ai` is a lightweight and efficient AI adapter for integrating Gemini Nano AI model into WASM frameworks.

## 🚀 Features

| Method                  | Supported | Tested | Windows Chrome Canary | Linux Chrome Canary |
|-------------------------|-----------|--------|-----------------------|---------------------|
| `get_capabilities`      | ✅        | ✅     | ✅                    | ✅                  |
| `create_session`        | ✅        | ✅     | ✅                    | ✅                  |
| `send_prompt`           | ✅        | ✅     | ✅                    | ❌                  |
| `stream_prompt`         | ✅        | ⬜     | ✅                    | ❌                  |
| `destroy_session`       | ✅        | ✅     | ✅                    | ✅                  |

> [!WARNING]  
> This crate has been fully tested only on [Chrome Canary](https://www.google.com/chrome/canary/) running on a Windows machine. Compatibility on Linux Chrome Canary is limited, as only `get_capabilities`, `create_session`, and `destroy_session` methods are confirmed to work. 

## 🌐 WASM Frameworks

| Framework | Supported   |
|-----------|-------------|
| Yew       | ✅          |
| Dioxus    | ⬜          |
| Leptos    | ⬜          |

## 📖 Examples

| Framework | Example                                                                                   |
|-----------|-------------------------------------------------------------------------------------------|
| Yew       | [![Github](https://img.shields.io/badge/Open-Github-181717.svg?logo=github&logoColor=white)](./examples/yew)       |
| Dioxus    | ⬜          |
| Leptos    | ⬜          |

⬜: TODO

## 👥 Contributing

We welcome contributions to enhance `nano-ai`! Feel free to submit issues, create pull requests, or discuss improvements on our [GitHub repository](https://github.com/opensass/nano-ai).

## 📝 License

This project is licensed under the [MIT License](LICENSE).
