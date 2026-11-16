use std::collections::HashMap;

use crate::tools::report::section_types::SectionType;

#[derive(Debug, serde::Serialize, serde::Deserialize, Default, Clone)]
#[cfg_attr(feature = "docs", derive(schemars::JsonSchema))]
pub struct ReportConfig {
    presets: HashMap<String, ReportPreset>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default, Clone)]
#[cfg_attr(feature = "docs", derive(schemars::JsonSchema))]
pub struct ReportPreset {
    pub sections: Vec<SectionType>,
}

impl ReportConfig {
    pub fn get_preset(&self, name: &str) -> Option<&ReportPreset> {
        self.presets.get(name)
    }

    pub fn get_default_preset() -> ReportPreset {
        ReportPreset {
            sections: vec![
                SectionType::GeneralStats(Default::default()),
                SectionType::TopRecordings(Default::default()),
                SectionType::DiscoveredRecordings(Default::default()),
                SectionType::ComebackRecordings(Default::default()),
                SectionType::DebutsRecordings(Default::default()),
                SectionType::AllTimeChangesRecordings(Default::default()),
                SectionType::TopArtists(Default::default()),
                SectionType::DiscoveredArtists(Default::default()),
                SectionType::ComebackArtists(Default::default()),
                SectionType::AllTimeChangesArtists(Default::default()),
                SectionType::TopReleases(Default::default()),
                SectionType::DiscoveredReleases(Default::default()),
                SectionType::ComebackReleases(Default::default()),
                SectionType::DebutsReleases(Default::default()),
                SectionType::AllTimeChangesReleases(Default::default()),
                SectionType::TopReleaseGroups(Default::default()),
                SectionType::ComebackReleaseGroups(Default::default()),
                SectionType::DiscoveredReleaseGroups(Default::default()),
                SectionType::AllTimeChangesReleaseGroups(Default::default()),
                SectionType::Toplabels(Default::default()),
                SectionType::ComebackLabels(Default::default()),
                SectionType::DiscoveredLabels(Default::default()),
                SectionType::AllTimeChangesLabels(Default::default()),
                SectionType::GroupByWeekDay(Default::default()),
                SectionType::GroupByMonth(Default::default()),
            ],
        }
    }
}
