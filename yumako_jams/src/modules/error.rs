use crate::modules::seeders::artist_seeder::ArtistSeederError;

#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub(super)))]
pub enum StreamModuleError {
    ArtistSeederError {
        source: ArtistSeederError,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}
