#[derive(Debug, PartialEq, diesel_derive_enum::DbEnum)]
pub enum Unit {
    G,
    Ml,
    Ts,
    Tbls,
}
