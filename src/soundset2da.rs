use nwn_files::types::{
    X2daRow,
    X2daItem,
    X2daError,
};

#[derive(Debug, Clone)]
pub struct Soundset2da {
    pub label: Option<String>,
    pub res_ref: Option<String>,
    pub str_ref: Option<u32>,
    pub gender: Option<u32>,
    pub soundset_type: Option<u32>,
}

impl X2daRow for Soundset2da {
    const SIZE: usize = 5;

    type Row = [Option<Box<dyn X2daItem>>; 5];

    fn to_row(&self) -> Self::Row {
        [
            self.label.to_owned().map(X2daItem::boxed),
            self.res_ref.to_owned().map(X2daItem::boxed),
            self.str_ref.to_owned().map(X2daItem::boxed),
            self.gender.to_owned().map(X2daItem::boxed),
            self.soundset_type.to_owned().map(X2daItem::boxed),
        ]
    }

    fn from_strings(mut strings: Vec<Option<String>>)
        -> Result<Self, X2daError>
    {
        let label = strings.remove(0);
        let res_ref = strings.remove(0);

        let str_ref = strings.remove(0)
            .map(|v| v.parse::<u32>())
            .transpose()
            .or(Err(X2daError::InvalidTableItem))?;

        let gender = strings.remove(0)
            .map(|v| v.parse::<u32>())
            .transpose()
            .or(Err(X2daError::InvalidTableItem))?;

        let soundset_type = strings.remove(0)
            .map(|v| v.parse::<u32>())
            .transpose()
            .or(Err(X2daError::InvalidTableItem))?;

        Ok(Soundset2da {
            label,
            res_ref,
            str_ref,
            gender,
            soundset_type,
        })
    }
}