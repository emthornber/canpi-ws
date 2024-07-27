use crate::errors::CanPiAppError;
use ini::Ini;
use std::path::Path;

#[macro_export]
macro_rules! pkg_name {
    () => {
        env!("CARGO_BIN_NAME")
    };
}

const CANGRID_URI: &str = "localhost:5550";
const CFGFILE: &str = "canpi-ws.cfg";

/// Structure that holds configuration items expanded from EVs and static text
#[derive(Debug)]
pub struct CanpiConfig {
    /// Host and port that provides a CBus channel
    pub cangrid_uri: String,
    /// Host and port of the web service
    pub host_port: Option<String>,
}

impl CanpiConfig {
    /// Creates a new instance of the structure
    ///
    /// The contents of the EV CANPIWS_HOME is used with the const text above to create path strings
    /// for items.
    ///
    /// If CANPIWS_HOME is not defined or does not point to a valid directory then an error result is
    /// returned.
    ///
    /// If the EV HOST_PORT is not defined then the entry in the struct is set to None.  No further
    /// validation is done if the EV does exist.
    ///
    pub fn new() -> Result<CanpiConfig, CanPiAppError> {
        let h = std::env::var("CANPIWS_HOME");
        if let Ok(home) = h {
            let cpp_home = home;
            if !Path::new(&cpp_home).is_dir() {
                return Err(CanPiAppError::NotFound(
                    "EV CPPANEL_HOME not a directory".to_string(),
                ));
            }

            let mut cfg = CanpiConfig {
                cangrid_uri: CANGRID_URI.to_string(),
                host_port: None,
            };

            let cfile = cpp_home.clone() + "/" + CFGFILE;
            let config_path = Path::new(&cfile);
            if config_path.is_file() {
                if let Ok(ini) = Ini::load_from_file(config_path) {
                    let properties = ini.general_section();
                    if let Some(uri) = properties.get("cangrid_uri") {
                        cfg.cangrid_uri = uri.to_string();
                    } else {
                        log::info!("Default canpi grid uri used - {}", cfg.cangrid_uri);
                    }
                }
            } else {
                log::info!("Configuration file '{cfile}' not found");
                log::info!("canpi grid uri = {}", cfg.cangrid_uri);
            }
            if let Ok(port) = std::env::var("HOST_PORT") {
                cfg.host_port = Some(port);
            } else {
                return Err(CanPiAppError::NotFound(
                    "EV HOST_PORT not valid".to_string(),
                ));
            }
            log::info!("{:#?}", cfg);
            Ok(cfg)
        } else {
            Err(CanPiAppError::NotFound(
                "EV CANPIWS_HOME not defined".to_string(),
            ))
        }
    }
}
