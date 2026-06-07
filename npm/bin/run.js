#!/usr/bin/env node

const { spawnSync } = require('child_process');
const path = require('path');
const os = require('os');
const fs = require('fs');

const binName = os.type() === 'Windows_NT' ? 'smart-diff-ci.exe' : 'smart-diff-ci';
const binPath = path.join(__dirname, binName);

if (!fs.existsSync(binPath)) {
    console.error(`❌ smart-diff-ci binary not found at ${binPath}. Please reinstall the package.`);
    process.exit(1);
}

const args = process.argv.slice(2);

const result = spawnSync(binPath, args, {
  stdio: 'inherit'
});

if (result.error) {
  console.error(result.error);
  process.exit(1);
}

process.exit(result.status ?? 1);
