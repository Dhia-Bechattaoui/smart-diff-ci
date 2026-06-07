use regex::Regex;
use std::fs;
use std::path::Path;

/// Simple regex-based extractor for identifying imports in source files.
pub struct ImportParser {
    dart_regex: Regex,
    ts_regex: Regex,
    python_regex: Regex,
}

impl ImportParser {
    pub fn new() -> Self {
        Self {
            // Matches: import 'package/file.dart'; or part 'file.dart';
            dart_regex: Regex::new(r#"(?:import|part)\s+['"]([^'"]+)['"]"#).unwrap(),
            
            // Matches: import { X } from 'file'; or const X = require('file');
            ts_regex: Regex::new(r#"(?:from\s+['"]([^'"]+)['"]|require\s*\(\s*['"]([^'"]+)['"]\s*\))"#).unwrap(),
            
            // Matches: import module or from module import symbol
            python_regex: Regex::new(r#"(?:^|\n)\s*(?:from\s+([^\s]+)\s+import|import\s+([^\s]+))"#).unwrap(),
        }
    }

    /// Extracts imported modules/paths from a file based on its extension.
    pub fn extract_imports(&self, file_path: &Path) -> Vec<String> {
        let mut imports = Vec::new();
        
        let content = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(_) => return imports, // Silently skip unreadable or binary files
        };

        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            match ext {
                "dart" => {
                    for cap in self.dart_regex.captures_iter(&content) {
                        if let Some(m) = cap.get(1) {
                            imports.push(m.as_str().to_string());
                        }
                    }
                }
                "ts" | "tsx" | "js" | "jsx" => {
                    for cap in self.ts_regex.captures_iter(&content) {
                        if let Some(m) = cap.get(1).or_else(|| cap.get(2)) {
                            imports.push(m.as_str().to_string());
                        }
                    }
                }
                "py" => {
                    for cap in self.python_regex.captures_iter(&content) {
                        if let Some(m) = cap.get(1).or_else(|| cap.get(2)) {
                            // Strip any trailing commas or semicolons from Python imports if captured
                            let cleaned = m.as_str().trim_end_matches([',', ';']).to_string();
                            imports.push(cleaned);
                        }
                    }
                }
                _ => {} // Ignore unsupported extensions
            }
        }

        imports
    }
}
