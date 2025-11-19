// Quick build script to avoid Vite hanging
const { exec } = require('child_process');
const path = require('path');

console.log('🔨 Starting quick production build...');

const viteCmd = 'npx vite build --mode production --minify terser 2>&1';
console.log(`Running: ${viteCmd}`);

const proc = exec(viteCmd, { 
  cwd: '/Volumes/tempext/Projects/disk-bloat-scanner',
  maxBuffer: 10 * 1024 * 1024,
  timeout: 120000 
});

let output = '';
proc.stdout.on('data', (data) => {
  output += data;
  console.log(data.toString());
});

proc.stderr.on('data', (data) => {
  output += data;
  console.log(data.toString());
});

proc.on('close', (code) => {
  if (code === 0) {
    console.log('\n✅ Build completed successfully!');
  } else {
    console.log(`\n❌ Build failed with code ${code}`);
  }
});

// Kill after 90 seconds
setTimeout(() => {
  console.log('\n⏱️ Build timeout (90s) - terminating');
  proc.kill('SIGKILL');
}, 90000);
