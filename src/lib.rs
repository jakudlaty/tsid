pub struct TsidFactory {
    node_bits: u8,
}

pub struct TsidFactoryBuilder {
    node_bits: u8,
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

    pub fn build(self) -> TsidFactory {
        TsidFactory {
            node_bits: self.node_bits,
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

#[cfg(test)]
mod tests {
    use crate::TsidFactoryBuilder;

    #[test]
    fn builder_should_set_node_bits() {
        let factory_under_test = TsidFactoryBuilder::new()
            .with_node_bits(8)
            .build();

        assert_eq!(8, factory_under_test.node_bits);
    }
}