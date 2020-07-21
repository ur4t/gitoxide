use super::{Error, Mode, Outcome};
use crate::{pack, pack::index};
use git_features::progress::{self, Progress};
use std::time::SystemTime;

impl index::File {
    pub(crate) fn inner_verify_with_indexed_lookup<P, C>(
        &self,
        _thread_limit: Option<usize>,
        _mode: Mode,
        _make_cache: impl Fn() -> C + Send + Sync,
        mut progress: progress::DoOrDiscard<P>,
        pack: &pack::data::File,
    ) -> Result<Outcome, Error>
    where
        P: Progress,
        <P as Progress>::SubProgress: Send,
        C: pack::cache::DecodeEntry,
    {
        let offsets = {
            let mut indexing_progress = progress.add_child("preparing pack offsets");
            indexing_progress.init(Some(self.num_objects), Some("objects"));
            let then = SystemTime::now();
            let iter = self.sorted_offsets().into_iter();
            let elapsed = then.elapsed().expect("system time").as_secs_f32();
            indexing_progress.info(format!(
                "in {:.02}s ({} objects/s)",
                elapsed,
                self.num_objects as f32 / elapsed
            ));
            iter
        };
        pack::graph::DeltaTree::from_sorted_offsets(offsets, pack.path(), progress.add_child("indexing"))?;

        unimplemented!()
    }
}
