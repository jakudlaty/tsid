pub struct TsidFactory {}

pub struct TsidFactoryBuilder {
    node_bits: u8
}

impl TsidFactoryBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_node_bits(self, node_bits: u8) -> Self {
        Self {
            node_bits
        }
    }

    pub fn build<T>(self) -> TsidFactory {
        TsidFactory {

        }
    }
}

impl Default for TsidFactoryBuilder {
    fn default() -> Self {
        Self {
            node_bits: 0,
        }
    }
}


pub struct TSID {}

