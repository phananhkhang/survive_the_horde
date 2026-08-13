# 🧟 Survive The Horde

[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Engine](https://img.shields.io/badge/Engine-Bevy%200.19.1-blue.svg?style=flat-square)](https://bevyengine.org/)

**Survive The Horde** là một tựa game 2D Top-down Action Survival bắn súng sinh tồn kịch tính được xây dựng bằng ngôn ngữ **Rust** và **Bevy Game Engine** (v0.19.1). Người chơi sẽ vào vai một chiến binh đơn độc chống lại đợt quái vật hung hãn đang săn đuổi mình.

---

## 🎯 Giới thiệu Game

Trong **Survive The Horde**, bạn sẽ bước vào một trận chiến sinh tử:
- **Người chơi (Player)** xuất phát ở trung tâm bản đồ với **10 HP**.
- **50 quái vật (Enemies)** sẽ xuất hiện ngẫu nhiên và liên tục săn đuổi bạn.
- Bạn cần né tránh, di chuyển khéo léo và xả đạn để tiêu diệt toàn bộ bầy quái vật trước khi lượng HP của bạn cạn kiệt!

---

## 🎮 Cách chơi & Lối chơi (Gameplay)

### 🏆 Điều kiện Thắng / Thua
- **Chiến thắng (Win)**: Tiêu diệt tất cả **50 quái vật** trên bản đồ.
- **Thất bại (Game Over)**: Máu của người chơi giảm xuống **0 HP**.

### 📊 Thông số nhân vật & Cơ chế
- **Người chơi (Player)**:
  - **Máu**: 10 HP
  - **Tốc độ**: 300 px/s
  - Giới hạn di chuyển trong phạm vi màn hình.
- **Quái vật (Enemy)**:
  - **Số lượng**: 50 quái vật.
  - **Máu**: 2 HP (Cần 2 phát bắn để tiêu diệt).
  - **Tốc độ**: 50 px/s (Luôn hướng về phía người chơi).
  - **Sát thương**: Va chạm với người chơi sẽ gây **1 sát thương** và quái vật sẽ tự biến mất.
- **Cơ chế Bắn (Shooting)**:
  - Khi nhấn **Space** hoặc **Chuột trái**, nhân vật sẽ tự động nhằm và bắn vào một quái vật ngẫu nhiên trên màn hình.
  - Mỗi viên đạn trúng đích làm quái vật mất **1 HP** và cộng **1 điểm (Score)** khi quái vật bị hạ gục.

---

## ⌨️ Phím điều khiển (Controls)

| Thao tác | Phím / Thao tác |
| :--- | :--- |
| **Di chuyển Up / Down / Left / Right** | `W`, `A`, `S`, `D` hoặc Phím mũi tên (`↑`, `↓`, `←`, `→`) |
| **Bắn đạn (Auto Target)** | Phím `Space` hoặc **Chuột trái (Left Click)** |
| **Tương tác UI (Menu / Replay)** | Click **Chuột trái** vào các nút trên màn hình |

---

## 🛠️ Công nghệ sử dụng (Tech Stack)

- **Ngôn ngữ**: [Rust](https://www.rust-lang.org/) (Edition 2024)
- **Game Engine**: [Bevy Engine 0.19.1](https://bevyengine.org/) (Kiến trúc ECS - Entity Component System)
- **Thư viện phụ trợ**: `rand 0.10.2` (Xử lý ngẫu nhiên vị trí & mục tiêu bắn)

---

## 📁 Cấu trúc thư mục dự án (Project Structure)

```text
survive_the_horde/
├── assets/                  # Tài nguyên đồ họa & hình ảnh
│   ├── enemy/               # Sprite quái vật
│   ├── player/              # Sprite người chơi
│   └── ui/                  # Backgrounds & giao diện UI
├── src/
│   ├── main.rs              # Khởi tạo game, quản lý GameState & Systems
│   ├── player.rs            # Logic người chơi (xử lý di chuyển & HP)
│   ├── enemy.rs             # AI quái vật (đuổi theo người chơi & va chạm)
│   ├── bullet.rs            # Logic viên đạn (bắn tự động, va chạm, gây sát thương)
│   └── ui/                  # Các màn hình UI
│       ├── mod.rs
│       ├── menu.rs          # Màn hình Menu chính (Start Game)
│       ├── hud.rs           # Hiển thị HP & Điểm số trực tiếp
│       ├── win.rs           # Màn hình chiến thắng
│       └── game_over.rs     # Màn hình thất bại (Retry / Main Menu)
├── Cargo.toml               # Khai báo dependency & thông tin package
└── Cargo.lock
```

---

## 🚀 Hướng dẫn cài đặt & Chạy game (How to Run)

### Yêu cầu hệ thống:
- Đã cài đặt **Rust** và **Cargo** (Khuyên dùng bản ổn định mới nhất).
  *(Tải tại [rustup.rs](https://rustup.rs/))*

### Các bước thực hiện:

1. **Clone dự án (hoặc mở thư mục dự án):**
   ```bash
   git clone <repository-url>
   cd survive_the_horde
   ```

2. **Chạy game (Development Mode):**
   ```bash
   cargo run
   ```

3. **Chạy game tối ưu hiệu năng (Release Mode):**
   ```bash
   cargo run --release
   ```

---
*Chúc bạn có những phút giây giải trí tuyệt vời và sinh tồn thành công trước bầy quái vật! 🧟‍♂️💥*
