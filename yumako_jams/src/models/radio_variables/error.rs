#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub(super)))]
pub enum RadioInputsError {
    #[snafu(display("The layer `{layer_id}` recieved an input json that is not an object."))]
    LayerVariableNotObject {
        layer_id: String,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}
