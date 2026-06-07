use crate::graph::DependencyGraph;
use crate::parser::ImportParser;
use ignore::WalkBuilder;
use std::path::Path;

/// Crawls the project to build the comprehensive dependency graph.
pub struct DependencyCrawler {
    parser: ImportParser,
}

impl DependencyCrawler {
    pub fn new() -> Self {
        Self {
            parser: ImportParser::new(),
        }
    }

    /// Traverses the directory recursively (respecting .gitignore),
    /// parses imports from all supported files, and builds the DAG.
    pub fn build_graph(&self, root_dir: &Path) -> DependencyGraph {
        let mut graph = DependencyGraph::new();

        // WalkBuilder automatically reads and respects `.gitignore` rules
        for result in WalkBuilder::new(root_dir).build() {
            if let Ok(entry) = result {
                let path = entry.path();
                
                if path.is_file() {
                    let imports = self.parser.extract_imports(path);
                    
                    // Normalize the file path to a string
                    // Note: We strip the leading `./` if it exists to match git diff formatting
                    let file_str = path
                        .strip_prefix(root_dir)
                        .unwrap_or(path)
                        .to_string_lossy()
                        .to_string();
                    
                    for import in imports {
                        // Graph stores `import` as a dependency of `file_str`.
                        // Therefore, if `import` changes, `file_str` is affected.
                        graph.add_dependency(&file_str, &import);
                    }
                }
            }
        }

        graph
    }
}
