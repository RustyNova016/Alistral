use crate::modules::seeders::artist_seeder::ArtistSeederError;
use crate::modules::seeders::release_seeder::ReleaseSeederError;

#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub(super)))]
pub enum StreamModuleError {
    ArtistSeederError {
        source: ArtistSeederError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    ReleaseSeederError {
        source: ReleaseSeederError,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}
