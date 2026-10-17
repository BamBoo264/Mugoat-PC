const cp = require('node:child_process');
const fs = require('node:fs');
const path = require('node:path');

const runId = process.argv[2] || '';
const tag = 'v2.0';
const title = 'Mugoat iOS v2.0 — Voice Chat 3D Proximity, Mặc Định Loa Thường, Sửa Freeze Eclipse Blvd & Sandbox Online';
const rootDir = path.resolve(__dirname, '..');
const keyFile = path.join(rootDir, 'build', 'release-8.10', 'artifact.key');
const artifactScript = path.join(rootDir, 'native-port', 'app', 'Scripts', 'ipa-artifact.cjs');
const outputDir = path.join(rootDir, 'build', `artifacts-${runId || 'latest'}`);

function exec(cmd, args, options = {}) {
  return cp.execFileSync(cmd, args, { encoding: 'utf8', stdio: ['ignore', 'inherit', 'inherit'], ...options });
}

async function main() {
  if (!runId) {
    console.error('Usage: node tools/publish-v20.cjs <runId>');
    process.exit(1);
  }

  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  console.log(`[Tải về] Đang tải artifact từ workflow run ${runId}...`);
  try {
    exec('gh', ['run', 'download', runId, '-n', 'Mugoat-Native-IPAs-encrypted', '-D', outputDir]);
  } catch (e) {
    console.error('Lỗi tải artifact:', e.message);
    process.exit(1);
  }

  const encFree = path.join(outputDir, 'Mugoat-AppleID-Free.enc');
  const ipaFree = path.join(outputDir, 'Mugoat.ipa');

  if (!fs.existsSync(encFree)) {
    console.error('Không tìm thấy tệp mã hóa trong:', outputDir);
    process.exit(1);
  }

  if (fs.existsSync(ipaFree)) fs.unlinkSync(ipaFree);

  console.log(`[Giải mã] Đang giải mã Mugoat.ipa...`);
  cp.execFileSync('node', [artifactScript, 'decrypt', encFree, ipaFree, keyFile], { stdio: 'inherit' });

  const releaseNotes = path.join(rootDir, 'release_v20_notes.md');
  const notesContent = `# Mugoat iOS v2.0 — Voice Chat 3D Proximity, Mặc Định Loa Thường, Sửa Freeze Eclipse Blvd & Sandbox Online

Bản phát hành chính thức **v2.0** mang đến nâng cấp toàn diện và ổn định tối đa cho trải nghiệm Multiplayer và Sandbox:

---

### 🌟 Các cải tiến và sửa lỗi chính trong bản v2.0:

1. **🔊 Âm Thanh Chuẩn Loa Ngoài (Không Còn Bị Ép Vào Loa Thoại):**
   - Game khởi động ở chế độ phát nhạc chuẩn (\`playback\`), âm thanh luôn phát qua loa ngoài với chất lượng stereo cao nhất và âm lượng đầy đủ.
   - Chế độ loa thoại (\`voiceChat\`) chỉ kích hoạt khi người chơi chủ động bấm BẬT mic.
   - Khi tắt mic, loa lập tức trở lại loa ngoài thường.

2. **🎙️ Voice Chat 3D Giới Hạn Khoảng Cách (Proximity Voice):**
   - Âm thanh đàm thoại chỉ nghe được trong phạm vi tối đa 35 mét.
   - Âm lượng to rõ từ 0m - 6m và giảm dần khi ra xa, không còn bị nghe thấy toàn server.

3. **🚦 Sửa Triệt Để Freeze Treo Game Tại Eclipse Blvd & Khởi Động An Toàn:**
   - Xóa bỏ việc cưỡng bức nạp kịch bản nhiệm vụ không tương thích, dọn sạch lỗi tràn stack script tại ngã tư Eclipse Blvd.
   - Điểm spawn trung tâm Los Santos (Legion Square) thông thoáng, không còn bị nhốt sau cửa sắt biệt thự.

4. **📱 Điện Thoại & Súng Khởi Đầu Trong Sandbox Online:**
   - Mở khóa bàn phím và điện thoại trong Sandbox (không còn bị chặn mở điện thoại).
   - Cấp sẵn súng lục tiêu chuẩn và dao găm để đổi súng tức thì, chặn menu cheat trainer khi online.

5. **🚀 Màn Hình Chờ Vào Game:**
   - Mờ dần và ẩn dứt điểm giao diện khi vào thế giới, bổ sung thông báo lỗi rõ ràng nếu máy thiếu RAM thay vì đứng yên ở "Đang tải...".
`;

  fs.writeFileSync(releaseNotes, notesContent, 'utf8');

  console.log(`[Phát hành] Đang tạo/cập nhật Release ${tag} trên GitHub...`);
  try {
    exec('gh', ['release', 'view', tag]);
    exec('gh', ['release', 'edit', tag, '--title', title, '--notes-file', releaseNotes]);
  } catch {
    exec('gh', ['release', 'create', tag, '--title', title, '--notes-file', releaseNotes]);
  }

  console.log(`[Tải lên] Đang tải Mugoat.ipa lên Release ${tag}...`);
  exec('gh', ['release', 'upload', tag, ipaFree, '--clobber']);

  console.log(`[Thành công] Đã xuất bản hoàn tất Mugoat iOS ${tag}!`);
}

main().catch(err => {
  console.error(err);
  process.exit(1);
});
