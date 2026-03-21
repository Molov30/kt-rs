pub struct Config {
    pub config_path: &'static str,
    pub dp_path: &'static str,
    pub service_name: &'static str,
    pub logs_path: &'static str,
    pub client_ps_name: &'static str,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            config_path: "/opt/AxxonSoft/AxxonNext/instance.conf",
            dp_path: "/opt/AxxonSoft/Ipint.DriverPack/3.0.0",
            service_name: "axxon-next",
            logs_path: "/opt/AxxonSoft/AxxonNext/Logs",
            client_ps_name: "UILauncher",
        }
    }
}

pub struct AppState {
    pub cfg: Config,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            cfg: Config::default(),
        }
    }
}
