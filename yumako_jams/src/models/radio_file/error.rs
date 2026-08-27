#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub(super)))]
pub enum RadioFileError {
    #[snafu(display("Unknown step type `{step_type}`. Please check for typos"))]
    UnknownStepTypeError {
        step_type: String,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}
