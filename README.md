# Mugoat iOS v2.0 — Native Port for GTA V

[![License: GPL-3.0](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-iOS%2016.0%2B-black.svg)]()
[![Architecture](https://img.shields.io/badge/Architecture-ARM64%20Native-orange.svg)]()
[![Graphics](https://img.shields.io/badge/Graphics-Apple%20Metal-red.svg)]()
[![Release](https://img.shields.io/badge/Release-v2.0-success.svg)](https://github.com/iOSVNNews/mugoat/releases/tag/v2.0)

**Mugoat** là dự án mã nguồn mở đưa tựa game Grand Theft Auto V lên nền tảng **iOS / iPadOS (iOS 16.0+)**, được biên dịch mã nguồn gốc ARM64 chạy thuần native trực tiếp cùng giao diện đồ họa **Apple Metal**. Ứng dụng hoạt động hoàn toàn không cần JIT, không dùng WebView hay trình giả lập máy ảo, mang lại hiệu năng cao và độ trễ đầu vào thấp nhất trên thiết bị iPhone/iPad.

Dự án được cấp phép đầy đủ theo chuẩn **GNU General Public License v3.0 (GPL-3.0)** nhằm phục vụ cộng đồng nghiên cứu, trải nghiệm và tự do phát triển tiếp.

---

## 🌟 Lời Cảm Ơn & Người Đóng Góp (Credits & Acknowledgments)

Dự án Mugoat kế thừa và phát triển dựa trên nền tảng quý báu từ cộng đồng nguồn mở:

- **Kevin Erik ([kevinerikjs/gta5-ios-port](https://github.com/kevinerikjs/gta5-ios-port))**: Dự án port gốc đột phá đã khai phá khả năng chạy engine trên hệ điều hành iOS qua Metal và kiến trúc UIKit. Chúng tôi chân thành ghi nhận và cảm ơn đóng góp nền tảng to lớn này.
- **Constantin Clerc (c22dev)**: Tác giả của giao thức mạng multiplayer UDP gốc và cầu nối logic Muguet.
- **iOSVN News Team**: Đội ngũ phát triển hệ thống Multiplayer thời gian thực, Voice Chat 3D Proximity theo khoảng cách, hệ thống điều khiển cảm ứng đa điểm thế hệ mới, tối ưu hóa bộ nhớ Jetsam và duy trì các bản cập nhật.

---

## 🚀 Các Tính Năng Chính Trong Phiên Bản v2.0

### 1. 🎙️ Voice Chat 3D Proximity (Đàm Thoại Khoảng Cách Không Gian)
- **Âm thanh theo khoảng cách thực**: Giới hạn trong bán kính 35 mét. Âm lượng rõ nét từ 0m – 6m và suy giảm mượt mà khi đi xa; người chơi ở xa ngoài 35m sẽ không bị nghe thấy tạp âm.
- **Thông minh chuyển đổi Loa ngoài / Loa thoại**:
  - Mặc định khởi động game ở chế độ loa ngoài (`.playback`), giữ trọn vẹn chất lượng âm thanh stereo nguyên bản, không bị nén âm hoặc ép sang loa thoại nhỏ.
  - Chế độ loa thoại (`.playAndRecord` / `.voiceChat`) **chỉ kích hoạt khi người chơi chủ động bấm nút Mic** trên màn hình.
  - Khi Tắt mic, hệ thống tự động hoàn trả luồng âm thanh về loa ngoài thường lập tức.

### 2. 🌐 Multiplayer & Chế Độ Chơi Toàn Diện
- **Chế độ chơi đa dạng**: Hỗ trợ đầy đủ chế độ **Cốt truyện (Story Mode)** và **Tự do (Sandbox)** cả Offline lẫn Online Multiplayer.
- **Đồng bộ hóa mượt mà**: Đồng bộ chuyển động nhân vật, lái xe, bắn súng, va chạm xe cộ và sát thương PvP/PvE.
- **Hồi sinh bệnh viện & Cứu thương**: Khi ngã xuống, nhân vật được cứu chữa và hồi sinh tại các bệnh viện gần nhất của Los Santos kèm khiên bảo vệ tạm thời.
- **Radar & Định vị toàn cầu**: Radar hiển thị vị trí của tất cả người chơi khác trong cùng phòng; kèm hệ thống HUD Locator thông báo người chơi gần nhất và cự ly (mét).
- **Hệ thống Vai trò (Roles)**: Cho phép chọn Role tự do (Cảnh sát, Bác sĩ, Taxi, Thợ sửa xe, Dân thường...) trực tiếp trong menu máy chủ mà không cần đăng ký tài khoản.

### 3. 📱 Sandbox Online & Khắc Phục Lỗi Nhiệm Vụ
- **Mở khóa điện thoại trong game**: Tự do sử dụng điện thoại trong chế độ Sandbox Online để trải nghiệm.
- **Trang bị vũ khí khởi đầu**: Khởi đầu với súng lục tiêu chuẩn (`weapon_pistol`) kèm dao găm để đổi súng ngay; đồng thời tự động ẩn menu cheat trainer khi tham gia máy chủ online để đảm bảo công bằng.
- **Khởi động an toàn tại Legion Square**: Loại bỏ hoàn toàn lỗi kẹt sau cửa sắt biệt thự và lỗi freeze đứng hình khi lái xe qua giao lộ **Eclipse Blvd**.

### 4. 🎮 Giao Diện & Điều Khiển Tiên Tiến
- **Nút bấm cảm ứng ngữ cảnh (Dynamic Context)**: Tự động đổi bố cục khi đi bộ, lái xe, lái máy bay, ngắm bắn hoặc đạp xe đạp.
- **Cảm biến con quay hồi chuyển (Gyro Aiming)**: Hỗ trợ nghiêng máy để ngắm bắn chuẩn xác.
- **Hỗ trợ phần cứng**: Nhận diện tay cầm chơi game MFi, Xbox, PlayStation DualSense và bàn phím chuột ngoài.

### 5. 💾 Tiện Ích Quản Lý Tệp
- **Tích hợp Add-on Mod DLC**: Cho phép nạp thêm các gói xe mod `.rpf` trực tiếp trên iPhone qua thư viện Files.
- **Sao lưu Save Game**: Xuất và khôi phục tệp save `.mugoatsave` độc lập mà không sợ mất dữ liệu.

---

## 🛠️ Hướng Dẫn Cài Đặt Dữ Liệu Game (Dành Cho Người Chơi)

1. Tải file **`Mugoat.ipa`** phiên bản mới nhất từ trang [Releases v2.0](https://github.com/iOSVNNews/mugoat/releases/tag/v2.0).
2. Cài đặt file IPA vào thiết bị bằng **TrollStore** (khuyên dùng để mở toàn bộ RAM), **Sideloadly**, hoặc **AltStore/Scarlet**.
3. Chuẩn bị thư mục dữ liệu GTA V (đã giải nén tệp game đầy đủ).
4. Mở ứng dụng **Tệp (Files)** trên iOS:
   - Truy cập: `Trên iPhone của tôi (On My iPhone) › Mugoat`
   - Đặt toàn bộ các tệp của game vào thư mục `Mugoat` này.
5. Mở ứng dụng **Mugoat**, chọn chế độ chơi mong muốn và bấm **Vào game**.

---

## 📡 Hướng Dẫn Tạo & Quản Lý Server Online Riêng (Dedicated Server)

Server của Mugoat được xây dựng bằng **Rust** siêu nhẹ, sử dụng giao thức truyền tải UDP tốc độ cao, hỗ trợ chạy trên Linux (Ubuntu/Debian/CentOS), macOS hoặc Windows.

### Cách 1: Chạy Server Trực Tiếp Bằng Rust / Cargo (Khuyên dùng)

1. **Cài đặt Rust** (nếu máy chủ chưa có):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Tải mã nguồn và biên dịch Server:**
   ```bash
   git clone https://github.com/iOSVNNews/mugoat.git
   cd mugoat
   cargo build --release --bin muguet-server
   ```

3. **Khởi chạy Server:**
   ```bash
   # Cú pháp: ./target/release/muguet-server [CỔNG_UDP] [SỐ_NGƯỜI_TỐI_ĐA_MỖI_PHÒNG]
   ./target/release/muguet-server 7777 32
   ```
   - Mặc định: Cổng UDP `7777`, tối đa `32` người chơi cùng một phòng.
   - Để chạy nền liên tục trên VPS/Linux server:
     ```bash
     nohup ./target/release/muguet-server 7777 32 > server.log 2>&1 &
     ```

### Cách 2: Mở Cổng Tường Lửa (Port Forwarding / Firewall)
- Đảm bảo bạn đã mở cổng **UDP** tương ứng (ví dụ: `7777/udp`):
  ```bash
  # Trên Ubuntu / Debian:
  sudo ufw allow 7777/udp
  ```
- Nếu dùng router gia đình, cần cấu hình **NAT / Port Forwarding** cổng `7777 UDP` tới địa chỉ IP nội bộ của máy chủ.

### Cách 3: Kết Nối Từ Thiết Bị iOS Vào Server
1. Trên ứng dụng Mugoat ở iPhone/iPad, bấm vào biểu tượng **Máy chủ online** (hoặc biểu tượng Trái đất trên màn hình chính).
2. Bật công tắc **Chơi online**.
3. Nhập địa chỉ máy chủ:
   - Nếu trong cùng mạng Wi-Fi nội bộ: `192.168.x.x:7777`
   - Nếu server có IP Public / VPS: `dia-chi-ip-vps:7777` hoặc `tenmien.com:7777`
4. Chọn **Tên nhân vật**, **Tên phòng (Room)** (người chơi cùng phòng sẽ nhìn thấy nhau), chọn **Vai trò (Role)** và bật/tắt **PvP**.
5. Bấm đóng cài đặt và nhấn **Vào game** để bắt đầu trải nghiệm Multiplayer cùng bạn bè!

---

## 🏗️ Hướng Dẫn Tự Biên Dịch Ứng Dụng (Build Source Code)

### Yêu cầu môi trường:
- macOS (Apple Silicon M1/M2/M3/M4 khuyên dùng)
- Xcode 15+ hoặc 16+ kèm Command Line Tools
- `xcodegen` (`brew install xcodegen`)
- Rust toolchain với mục tiêu iOS: `rustup target add aarch64-apple-ios`

### Các bước biên dịch IPA:
```bash
git clone https://github.com/iOSVNNews/mugoat.git
cd mugoat/native-port
./build_ipa.sh
```
Sau khi hoàn tất, file `Mugoat.ipa` sẽ được tạo trong thư mục `native-port/`.

---

## 📄 Bản Quyền & Giấy Phép (License)

Dự án này được phát hành theo giấy phép **GNU General Public License v3.0 (GPL-3.0)**.
Xem nội dung chi tiết tại tệp [LICENSE](LICENSE).

Mọi người dùng và lập trình viên đều có quyền tự do xem, sửa đổi, đóng góp và phân phối lại mã nguồn theo các điều khoản của giấy phép GPL-3.0.
