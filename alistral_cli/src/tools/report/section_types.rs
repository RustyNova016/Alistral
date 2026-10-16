use chrono::DateTime;
use chrono::Local;
use spire_enum::prelude::delegate_impl;
use spire_enum::prelude::delegated_enum;

use crate::tools::report::ReportListenData;
use crate::tools::report::sections::error::SectionsError;

pub(super) trait SectionTypeTrait {
    async fn generate(
        &self,
        listens: &ReportListenData,

        start_time: DateTime<Local>,
        end_time: DateTime<Local>,
        previous_start_time: DateTime<Local>,
    ) -> Result<String, SectionsError>;
}

#[delegate_impl]
impl SectionTypeTrait for SectionType {
    async fn generate(
        &self,
        listens: &ReportListenData,

        start_time: DateTime<Local>,
        end_time: DateTime<Local>,
        previous_start_time: DateTime<Local>,
    ) -> Result<String, SectionsError>;
}

#[delegated_enum(extract_variants(derive(Debug, Eq, PartialEq, Clone, Copy, Default)))]
pub enum SectionType {
    GeneralStats,

    // --- Group By ---
    GroupByWeekDay,
    GroupByMonth,

    // --- Tops ---
    TopArtists,
    Toplabels,
    TopRecordings,
    TopReleases,
    TopReleaseGroups,

    // --- Discoveries ---
    DiscoveredArtists,
    DiscoveredLabels,
    DiscoveredRecordings,
    DiscoveredReleases,
    DiscoveredReleaseGroups,

    // --- All time changes ---
    AllTimeChangesArtists,
    AllTimeChangesLabels,
    AllTimeChangesRecordings,
    AllTimeChangesReleases,
    AllTimeChangesReleaseGroups,

    // ---  Debuts ---
    DebutsRecordings,
    DebutsReleases,
}
