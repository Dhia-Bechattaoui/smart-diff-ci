use std::collections::{HashMap, HashSet};

/// A directed graph representing file dependencies within the project.
pub struct DependencyGraph {
    /// Maps a file to a list of files that IMPORT it (inverse dependencies).
    /// This is optimized for resolving what is affected by a change.
    inverse_edges: HashMap<String, HashSet<String>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            inverse_edges: HashMap::new(),
        }
    }

    /// Records that `file` imports `dependency`
    pub fn add_dependency(&mut self, file: &str, dependency: &str) {
        self.inverse_edges
            .entry(dependency.to_string())
            .or_default()
            .insert(file.to_string());
    }

    /// Recursively finds all files that transitively depend on the `changed_files`.
    pub fn find_affected_files(&self, changed_files: &[String]) -> HashSet<String> {
        let mut affected = HashSet::new();
        let mut queue: Vec<String> = changed_files.to_vec();

        while let Some(file) = queue.pop() {
            // If we successfully insert (meaning it wasn't already processed)
            if affected.insert(file.clone()) {
                // Since imports can be relative (e.g. `../utils.dart`) or package-based 
                // (`package:law_max/utils.dart`), and Git diffs are relative to the root 
                // (`lib/utils.dart`), we use fuzzy substring matching for the MVP to connect them.
                let normalized_file = file.replace("lib/", "").replace("src/", "");
                
                for (import_path, dependents) in &self.inverse_edges {
                    let normalized_import = import_path.replace("package:", "").replace("../", "").replace("./", "");
                    
                    if normalized_import.ends_with(&normalized_file) || normalized_file.ends_with(&normalized_import) {
                        for dep in dependents {
                            if !affected.contains(dep) {
                                queue.push(dep.clone());
                            }
                        }
                    }
                }
            }
        }

        affected
    }
}
