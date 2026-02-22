use crate::keyboard::LayoutVariant;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub layout_variant: LayoutVariant,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            layout_variant: LayoutVariant::Mac,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_layout_is_mac() {
        let config = Config::default();
        assert_eq!(config.layout_variant, LayoutVariant::Mac);
    }

    #[test]
    fn test_config_roundtrip_json() {
        let config = Config {
            layout_variant: LayoutVariant::Pc,
        };
        let json = serde_json::to_string(&config).unwrap();
        let loaded: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.layout_variant, LayoutVariant::Pc);
    }
}
