use derive_more::FromStr;

#[derive(FromStr, Debug, Eq, PartialEq, Clone, Copy)]
#[from_str(rename_all = "snake_case")]
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
