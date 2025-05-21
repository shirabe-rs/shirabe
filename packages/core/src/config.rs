use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BasicConfig {
    pub log_level: String,
    pub ignore_self_message: bool,
    pub prefix: Vec<String>,
}

#[cfg(test)]
mod test {
    use crate::config::BasicConfig;
    use serde::Deserialize;

    use config::{Config, File, FileFormat};

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    struct AppConfig {
        basic: BasicConfig,
    }

    #[test]
    fn test_config_load() -> Result<(), Box<dyn std::error::Error>> {
        let config = Config::builder()
            .add_source(File::new("config", FileFormat::Toml)) // 从 "config.toml" 文件加载
            .build()?
            .try_deserialize::<AppConfig>(); // 尝试反序列化为 AppConfig 结构体
        println!("{:?}", config);
        assert!(config.is_ok());

        Ok(())
    }
}
