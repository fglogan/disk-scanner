# Screenshot Instructions for README

To add a screenshot to the README:

## Option 1: Manual Screenshot (Recommended)

1. **Start the application**:
   ```bash
   npm run tauri:dev
   ```

2. **Wait for the app window to open**

3. **Take a screenshot**:
   - **macOS**: Press `Cmd + Shift + 4`, then press `Space`, then click the app window
   - **Windows**: Press `Win + Shift + S`, select the window
   - **Linux**: Use your system's screenshot tool (e.g., `gnome-screenshot -w`)

4. **Save the screenshot** as `screenshot.png` in the project root

5. **Add to README**: Update the README.md to include:
   ```markdown
   ## Screenshot
   
   ![Disk Bloat Scanner Interface](screenshot.png)
   ```

## Option 2: Using Command Line (macOS)

```bash
# Start the app first
npm run tauri:dev

# In another terminal, wait 5 seconds then capture
sleep 5 && screencapture -w screenshot.png
# Then click the app window when the camera icon appears
```

## Recommended Screenshot Content

Capture the app showing:
- The Dashboard with scan results visible
- Multiple categories of bloat detected
- The sidebar navigation visible
- Dark theme UI clearly displayed
- Some file entries with safety indicators (green/amber/red)

## Dimensions

Recommended screenshot size:
- Width: 1200-1600px
- Height: 800-1000px
- Format: PNG
- Keep file size under 500KB if possible

## Alternative: GIF Demo

For a more dynamic demonstration, consider creating a short GIF:

```bash
# Use a tool like Kap (macOS), ScreenToGif (Windows), or Peek (Linux)
# Show the following workflow:
# 1. App launch
# 2. Click "Scan for Bloat"
# 3. Watch progress indicator
# 4. Review results with safety indicators
# Duration: 10-15 seconds max
```

Save as `demo.gif` and add to README:
```markdown
## Demo

![Disk Bloat Scanner Demo](demo.gif)
```

---

**Note**: Once you have your screenshot, delete this file and commit the image.
