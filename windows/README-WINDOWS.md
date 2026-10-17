# Mugoat PC — bản Windows thử nghiệm v0.1

Đã biên dịch client Windows x64, có launcher chọn thư mục game và nhập server/phòng. Chưa có bộ game đầy đủ để xác nhận vào màn chơi hay chơi chéo với iPhone. Đây là bản port thử nghiệm của host Rust/Wasmtime trong repository, không phải bản GTA V PC chính thức và không phải chuyển nguyên engine native iOS sang Windows.

Nguồn: https://github.com/iOSVNNews/mugoat
Commit gốc: `20df81e6a4cdf45c794959c69aabedba70168379`.
Các chỉnh sửa Windows thực hiện ngày 15/10/2026. Giấy phép dự án: GPL-3.0-or-later; xem LICENSE và THIRD-PARTY-LICENSES.txt. Mã nguồn tương ứng nằm trong gói Mugoat-PC-Windows-Source-v0.1.zip.

## Cách dùng

1. Giải nén toàn bộ ZIP vào một thư mục có quyền ghi. Chạy **Mugoat-Launcher.exe**. Giữ **Mugoat-PC.exe** cạnh launcher.
2. Bấm **Kiểm tra GPU**. Bước này không cần data, không kết nối server.
3. Khi có game, chọn thư mục gốc chứa **data/manifest.json**, engine và shaders; bấm **Kiểm tra data**.
4. Bật **Multiplayer**, điền **cùng địa chỉ server IP:port, tên phòng và mật khẩu phòng** như bản iOS. Điền tên người chơi rồi bấm **Chơi**. Địa chỉ và port lấy từ bản iOS đang dùng; không tự đoán địa chỉ server. Port mặc định trong mã nguồn là UDP 47520.
5. Log mỗi lượt nằm trong `user-data/latest.log`; cấu hình trong `user-data/config.json`; save/cache trong `user-data`. Khi game đang chạy, đóng cửa sổ game trước khi thoát launcher.

Launcher giữ dữ liệu ở thư mục đã chọn, không tự sao chép bộ game nặng vào thư mục client. Lượt đầu có thể mất thời gian biên dịch WASM; chưa đo được thời gian thực tế vì chưa có engine.

## Dữ liệu cần có

Một bố cục được hỗ trợ:

```text
thu-muc-game/
  data/
    manifest.json
    ... toàn bộ file mà manifest yêu cầu ...
  b/
    <build>/
      game.wasm
      shaders/
        index.json
        ... các shader WGSL tương ứng ...
```

Cũng hỗ trợ `game.wasm` và thư mục `shaders` đặt trực tiếp trong thư mục gốc. Engine phải tương thích với ABI mà host trong commit này hỗ trợ; tên file giống nhau chưa bảo đảm tương thích.

Thư mục **gta5PVP/data** đã cung cấp là data mod, không thay thế full game, engine hoặc shader. Không cần chuyển hàng chục GB để thử launcher/GPU. Trước khi chuyển full data từ iPhone, hãy kiểm tra gói gốc có **game.wasm** và shader pack **WGSL** hay không. File IPA, engine ARM64 native, shader `.metallib` hoặc cache biên dịch cho iOS không dùng thay thế được. Nếu gói iOS không có các file này, cần bộ WASM/WGSL tương thích từ nhóm phát triển; riêng repository này chưa cung cấp engine và assets đầy đủ.

## Chơi chung với iOS

Giữ nguyên từng byte của `src/mp.rs`, `src/proto.rs`, `src/server.rs` so với commit gốc. `native-port/online-bridge/src/lib.rs` của iOS cũng đưa trực tiếp ba file này vào thư viện mạng. Windows launcher ánh xạ server, room, room_pass, player_name và pvp vào đúng cấu hình của host.

Đây là cơ sở về mã nguồn cho khả năng cùng giao thức. Kiểm tra relay cục bộ đã qua handshake, ID, tên người chơi, truyền state/appearance, tách phòng/mật khẩu và thông báo rời phòng. Client thứ hai trong bài test chỉ là trình mô phỏng gói tin, **không phải thiết bị iOS**. Không kết nối server công cộng trong quá trình kiểm tra.

Chưa xác nhận phiên bản iOS đang cài, phiên bản server cộng đồng, ABI của engine hoặc đồng bộ thực tế khi vào game. Cần thử hai thiết bị với dữ liệu và phiên bản tương thích để kết luận chơi chéo hoạt động.

## Phần đã làm và giới hạn

- Windows x64: Vulkan/DirectX 12 qua wgpu; âm thanh qua backend Windows của CPAL; đọc/ghi file thay API Unix; thư mục lưu portable.
- Giữ xử lý bàn phím/chuột của desktop host; thêm XInput theo mapping tay cầm của bản Apple. Chưa thử tay cầm vật lý hay điều khiển trong game.
- Launcher chọn game, GPU, độ phân giải, tỷ lệ render, FPS, multiplayer, phòng, mật khẩu và PvP. Preset ban đầu thấp, 1280×720, tỷ lệ 0.5, giới hạn 60 FPS; có thể đổi trong launcher. Chưa có đo FPS gameplay.
- Không có giao diện cảm ứng/menu iOS. Voice chat/microphone của iOS chưa được nối vào âm thanh của desktop host; không coi voice chat đã hỗ trợ. Âm thanh thông báo dùng `afplay` trong upstream cũng chưa được chuyển sang Windows.
- Kiểm tra GPU chỉ xác nhận tạo được thiết bị; chưa xác nhận shader game, render khung hình, map, crash, save, hiệu năng hoặc vào server thật.
- Mã upstream không yêu cầu đăng nhập chính thức/anti-cheat trong relay đã kiểm tra; các server thực tế có thể có yêu cầu khác chưa kiểm chứng.

## Kết quả kiểm tra trên máy hiện tại

- Build release Windows x64 MSVC thành công (Rust 1.99.0), có một số cảnh báo mã chỉ dùng trên iOS.
- 18 test client, 3 test server, 7 test import/manifest đều qua; ba test giao thức xuất hiện ở cả client và server. Harness input iOS có 0 test chạy trên Windows.
- Vulkan và DirectX 12 đều tạo thiết bị trên NVIDIA GeForce RTX 2060.
- Test relay loopback qua, test serialize cấu hình launcher qua; giao diện launcher đã render kiểm tra bố cục.
- Thiếu data được báo rõ và dừng với mã 1, không giả lập kết quả vào game.
- Chưa có bất kỳ bài test gameplay hoặc PC ↔ iPhone thật nào.

Yêu cầu dự kiến: Windows 10/11 x64, .NET Framework 4.8 cho launcher, driver Vulkan/DirectX 12 và Microsoft Visual C++ Runtime x64 nếu máy chưa có. Máy hiện tại chạy được cả hai EXE. Chưa thử trên một bản Windows sạch. Không kèm game data trong ZIP.

## Biên dịch lại

Cài Rust stable `x86_64-pc-windows-msvc`, Visual Studio C++ Build Tools với Windows SDK; .NET Framework C# compiler. Trong thư mục nguồn chạy:

```powershell
.\windows\build.ps1
cargo test --locked --release
python .\windows\test_relay.py
```

File kết quả trong `target/release`. Mã nguồn Windows mới nằm trong `src/platform.rs`, `src/pad_windows.rs`, `src/desktop.rs`, `windows/Launcher.cs`. Các thay đổi còn lại nằm ở cấu hình dependency, khởi động, chọn GPU, file I/O và cấu hình game. Xem WINDOWS-PATCH.diff để so với commit gốc.

Các lệnh hỗ trợ không cần mở launcher:

```powershell
.\Mugoat-PC.exe --init
.\Mugoat-PC.exe --gpu-check
$env:MUGOAT_GAME_DIR = 'D:\MyGame'
.\Mugoat-PC.exe --check-game
# Sau khi kiểm tra dữ liệu thành công:
.\Mugoat-PC.exe
```

`MUGOAT_GPU` nhận `auto`, `vulkan`, `dx12`; `MUGOAT_HOME` đổi nơi lưu config/save/cache. Cấu hình đã lưu ở launcher được áp dụng bằng biến môi trường khi launcher mở engine. Chạy engine trực tiếp cần tự đặt đường dẫn game ngoài thư mục mặc định.
