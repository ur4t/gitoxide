use git_odb::pack;

#[test]
fn size_of_entry() {
    assert_eq!(
        std::mem::size_of::<pack::data::iter::Entry>(),
        104,
        "let's keep the size in check as we have many of them"
    );
}

mod new_from_header {
    use crate::{fixture_path, pack::SMALL_PACK};
    use git_odb::{pack, pack::data::iter::TrailerMode};
    use std::fs;

    #[test]
    fn generic_iteration() -> Result<(), Box<dyn std::error::Error>> {
        for trailer_mode in &[TrailerMode::AsIs, TrailerMode::Verify, TrailerMode::Restore] {
            let mut iter = pack::data::Iter::new_from_header(
                std::io::BufReader::new(fs::File::open(fixture_path(SMALL_PACK))?),
                *trailer_mode,
            )?;

            let num_objects = iter.len();
            assert_eq!(iter.kind(), pack::data::Kind::V2);
            assert_eq!(num_objects, 42);
            assert_eq!(iter.by_ref().take(42 - 1).count(), num_objects - 1);
            assert_eq!(iter.len(), 1);
            assert_eq!(
                iter.next().expect("last object")?.trailer.expect("trailer id"),
                pack::data::File::at(fixture_path(SMALL_PACK))?.checksum(),
                "last object contains the trailer - a hash over all bytes in the pack"
            );
            assert_eq!(iter.len(), 0);
        }
        Ok(())
    }

    #[test]
    fn restore_missing_trailer() -> Result<(), Box<dyn std::error::Error>> {
        let pack = fs::read(fixture_path(SMALL_PACK))?;
        let mut iter =
            pack::data::Iter::new_from_header(std::io::BufReader::new(&pack[..pack.len() - 20]), TrailerMode::Restore)?;
        let num_objects = iter.len();
        assert_eq!(iter.by_ref().take(42 - 1).count(), num_objects - 1);
        assert_eq!(
            iter.next().expect("last object")?.trailer.expect("trailer id"),
            pack::data::File::at(fixture_path(SMALL_PACK))?.checksum(),
            "the correct checksum should be restored"
        );
        Ok(())
    }

    #[test]
    fn restore_partial_pack() -> Result<(), Box<dyn std::error::Error>> {
        let pack = fs::read(fixture_path(SMALL_PACK))?;
        let mut iter =
            pack::data::Iter::new_from_header(std::io::BufReader::new(&pack[..pack.len() / 2]), TrailerMode::Restore)?;
        let mut num_objects = 0;
        while let Some(entry) = iter.next() {
            let entry = entry?;
            num_objects += 1;
            assert!(
                entry.trailer.is_some(),
                "every entry has a trailer as we don't know when an object will fail - thus we never fail"
            );
        }
        assert_eq!(num_objects, 12);
        assert_eq!(
            iter.len(),
            0,
            "it will never return any more objects (right now), so nothing is left"
        );
        Ok(())
    }
}
