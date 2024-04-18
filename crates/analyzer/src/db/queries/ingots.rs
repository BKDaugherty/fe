use crate::{
    context::{Analysis, AnalyzerContext},
    display::Displayable,
    namespace::items::{ImplId, IngotId, IngotMode, ModuleId, ModuleSource, TraitId},
    namespace::scopes::ItemScope,
    namespace::types::TypeId,
    AnalyzerDb,
};
use fe_common::files::{SourceFileId, Utf8Path, Utf8PathBuf};
use indexmap::{
    map::{Entry, IndexMap},
    IndexSet,
};
use std::rc::Rc;

pub fn ingot_modules(db: &dyn AnalyzerDb, ingot: IngotId) -> Rc<[ModuleId]> {
    let files: Vec<(SourceFileId, Rc<Utf8PathBuf>)> = db
        .ingot_files(ingot)
        .iter()
        .map(|f| (*f, f.path(db.upcast())))
        .collect();

    // Create a module for every .fe source file.
    let file_mods = files
        .iter()
        .map(|(file, path)| {
            ModuleId::new(
                db,
                path.file_stem().unwrap(),
                ModuleSource::File(*file),
                ingot,
            )
        })
        .collect();

    // We automatically build a module hierarchy that matches the directory
    // structure. We don't (yet?) require a .fe file for each directory like
    // rust does. (eg `a/b.fe` alongside `a/b/`), but we do allow it (the
    // module's items will be everything inside the .fe file, and the
    // submodules inside the dir).
    //
    // Collect the set of all directories in the file hierarchy
    // (after stripping the common prefix from all paths).
    // eg given ["src/lib.fe", "src/a/b/x.fe", "src/a/c/d/y.fe"],
    // the dir set is {"a", "a/b", "a/c", "a/c/d"}.
    let file_path_prefix = &ingot.data(db).src_dir;
    let dirs = files
        .iter()
        .flat_map(|(_file, path)| {
            path.strip_prefix(file_path_prefix.as_str())
                .unwrap_or(path)
                .ancestors()
                .skip(1) // first elem of .ancestors() is the path itself
        })
        .collect::<IndexSet<&Utf8Path>>();

    let dir_mods = dirs
        // Skip the dirs that have an associated fe file; eg skip "a/b" if "a/b.fe" exists.
        .difference(
            &files
                .iter()
                .map(|(_file, path)| {
                    path.strip_prefix(file_path_prefix.as_str())
                        .unwrap_or(path)
                        .as_str()
                        .trim_end_matches(".fe")
                        .into()
                })
                .collect::<IndexSet<&Utf8Path>>(),
        )
        .filter_map(|dir| {
            dir.file_name()
                .map(|name| ModuleId::new(db, name, ModuleSource::Dir(dir.as_str().into()), ingot))
        })
        .collect::<Vec<_>>();

    [file_mods, dir_mods].concat().into()
}

pub fn ingot_root_module(db: &dyn AnalyzerDb, ingot: IngotId) -> Option<ModuleId> {
    let filename = match ingot.data(db).mode {
        IngotMode::Lib => "lib.fe",
        IngotMode::Main => "main.fe",
        IngotMode::StandaloneModule => return Some(ingot.all_modules(db)[0]),
    };

    ingot
        .all_modules(db)
        .iter()
        .find(|modid| modid.file_path_relative_to_src_dir(db) == filename)
        .copied()
}

// TODO: This implementation duplicates a fair bit of code from
// `module_impl_map`. It'd be great to not do so. If we still need
// module_impl_map, let's ensure that we refactor this to just use
// module_impl_map, or some shared helper.

// TODO: I'm actually not convinced this is sufficient, but it's
// better than existing? Without orphan rules, I think we also need to
// check external ingots. Think more about this.
/// Analyze every impl in the ingot. Ensure there are no duplicates.
pub fn ingot_impl_map(
    db: &dyn AnalyzerDb,
    ingot: IngotId,
) -> Analysis<Rc<IndexMap<(TraitId, TypeId), ImplId>>> {
    let mut map = IndexMap::<(TraitId, TypeId), ImplId>::new();
    let mut all_diagnostics = Vec::new();
    let ingot_all_modules = db.ingot_modules(ingot);
    for module_id in ingot_all_modules.iter() {
        let scope = ItemScope::new(db, *module_id);
        let module_all_impls = db.module_all_impls(*module_id);
        for impl_ in module_all_impls.value.iter() {
            let key = &(impl_.trait_id(db), impl_.receiver(db));
            match map.entry(*key) {
                Entry::Occupied(entry) => {
                    scope.duplicate_name_error(
                        &format!(
                            "duplicate `impl` blocks for trait `{}` for type `{}`",
                            key.0.name(db),
                            key.1.display(db)
                        ),
                        "",
                        entry.get().ast(db).span,
                        impl_.ast(db).span,
                    );
                }
                Entry::Vacant(entry) => {
                    entry.insert(*impl_);
                }
            }
        }

        all_diagnostics.extend_from_slice(
            &[
                module_all_impls.diagnostics,
                scope.diagnostics.take().into(),
            ]
            .concat(),
        );
    }

    Analysis::new(Rc::new(map), (*all_diagnostics.as_slice()).into())
}
