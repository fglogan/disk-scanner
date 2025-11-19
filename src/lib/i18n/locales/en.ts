export default {
  app: {
    title: "Disk Bloat Scanner",
    subtitle: "Clean up your disk space efficiently"
  },
  nav: {
    dashboard: "Dashboard",
    settings: "Settings", 
    help: "Help",
    about: "About"
  },
  actions: {
    scan: "Scan",
    cancel: "Cancel",
    delete: "Delete",
    cleanup: "Clean Up",
    refresh: "Refresh",
    select: "Select",
    selectAll: "Select All",
    deselectAll: "Deselect All",
    back: "Back",
    next: "Next",
    finish: "Finish",
    skip: "Skip",
    close: "Close",
    save: "Save",
    export: "Export",
    import: "Import"
  },
  status: {
    scanning: "Scanning...",
    idle: "Ready to scan",
    complete: "Scan complete",
    error: "Error occurred",
    cleaning: "Cleaning up...",
    cleanupComplete: "Cleanup complete"
  },
  dashboard: {
    diskUsage: "Disk Usage",
    totalSpace: "Total Space",
    usedSpace: "Used Space",
    freeSpace: "Free Space",
    lastScan: "Last Scan",
    never: "Never",
    selectDirectory: "Select Directory",
    scanDirectory: "Scan Directory", 
    cleanableSpace: "Cleanable Space",
    filesFound: "{count} files found"
  },
  categories: {
    largeFiles: "Large Files",
    duplicates: "Duplicate Files",
    devCaches: "Development Caches",
    projectBloat: "Project Bloat",
    systemJunk: "System Junk",
    tempFiles: "Temporary Files"
  },
  settings: {
    title: "Settings",
    theme: "Theme",
    language: "Language",
    minFileSize: "Minimum File Size",
    minDuplicateSize: "Minimum Duplicate Size",
    scanOptions: "Scan Options",
    excludePatterns: "Exclude Patterns",
    backgroundMonitoring: "Background Monitoring",
    scanInterval: "Scan Interval"
  },
  help: {
    title: "Help & Documentation",
    gettingStarted: "Getting Started",
    keyboardShortcuts: "Keyboard Shortcuts",
    faq: "Frequently Asked Questions",
    troubleshooting: "Troubleshooting",
    contact: "Contact Support"
  },
  tutorial: {
    welcome: "Welcome to Disk Bloat Scanner",
    step1Title: "Select a Directory",
    step1Text: "Choose a directory to scan for large files and duplicates",
    step2Title: "Review Results", 
    step2Text: "Examine the found files and select which ones to clean up",
    step3Title: "Clean Up",
    step3Text: "Remove selected files to free up disk space",
    skipTutorial: "Skip Tutorial",
    startTutorial: "Start Tutorial"
  },
  messages: {
    scanStarted: "Scan started",
    scanComplete: "Scan completed successfully",
    scanError: "Error during scan: {error}",
    deleteConfirm: "Are you sure you want to delete {count} files?",
    deleteSuccess: "Successfully deleted {count} files", 
    deleteError: "Error deleting files: {error}",
    settingsSaved: "Settings saved successfully",
    noFilesSelected: "No files selected",
    selectDirectoryFirst: "Please select a directory first",
    themeChanged: "Switched to {theme} mode"
  },
  accessibility: {
    skipToMain: "Skip to main content",
    loading: "Loading",
    expandMenu: "Expand menu",
    collapseMenu: "Collapse menu",
    sortBy: "Sort by {field}",
    filterBy: "Filter by {field}",
    selectFile: "Select file {name}",
    fileSize: "File size: {size}",
    lastModified: "Last modified: {date}"
  },
  performance: {
    title: "Performance Metrics",
    scanDuration: "Scan Duration",
    filesScanned: "Files Scanned",
    memoryUsage: "Memory Usage",
    cpuUsage: "CPU Usage",
    diskSpeed: "Disk Read Speed"
  }
};