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
                // If other files depend on this file, add them to the queue to be checked
                if let Some(dependents) = self.inverse_edges.get(&file) {
                    for dep in dependents {
                        if !affected.contains(dep) {
                            queue.push(dep.clone());
                        }
                    }
                }
            }
        }

        affected
    }
}
