pub mod label_as_artist;
pub mod missing_artist_link;
pub mod missing_recording_link;
use crate::clippy::lints::missing_release_barcode::MissingBarcodeLint;
use crate::clippy::lints::missing_remix_rel::MissingRemixRelLint;
use crate::clippy::lints::missing_remixer_rel::MissingRemixerRelLint;
use crate::clippy::lints::missing_work::MissingWorkLint;
use crate::clippy::lints::soundtrack_without_disambiguation::SoundtrackWithoutDisambiguationLint;
use crate::clippy::lints::suspicious_remix::SuspiciousRemixLint;

pub mod dash_eti;
pub mod missing_release_barcode;
pub mod missing_remix_rel;
pub mod missing_remixer_rel;
pub mod missing_work;
pub mod soundtrack_without_disambiguation;
pub mod suspicious_remix;
//pub mod missing_work_language; // Need work languages
//pub mod missing_isrc;

pub enum MusicbrainzLints {
    MissingWorkLint(MissingWorkLint),
    MissingBarcodeLint(MissingBarcodeLint),
    MissingRemixRelLint(MissingRemixRelLint),
    MissingRemixerRelLint(MissingRemixerRelLint),
    SoundtrackWithoutDisambiguationLint(SoundtrackWithoutDisambiguationLint),
    SuspiciousRemixLint(SuspiciousRemixLint),
}
