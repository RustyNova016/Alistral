use crate::Artist;
use crate::Label;
use crate::MainEntity;
use crate::Recording;
use crate::Release;
use crate::Work;

impl From<Artist> for MainEntity {
    fn from(value: Artist) -> MainEntity {
        MainEntity::Artist(value)
    }
}

impl From<Label> for MainEntity {
    fn from(value: Label) -> MainEntity {
        MainEntity::Label(value)
    }
}

impl From<Recording> for MainEntity {
    fn from(value: Recording) -> MainEntity {
        MainEntity::Recording(value)
    }
}

impl From<Release> for MainEntity {
    fn from(value: Release) -> MainEntity {
        MainEntity::Release(value)
    }
}

impl From<Work> for MainEntity {
    fn from(value: Work) -> MainEntity {
        MainEntity::Work(value)
    }
}
