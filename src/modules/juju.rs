use std::fs::read_to_string;
use std::path::PathBuf;

use super::{Context, Module, RootModuleConfig};

use crate::configs::juju::JujuConfig;
use crate::formatter::StringFormatter;

use yaml_rust::{Yaml, YamlLoader};

fn get_yaml<P: Into<PathBuf>>(path: P) -> Option<Yaml> {
    let contents = read_to_string(path.into()).ok()?;
    Some(YamlLoader::load_from_str(&contents).ok()?.get(0)?.clone())
}

/// Creates a module that display the current Juju version and model
///
/// Will display the Juju version if the Juju snap is installed.
/// Will also display the active controller and model if they exist.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("juju");
    let config = JujuConfig::try_load(module.config);

    // Read version information directly from the snap directory,
    // instead of querying the CLI about version information.
    let doc = get_yaml("/snap/juju/current/meta/snap.yaml")?;
    let version = doc["version"].as_str()?;

    // Optionally, calculate the controller and model, and display
    // it in parentheses. The user may not have an active model,
    // in which case we just show the version number.
    let model = std::env::var("HOME").ok().and_then(|h| {
        let juju_dir = PathBuf::from(h).join(".local/share/juju/");
        let doc = get_yaml(juju_dir.join("controllers.yaml"))?;

        doc["current-controller"].as_str().and_then(|c| {
            let doc = get_yaml(juju_dir.join("models.yaml"))?;
            let model = doc["controllers"][c]["current-model"].as_str()?;

            Some(format!(" ({}:{})", c, model))
        })
    });

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "version" => Some(Ok(version)),
                "model" => model.as_ref().map(|m| Ok(m.as_str())),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::error!("Error in module `juju`: \n{}", error);
            return None;
        }
    });

    Some(module)
}
