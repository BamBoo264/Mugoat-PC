const cp = require('node:child_process');
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');

const rootDir = path.resolve(__dirname, '..');

function exec(cmd, args, options = {}) {
  return cp.execFileSync(cmd, args, { cwd: rootDir, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'], ...options });
}

function getDiffHash() {
  try {
    const diff = exec('git', ['diff']);
    const untracked = exec('git', ['status', '--porcelain']);
    return crypto.createHash('sha256').update(diff + untracked).digest('hex');
  } catch {
    return '';
  }
}

function getHeadCommit() {
  try {
    return exec('git', ['rev-parse', 'HEAD']).trim();
  } catch {
    return '';
  }
}

async function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function main() {
  console.log('[AutoWatcher] Khởi động trình giám sát thời gian thực...');
  console.log('[AutoWatcher] Theo dõi các thay đổi từ Claude...');

  let lastHash = getDiffHash();
  let lastHead = getHeadCommit();
  let stableCount = 0;
  const REQUIRED_STABLE_SECONDS = 180; // 3 phút yên lặng không sửa file mới kích hoạt update
  const CHECK_INTERVAL_MS = 5000;
  const REQUIRED_STABLE_CHECKS = Math.ceil((REQUIRED_STABLE_SECONDS * 1000) / CHECK_INTERVAL_MS);

  while (true) {
    await sleep(CHECK_INTERVAL_MS);

    const currentHash = getDiffHash();
    const currentHead = getHeadCommit();
    const status = exec('git', ['status', '--porcelain']).trim();

    if (currentHead !== lastHead) {
      console.log(`[AutoWatcher] Phát hiện commit mới: ${currentHead}`);
      break;
    }

    if (!status) {
      console.log('[AutoWatcher] Working tree sạch, kiểm tra commit từ xa...');
      // Check if local is ahead of remote
      try {
        const ahead = exec('git', ['log', '@{u}..HEAD', '--oneline']).trim();
        if (ahead) {
          console.log(`[AutoWatcher] Có commit cục bộ chưa push: ${ahead}`);
          break;
        }
      } catch {}
      continue;
    }

    if (currentHash === lastHash) {
      stableCount++;
      const stableSecs = stableCount * (CHECK_INTERVAL_MS / 1000);
      console.log(`[AutoWatcher] Mã nguồn ổn định không thay đổi: ${stableSecs}s / ${REQUIRED_STABLE_SECONDS}s...`);

      if (stableCount >= REQUIRED_STABLE_CHECKS) {
        console.log('[AutoWatcher] Claude đã hoàn tất chỉnh sửa! Bắt đầu kiểm tra và đóng gói...');
        break;
      }
    } else {
      console.log('[AutoWatcher] Phát hiện Claude đang cập nhật file... reset bộ đếm thời gian.');
      lastHash = currentHash;
      stableCount = 0;
    }
  }

  // 1. Commit any remaining changes
  const finalStatus = exec('git', ['status', '--porcelain']).trim();
  if (finalStatus) {
    console.log('[AutoWatcher] Đang lưu và commit các thay đổi của Claude...');
    exec('git', ['add', '-A']);
    exec('git', ['commit', '-m', 'feat(v2.0): shopping waypoint, 911 dispatch, robbery & taxi mechanics, audio routing fixes']);
  }

  // 2. Push to origin main
  console.log('[AutoWatcher] Đang đẩy toàn bộ mã nguồn lên GitHub (origin/main)...');
  try {
    cp.execFileSync('git', ['push', 'origin', 'main'], { cwd: rootDir, stdio: 'inherit' });
  } catch (err) {
    console.error('[AutoWatcher] Lỗi push:', err.message);
  }

  // Record current run ID before triggering
  let prevRunId = 0;
  try {
    const listBefore = JSON.parse(exec('gh', ['run', 'list', '--workflow=build-mugoat-native.yml', '--limit', '1', '--json', 'databaseId']));
    if (listBefore.length > 0) prevRunId = listBefore[0].databaseId;
  } catch {}

  // 3. Trigger build workflow
  console.log('[AutoWatcher] Đang kích hoạt workflow build Mugoat Native IPA v2.0...');
  cp.execFileSync('gh', ['workflow', 'run', 'build-mugoat-native.yml', '--ref', 'main'], { cwd: rootDir, stdio: 'inherit' });

  // 4. Wait for new run to register
  let runId = null;
  for (let attempt = 0; attempt < 25; attempt++) {
    await sleep(3000);
    try {
      const runList = exec('gh', ['run', 'list', '--workflow=build-mugoat-native.yml', '--limit', '1', '--json', 'databaseId,status']);
      const runs = JSON.parse(runList);
      if (runs.length && runs[0].databaseId !== prevRunId) {
        runId = runs[0].databaseId;
        break;
      }
    } catch {}
  }

  if (!runId) {
    console.error('[AutoWatcher] Không thể xác định run ID mới của workflow!');
    return;
  }

  console.log(`[AutoWatcher] Đang theo dõi realtime tiến trình build: ${runId}...`);
  cp.execFileSync('gh', ['run', 'watch', String(runId)], { cwd: rootDir, stdio: 'inherit' });

  console.log(`[AutoWatcher] Hoàn tất build! Tệp IPA v2.0 đã được xuất bản lên Release v2.0.`);
}

main().catch(err => {
  console.error('[AutoWatcher Lỗi]', err);
  process.exit(1);
});
