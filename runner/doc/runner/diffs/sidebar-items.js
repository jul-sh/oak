initSidebarItems({"fn":[["add_affected_crates","Checks if `crate_toml_path` has a direct or indirect dependency to any of the affected crates in `affected_crates_toml_path`. If so, adds `crate_toml_path` and any of its affected dependencies to `affected_crates_toml_path`."],["add_affected_protos","Adds `proto_path` to the list of `affected_protos` if `proto_path` or any of its imported protos imports any of the proto files in `affected_protos`."],["affected_protos","Returns the list of paths to all `.proto` files affected by the given list of modified files."],["all_affected_crates","Path to the `Cargo.toml` files for all crates that are either directly modified or have a dependency to a modified crate."],["crates_affected_by_files",""],["crates_affected_by_protos","Returns the paths to `Cargo.toml` files of crates affected by the changed proto files."],["directly_modified_crates","Returns the list of paths to `Cargo.toml` files for all crates that include at least one of the given modified files."],["find_crate_toml_file",""],["get_local_dependencies","Returns paths to `Cargo.toml` files of local crates (crates belonging to the Oak repo) that the given crate has a dependency to. Converts the relative dependency paths in `Cargo.toml` into paths relative to the repo’s root."],["imported_proto_files","Returns paths to all `.proto` files that `proto_file_path` imports."],["modified_files","Get all the files that have been modified in the given `scope`. If the scope is `Scope::All` returns `None` as the list of files in `ModifiedContent`. For `Scope::Commits` and `Scope::DiffToMain` the returned files include tracked files with unstaged changes, but do not include new files that are not tracked (i.e., not added to git yet). For `Scope::Commits(N)`, a positive number of commits must be given. In case of zero or a negative number, only the last commit will be considered for finding the modified files."]],"struct":[["ModifiedContent",""]]});