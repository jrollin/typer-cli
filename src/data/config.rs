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
