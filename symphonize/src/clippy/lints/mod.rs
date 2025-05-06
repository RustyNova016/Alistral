use crate::clippy::lints::dash_eti::DashETILintRes;
use crate::clippy::lints::missing_release_barcode::MissingBarcodeLint;
use crate::clippy::lints::missing_release_barcode::MissingBarcodeLintRes;
use crate::clippy::lints::missing_remix_rel::MissingRemixRelLint;
use crate::clippy::lints::missing_remix_rel::MissingRemixRelLintRes;
use crate::clippy::lints::missing_remixer_rel::MissingRemixerRelLint;
use crate::clippy::lints::missing_remixer_rel::MissingRemixerRelLintRes;
use crate::clippy::lints::missing_work::MissingWorkLint;
use crate::clippy::lints::missing_work::MissingWorkLintRes;
use crate::clippy::lints::soundtrack_without_disambiguation::SoundtrackWithoutDisambiguationLint;
use crate::clippy::lints::soundtrack_without_disambiguation::SoundtrackWithoutDisambiguationLintRes;
use crate::clippy::lints::suspicious_remix::SuspiciousRemixLint;
use crate::clippy::lints::suspicious_remix::SuspiciousRemixLintRes;

pub mod dash_eti;
pub mod missing_recording_links;
pub mod missing_release_barcode;
pub mod missing_remix_rel;
pub mod missing_remixer_rel;
pub mod missing_work;
pub mod soundtrack_without_disambiguation;
pub mod suspicious_remix;
//pub mod missing_work_language; // Need work languages
//pub mod missing_isrc;

pub enum MusicbrainzLints {
    DashETILint,
    MissingWorkLint,
    MissingBarcodeLint,
    MissingRemixRelLint,
    MissingRemixerRelLint,
    SoundtrackWithoutDisambiguationLint,
    SuspiciousRemixLint,
}

pub enum MusicbrainzLintsRes {
    DashETILint(DashETILintRes),
    MissingWorkLint(MissingWorkLintRes),
    MissingBarcodeLint(MissingBarcodeLintRes),
    MissingRemixRelLint(MissingRemixRelLintRes),
    MissingRemixerRelLint(MissingRemixerRelLintRes),
    SoundtrackWithoutDisambiguationLint(SoundtrackWithoutDisambiguationLintRes),
    SuspiciousRemixLint(SuspiciousRemixLintRes),
}
