# Mugoat PC 
![License](https://img.shields.io/badge/License-GPLv3-blue.svg)
![Platform](https://img.shields.io/badge/Platform-Windows%2010%2F11-0078D6.svg)
![Graphics](https://img.shields.io/badge/Graphics-Vulkan%20%7C%20DirectX%2012-green.svg)
![Renderer](https://img.shields.io/badge/Renderer-WebGPU%20(wgpu)-orange.svg)

Mugoat PC là bản port của dự án Mugoat, chạy Grand Theft Auto V lên hệ điều hành Windows thông qua WebAssembly (WASM) và WebGPU (wgpu). Game chạy native, hỗ trợ hai backend đồ họa **Vulkan** và **DirectX 12**.


## Cài đặt nhanh

1. Tải bản build mới nhất từ trang [Releases](../../releases).
2. Giải nén vào một thư mục (ví dụ `C:\MugoatPC`).
3. Chuẩn bị dữ liệu game GTA V (đã giải nén đầy đủ).
4. Tạo một thư mục tên là `game` bên trong thư mục vừa giải nén.
5. Copy toàn bộ dữ liệu game vào thư mục `game` này.
6. Chạy `Launcher.exe`, chọn backend đồ họa và vào game.

## Build từ mã nguồn
### Yêu cầu công cụ
- Visual Studio 2022 (workload Desktop development with C++)
- Rust toolchain: rustup target add x86_64-pc-windows-msvc
- CMake 3.20+
- Vulkan SDK (nếu build backend Vulkan)

## Tham số dòng lệnh

| Tham số | Mô tả |
|---------|-------|
| `-wgpu` | Sử dụng renderer WebGPU |
| `-width=1280 -height=720` | Độ phân giải cửa sổ |
| `-frameLimit=0` | Không giới hạn FPS |
| `-frameLimit=60` | Giới hạn 60 FPS |
| `-rootdir=/game/` | Đường dẫn tới thư mục dữ liệu game |
| `-noSocialClub` | Tắt kết nối Social Club |

## Backend đồ họa

| Backend | Ưu điểm | Nhược điểm |
|---------|---------|------------|
| **Vulkan** ✅ | Ổn định, ít giật lag, phù hợp GPU NVIDIA | Cần cài Vulkan SDK |
| **DirectX 12** | Tương thích tốt với GPU AMD | Có thể khựng khi compile shader lần đầu |

> **Khuyến nghị:** Dùng **Vulkan** để có trải nghiệm mượt nhất.


Chơi Online / Multiplayer
1. Build Server
Server của Mugoat được viết bằng Rust, sử dụng giao thức UDP, có thể chạy trên Windows / Linux / macOS.

git clone https://github.com/iOSVNNews/mugoat.git
cd mugoat
cargo build --release --bin muguet-server

### 2. Khởi chạy Server
# Cú pháp: muguet-server [CỔNG_UDP] [SỐ_NGƯỜI_TỐI_ĐA]
./target/release/muguet-server 7777 32

- Cổng mặc định: 7777 (UDP)
- Tối đa: 32 người chơi mỗi phòng

Lưu ý: Đảm bảo cổng UDP 7777 đã được mở trong Firewall / Router (Port Forwarding nếu chơi qua mạng Internet).

### 3. Kết nối từ Client PC
1. Mở Launcher, chọn chế độ Online / Multiplayer
2. Nhập địa chỉ máy chủ:
   - Mạng LAN / Nội bộ: 192.168.x.x:7777
   - VPS / Internet: dia-chi-ip-vps:7777 hoặc tenmien.com:7777
3. Chọn Tên nhân vật, Tên phòng (Room), Vai trò (Role) và bật/tắt PvP
4. Bấm Vào game

### 4. Tính năng Multiplayer
| Tính năng | Mô tả |
|-----------|-------|
| Voice Chat 3D | Đàm thoại theo khoảng cách (bán kính 35m), tự chuyển Loa ngoài / Loa thoại |
| Đồng bộ mượt | Đồng bộ nhân vật, lái xe, bắn súng, va chạm, sát thương PvP/PvE |
| Hồi sinh | Ngã xuống sẽ được cứu và hồi sinh tại bệnh viện gần nhất |
| Radar & Locator | Hiển thị vị trí người chơi trong phòng + cự ly (mét) |
| Hệ thống Role | Chọn vai trò tự do: Cảnh sát, Bác sĩ, Taxi, Thợ sửa xe, Dân thường... |
| Trang bị khởi đầu | Súng lục + dao găm; tự động ẩn menu cheat khi vào server |

## Khắc phục sự cố

### Game khựng, tụt FPS xuống 1–2
- Đảm bảo game đặt trên SSD
- Bật Shader Cache trong NVIDIA Control Panel → Manage 3D Settings → Shader Cache Size = 10 GB hoặc Unlimited
- Treo game ở màn hình chính 10–15 phút để game cache xong asset trước khi chơi

### Âm thanh rè hoặc mất tiếng
- Tắt User Music Scan trong cài đặt game
- Tăng Audio Buffer Size lên 2048 hoặc 4096 samples

### Game không khởi động
- Cài đặt Visual C++ Redistributable 2015–2022
- Cập nhật driver GPU lên phiên bản mới nhất

### Không kết nối được Server Online
- Kiểm tra Firewall có chặn cổng UDP 7777 không
- Nếu chơi qua Internet, đảm bảo đã Port Forward trên Router
- Thử đổi cổng khác (ví dụ 7778) nếu 7777 bị xung đột


## Khắc phục sự cố

### Game khựng, tụt FPS xuống 1–2
- Đảm bảo game đặt trên **SSD**.
- Bật **Shader Cache** trong NVIDIA Control Panel → Manage 3D Settings → *Shader Cache Size* = 10 GB hoặc Unlimited.

### Game không khởi động
- Cài đặt **Visual C++ Redistributable 2015–2022**.
- Cập nhật driver GPU lên phiên bản mới nhất.

## Credits

### PC Port & Optimization
- **ToanLee (tele: tolie_264)** – Phát triển bản port PC, Launcher, và tối ưu hóa hiệu năng.

### Original Project
- **iOSVNNews** – [Mugoat iOS Port](https://github.com/iOSVNNews/mugoat)
- **Kevin Erik** – [gta5-ios-port](https://github.com/kevinerikjs/gta5-ios-port)
- **Constantin Clerc (c22dev)** – Giao thức multiplayer UDP và cầu nối logic Muguet
- **Cộng đồng wgpu/gfx-rs** – Thư viện WebGPU cross-platform

## Giấy phép

Dự án phát hành theo **GNU General Public License v3.0 (GPL-3.0)**. Xem chi tiết tại [LICENSE](LICENSE).
