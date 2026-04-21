use std::fs;
use std::io;
use std::path::PathBuf;

use super::Conf;
use super::conf_service::ConfService;
use super::static_conf_service::StaticConfService;

pub struct FileConfService {
    config_path: PathBuf,
    fallback: StaticConfService,
    loaded_config: Option<Conf>,
}

impl FileConfService {
    pub fn new() -> Self {
        let home = std::env::var_os("HOME").unwrap_or(std::ffi::OsString::from("."));
        let config_path = PathBuf::from(home)
            .join(".config")
            .join("asus-touchpad-gesture.toml");
        FileConfService {
            config_path,
            fallback: StaticConfService::new(),
            loaded_config: None,
        }
    }

    pub fn load_file(&mut self) -> Result<(), io::Error> {
        let content = match fs::read_to_string(&self.config_path) {
            Ok(c) => c,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                self.loaded_config = None;
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        };
        let conf = toml::from_str::<Conf>(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        self.loaded_config = Some(conf);
        Ok(())
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub fn with_path(path: PathBuf) -> Self {
        FileConfService {
            config_path: path,
            fallback: StaticConfService::new(),
            loaded_config: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ConfService, FileConfService};
    use std::fs;
    use std::io;
    use tempfile::TempDir;

    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;

    fn temp_conf_service() -> (TempDir, FileConfService) {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");
        let service = FileConfService::with_path(config_path);
        (temp_dir, service)
    }

    #[test]
    fn test_missing_file_falls_back_to_static() {
        let (_temp_dir, service) = temp_conf_service();
        let result = service.get_conf();
        assert!(result.is_ok());
        let conf = result.unwrap();
        assert_eq!(conf.left_edge_threshold_percent, 0.1);
        assert_eq!(conf.sensitivity, 0.5);
    }

    #[test]
    fn test_missing_file_load_returns_ok() {
        let (_temp_dir, mut service) = temp_conf_service();

        let result = service.load_file();

        assert!(result.is_ok());
        assert!(service.loaded_config.is_none());
        let conf = service.get_conf().unwrap();
        assert_eq!(conf.left_edge_threshold_percent, 0.1);
    }

    #[test]
    fn test_valid_file_read() {
        let (temp_dir, mut service) = temp_conf_service();
        let toml_content = r#"
left_edge_threshold_percent = 0.2
right_edge_threshold_percent = 0.8
top_edge_threshold_percent = 0.15
sensitivity = 0.7
invert_y = true
volume_step = 0.1
brightness_step = 0.15
seek_step_microseconds = 5000000
"#;
        fs::write(&service.config_path, toml_content).unwrap();

        service.load_file().unwrap();
        let result = service.get_conf();
        drop(temp_dir);
        assert!(result.is_ok());
        let conf = result.unwrap();
        assert_eq!(conf.left_edge_threshold_percent, 0.2);
        assert_eq!(conf.right_edge_threshold_percent, 0.8);
        assert_eq!(conf.top_edge_threshold_percent, 0.15);
        assert_eq!(conf.sensitivity, 0.7);
        assert!(conf.invert_y);
        assert_eq!(conf.volume_step, 0.1);
        assert_eq!(conf.brightness_step, 0.15);
        assert_eq!(conf.seek_step_microseconds, 5_000_000);
    }

    #[test]
    fn test_invalid_toml_returns_error() {
        let (temp_dir, mut service) = temp_conf_service();
        fs::write(&service.config_path, "invalid toml content {").unwrap();

        let result = service.load_file();
        drop(temp_dir);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn test_unreadable_file_returns_error() {
        let (temp_dir, _service) = temp_conf_service();
        let config_path = temp_dir.path().join("unreadable_config.toml");
        let mut unreadable_service = FileConfService::with_path(config_path);

        let conf = super::super::Conf {
            left_edge_threshold_percent: 0.15,
            right_edge_threshold_percent: 0.85,
            top_edge_threshold_percent: 0.12,
            sensitivity: 0.6,
            invert_y: true,
            volume_step: 0.08,
            brightness_step: 0.12,
            seek_step_microseconds: 7_000_000,
        };
        unreadable_service.loaded_config = Some(conf.clone());
        unreadable_service.save_conf().unwrap();

        #[cfg(unix)]
        {
            let path = &unreadable_service.config_path;
            let mut perms = fs::metadata(path).unwrap().permissions();
            perms.set_mode(0o000);
            fs::set_permissions(path, perms).unwrap();

            let result = unreadable_service.load_file();
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert_eq!(err.kind(), io::ErrorKind::PermissionDenied);
        }
    }

    #[test]
    fn test_save_and_read_conf() {
        let (temp_dir, mut service) = temp_conf_service();
        let conf = super::super::Conf {
            left_edge_threshold_percent: 0.15,
            right_edge_threshold_percent: 0.85,
            top_edge_threshold_percent: 0.12,
            sensitivity: 0.6,
            invert_y: true,
            volume_step: 0.08,
            brightness_step: 0.12,
            seek_step_microseconds: 7_000_000,
        };
        service.loaded_config = Some(conf.clone());
        service.save_conf().unwrap();

        let cached = service.get_conf().unwrap();
        assert_eq!(cached.left_edge_threshold_percent, 0.15);
        assert_eq!(cached.sensitivity, 0.6);

        service.load_file().unwrap();
        let result = service.get_conf();
        drop(temp_dir);
        assert!(result.is_ok());
        let loaded = result.unwrap();
        assert_eq!(loaded.left_edge_threshold_percent, 0.15);
        assert_eq!(loaded.sensitivity, 0.6);
    }
}

impl ConfService for FileConfService {
    fn new() -> Self {
        Self::new()
    }

    fn get_conf(&self) -> Result<&Conf, io::Error> {
        match self.loaded_config {
            Some(ref conf) => Ok(conf),
            None => self.fallback.get_conf(),
        }
    }

    fn save_conf(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let conf = self
            .loaded_config
            .as_ref()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "missing loaded config"))?;
        let content = toml::to_string(conf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(fs::write(&self.config_path, content)?)
    }
}
