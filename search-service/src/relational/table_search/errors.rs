use thiserror::Error;

#[derive(Error, Debug)]
pub enum TableSearchError {
    #[error("Table not found in graph: {0}")]
    TableNotFoundInGraph(String),

    #[error("Edge not found int graph")]
    EdgeNotFoundInGraph,

    #[error("Atributes can't be joined")]
    AtributesCantBeJoined,

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
