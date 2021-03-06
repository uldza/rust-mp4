use super::{Mp4File, Kind, Header, Atom};

#[derive(Debug, Clone)]
pub struct Ignore {
    header: Header
}

impl Ignore {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str>{
        let curr_offset = f.offset();
        f.seek(curr_offset+header.data_size);
        f.offset_inc(header.data_size);
        Ok(Ignore{
            header: header,
        })
    }
}