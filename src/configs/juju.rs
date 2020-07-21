use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct JujuConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for JujuConfig<'a> {
    fn new() -> Self {
        JujuConfig {
            format: "via [$symbol$version$model]($style) ",
            symbol: "🔮 ",
            style: "fg:#E95420",
            disabled: false,
        }
    }
}
