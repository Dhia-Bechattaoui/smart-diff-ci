const os = require('os');
const path = require('path');
const fs = require('fs');
const { execSync } = require('child_process');

const VERSION = 'v0.2.0';
const REPO = 'dhia-bechattaoui/smart-diff-ci';

function getTarget() {
  const type = os.type();
  const arch = os.arch();

  if (type === 'Linux' && arch === 'x64') return 'x86_64-unknown-linux-gnu';
  if (type === 'Darwin' && arch === 'x64') return 'x86_64-apple-darwin';
  if (type === 'Darwin' && arch === 'arm64') return 'aarch64-apple-darwin';
  if (type === 'Windows_NT' && arch === 'x64') return 'x86_64-pc-windows-msvc';

  throw new Error(`Unsupported platform: ${type} ${arch}`);
}

const target = getTarget();
const ext = target.includes('windows') ? 'zip' : 'tar.gz';
const url = `https://github.com/${REPO}/releases/download/${VERSION}/smart-diff-ci-${target}.${ext}`;

console.log(`Downloading smart-diff-ci ${VERSION} from ${url}...`);

try {
  if (ext === 'tar.gz') {
    execSync(`curl -sL "${url}" | tar xz`, { stdio: 'inherit' });
  } else {
    execSync(`curl -sL "${url}" -o smart-diff-ci.zip && tar -xf smart-diff-ci.zip && del smart-diff-ci.zip`, { stdio: 'inherit' });
  }

  const binPath = target.includes('windows') ? 'smart-diff-ci.exe' : 'smart-diff-ci';
  
  if (!target.includes('windows')) {
    fs.chmodSync(binPath, 0o755);
  }

  if (!fs.existsSync('bin')) {
    fs.mkdirSync('bin');
  }
  const targetPath = path.join('bin', binPath);
  
  // Clean up if it already exists from a previous install
  if (fs.existsSync(targetPath)) {
      fs.unlinkSync(targetPath);
  }
  
  fs.renameSync(binPath, targetPath);

  console.log('✅ smart-diff-ci successfully installed via NPM!');
} catch (error) {
  console.error('❌ Failed to download smart-diff-ci binary:', error.message);
  process.exit(1);
}
