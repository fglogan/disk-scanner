import { execSync } from 'child_process';
import { createServer } from 'vite';

// Find available port (similar to Bash scan)
function findAvailablePort(startPort) {
  const isWindows = process.platform === 'win32';
  for (let port = startPort; port < startPort + 100; port++) {
    try {
      if (isWindows) {
        execSync(`netstat -ano | findstr :${port}`, { stdio: 'ignore' });
      } else {
        execSync(`lsof -i :${port}`, { stdio: 'ignore' });
      }
    } catch (e) {
      return port;  // Error means port free
    }
  }
  throw new Error('No available port found');
}

const port = findAvailablePort(3002);
console.log(`Starting Vite on available port ${port}`);

// Start Vite with dynamic port
const server = await createServer({ server: { port } });
await server.listen();