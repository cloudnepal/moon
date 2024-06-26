use moon_cache::cache_item;
use moon_common::path::WorkspaceRelativePathBuf;
use moon_common::Id;
use rustc_hash::FxHashMap;

cache_item!(
    pub struct ProjectsCacheState {
        pub last_hash: String,
        pub projects: FxHashMap<Id, WorkspaceRelativePathBuf>,
    }
);
