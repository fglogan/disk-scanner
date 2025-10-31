# Feature Testing Guide - PACS & Architecture Visualization

**Date:** October 30, 2025  
**Status:** ✅ Both features implemented and ready for testing

## 🚀 Quick Test Instructions

### 1. Launch Application
```bash
cd /Users/tempext/Projects/disk-bloat-scanner
npm run tauri:dev
```

### 2. Test PACS (Project Auditor & Compliance Scanner)
1. **Navigate:** Click "PACS Compliance" in sidebar OR press Alt+A
2. **Run Scan:** Click "Run PACS Scan" button
3. **Select Project:** Choose `/Users/tempext/Projects/disk-bloat-scanner` (default)
4. **Wait for Results:** Should complete in 10-30 seconds
5. **Review Report:** Check compliance score, findings, and recommendations

**Expected Results:**
- Compliance score: 85-95/100
- Standards checked: TES-2025, EDGS, LAIO, OpenSpec, BloomPlaybook
- Findings categorized by severity (Critical, High, Medium, Low)
- Baseline creation and drift detection

### 3. Test Architecture Visualization
1. **Navigate:** Click "Architecture Visualization" in sidebar OR press Alt+V
2. **Run Analysis:** Click "Run Analysis" button
3. **Select Project:** Choose `/Users/tempext/Projects/disk-bloat-scanner` (default)
4. **Wait for Analysis:** Should complete in 5-15 seconds
5. **Explore Results:** Check all 4 tabs (Overview, Modules, Diagram, Metrics)

**Expected Results:**
- **Overview:** Project stats, language breakdown (Rust, TypeScript, JavaScript)
- **Modules:** 50+ files analyzed with functions/classes
- **Diagram:** Mermaid code showing module relationships
- **Metrics:** Architecture quality scores and maintainability index

## 🔍 Detailed Testing Scenarios

### PACS Testing Scenarios

#### Scenario 1: First-time Scan
- **Action:** Run PACS scan on fresh project
- **Expected:** Baseline creation, full compliance analysis
- **Verify:** Baseline file created in `.pacs/baseline.json`

#### Scenario 2: Configuration Testing
- **Action:** Modify PACS config (enable/disable standards)
- **Expected:** Scan results reflect configuration changes
- **Verify:** Only selected standards are checked

#### Scenario 3: Drift Detection
- **Action:** Run scan twice without changes
- **Expected:** No drift detected, consistent scores
- **Verify:** Drift status shows "No changes detected"

### Architecture Visualization Testing Scenarios

#### Scenario 1: Multi-language Analysis
- **Action:** Analyze current project (Rust + TypeScript + JavaScript)
- **Expected:** All languages detected and parsed
- **Verify:** Language breakdown shows correct file counts

#### Scenario 2: Module Exploration
- **Action:** Click on modules in the Modules tab
- **Expected:** Expandable details showing functions/classes
- **Verify:** Function names, line numbers, visibility correctly shown

#### Scenario 3: Diagram Generation
- **Action:** View Diagram tab after analysis
- **Expected:** Mermaid code with colored nodes by language
- **Verify:** Valid Mermaid syntax, proper node relationships

## 🐛 Troubleshooting

### Common Issues

#### PACS Scan Fails
- **Symptom:** Error message during scan
- **Solution:** Check project path exists and is readable
- **Debug:** Check browser console for detailed error messages

#### Architecture Analysis Hangs
- **Symptom:** Analysis never completes
- **Solution:** Check for very large files or deep directory structures
- **Debug:** Check Tauri logs for Tree-sitter parsing errors

#### UI Not Responsive
- **Symptom:** Buttons don't work, tabs don't switch
- **Solution:** Refresh the application (Cmd+R)
- **Debug:** Check browser console for JavaScript errors

### Performance Expectations

#### PACS Performance
- **Small projects** (<100 files): 5-10 seconds
- **Medium projects** (100-500 files): 10-30 seconds
- **Large projects** (500+ files): 30-60 seconds

#### ArchViz Performance
- **Small projects** (<50 code files): 2-5 seconds
- **Medium projects** (50-200 code files): 5-15 seconds
- **Large projects** (200+ code files): 15-30 seconds

## ✅ Success Criteria

### PACS Success Indicators
- [ ] Compliance score calculated (0-100)
- [ ] At least 5 standards checked
- [ ] Findings categorized by severity
- [ ] Baseline file created
- [ ] Report saved to `.pacs/reports/`

### ArchViz Success Indicators
- [ ] File count matches project reality
- [ ] Language breakdown accurate
- [ ] Functions/classes extracted correctly
- [ ] Mermaid diagram syntactically valid
- [ ] Metrics calculated (complexity, maintainability)

## 🎯 Integration Testing

### Cross-feature Testing
1. **Run both scans simultaneously:** Verify no conflicts
2. **Switch between features:** Ensure UI state preserved
3. **Keyboard navigation:** Test Alt+A and Alt+V shortcuts
4. **Error handling:** Verify graceful failure modes

### Data Consistency
- Both features should analyze same project structure
- File counts should be consistent between features
- No data corruption or interference

## 📊 Expected Sample Results

### PACS Sample Output
```json
{
  "compliance_score": 92.5,
  "standards_checked": ["TES-2025", "EDGS", "LAIO", "OpenSpec"],
  "findings_count": {
    "critical": 0,
    "high": 2,
    "medium": 5,
    "low": 8
  }
}
```

### ArchViz Sample Output
```json
{
  "file_count": 67,
  "language_breakdown": {
    "rust": 15,
    "typescript": 25,
    "javascript": 12,
    "json": 15
  },
  "metrics": {
    "maintainability_index": 87.3,
    "coupling_score": 0.4,
    "total_functions": 234
  }
}
```

---

**Status:** ✅ Ready for comprehensive testing  
**Next Step:** Execute test scenarios and validate all functionality