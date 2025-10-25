// Shared utility functions for formatting and common operations

/**
 * Format file size in human-readable format
 * @param {number} sizeInKB - Size in kilobytes
 * @returns {string} Formatted size string
 */
export function formatSize(sizeInKB) {
  if (sizeInKB < 1024) {
    return `${sizeInKB.toFixed(1)} KB`;
  }
  
  const sizeInMB = sizeInKB / 1024;
  if (sizeInMB < 1024) {
    return `${sizeInMB.toFixed(1)} MB`;
  }
  
  const sizeInGB = sizeInMB / 1024;
  return `${sizeInGB.toFixed(2)} GB`;
}

/**
 * Format file size in MB (for values already in MB)
 * @param {number} sizeInMB - Size in megabytes
 * @returns {string} Formatted size string
 */
export function formatSizeMB(sizeInMB) {
  if (sizeInMB < 1) {
    return `${(sizeInMB * 1024).toFixed(1)} KB`;
  }
  
  if (sizeInMB < 1024) {
    return `${sizeInMB.toFixed(1)} MB`;
  }
  
  const sizeInGB = sizeInMB / 1024;
  return `${sizeInGB.toFixed(2)} GB`;
}

/**
 * Get safety color class based on safety level
 * @param {string} safety - Safety level ('safe', 'caution', 'dangerous')
 * @returns {string} CSS color class
 */
export function getSafetyColor(safety) {
  switch (safety) {
    case 'safe':
      return 'text-green-400';
    case 'caution':
      return 'text-yellow-400';
    case 'dangerous':
      return 'text-red-400';
    default:
      return 'text-gray-400';
  }
}

/**
 * Get safety background class based on safety level
 * @param {string} safety - Safety level ('safe', 'caution', 'dangerous')
 * @returns {string} CSS background class
 */
export function getSafetyBg(safety) {
  switch (safety) {
    case 'safe':
      return 'bg-green-900/30 border-green-700';
    case 'caution':
      return 'bg-yellow-900/30 border-yellow-700';
    case 'dangerous':
      return 'bg-red-900/30 border-red-700';
    default:
      return 'bg-gray-900/30 border-gray-700';
  }
}

/**
 * Get safety badge text
 * @param {string} safety - Safety level
 * @returns {string} Badge text
 */
export function getSafetyBadge(safety) {
  switch (safety) {
    case 'safe':
      return '🟢 Safe to Delete';
    case 'caution':
      return '🟡 Review First';
    case 'check':
      return '🟡 Review First';
    default:
      return '🔴 Caution';
  }
}

/**
 * Check if a path is critical (should not be deleted)
 * @param {string} path - File path
 * @returns {boolean} True if path is critical
 */
export function isCriticalPath(path) {
  return (
    path.includes('/src/') ||
    path.includes('/lib/') ||
    path.includes('/.git/') ||
    path.includes('/node_modules/') ||
    /\.(rs|js|ts|py|go|cpp|java|rb|php|swift|kt)$/.test(path)
  );
}

/**
 * Truncate text with ellipsis
 * @param {string} text - Text to truncate
 * @param {number} maxLength - Maximum length
 * @returns {string} Truncated text
 */
export function truncateText(text, maxLength = 50) {
  if (text.length <= maxLength) {
    return text;
  }
  return text.substring(0, maxLength - 3) + '...';
}