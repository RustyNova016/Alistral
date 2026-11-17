use chrono::DateTime;
use chrono::Local;
use serde::Deserialize;
use serde::Serialize;
use spire_enum::prelude::delegate_impl;
use spire_enum::prelude::delegated_enum;

use crate::models::cli_components::reports::utils::display_table::ReportTableSettings;
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

// No docs
#[cfg_attr(
    not(feature = "docs"),
    delegated_enum(extract_variants(derive(
        Debug,
        Eq,
        PartialEq,
        Clone,
        Default,
        Serialize,
        Deserialize
    )))
)]
#[cfg_attr(not(feature = "docs"), derive(Debug, Serialize, Deserialize, Clone))]
// Doc mode
#[cfg_attr(
    feature = "docs",
    delegated_enum(extract_variants(derive(
        Debug,
        Eq,
        PartialEq,
        Clone,
        Default,
        Serialize,
        Deserialize,
        schemars::JsonSchema
    )))
)]
#[cfg_attr(
    feature = "docs",
    derive(Debug, Serialize, Deserialize, Clone, schemars::JsonSchema)
)]
#[serde(tag = "type")]
pub enum SectionType {
    GeneralStats,

    // --- Group By ---
    GroupByWeekDay,
    GroupByMonth,

    // --- Tops ---
    TopArtists {
        table_settings: ReportTableSettings,
    },
    Toplabels {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },
    TopRecordings {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },
    TopReleases {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },
    TopReleaseGroups {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },

    // --- Discoveries ---
    DiscoveredArtists {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },
    DiscoveredLabels {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },
    DiscoveredRecordings {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },
    DiscoveredReleases {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },
    DiscoveredReleaseGroups {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },

    // --- All time changes ---
    AllTimeChangesArtists {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },
    AllTimeChangesLabels {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },
    AllTimeChangesRecordings {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },
    AllTimeChangesReleases {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },
    AllTimeChangesReleaseGroups {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },

    // ---  Debuts ---
    DebutsRecordings {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },
    DebutsReleases {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },

    // --- Comebacks ---
    ComebackArtists {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },
    ComebackLabels {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },
    ComebackRecordings {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },
    ComebackReleases {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },
    ComebackReleaseGroups {
        #[serde(default)]
        table_settings: ReportTableSettings,
    },
}
