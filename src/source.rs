use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum SourceKind{
    Netease,
    Applemusic,
    Other
}

#[derive(Clone, Serialize)]
pub struct  Source{
    id: String,
    kind: SourceKind
}
