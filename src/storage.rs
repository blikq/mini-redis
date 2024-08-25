use crate::datatypes::DataType;

pub struct Metdata {
    pub key: DataType,
    pub value: DataType,
}

pub struct Storage <T, Y> {
    pub key: T,
    pub value: Y,
    pub metadata: Metdata,
}