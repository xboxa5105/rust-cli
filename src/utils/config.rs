use crate::utils::{run_command, FileReader};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Debug)]
struct AliasConfig {
    general: HashMap<String, String>,
    group: Option<HashMap<String, HashMap<String, String>>>,
}

pub fn load_from_file(
    file_reader: &dyn FileReader,
    file_path: String,
) -> Result<TomlConfig, Box<dyn std::error::Error>> {
    let content = file_reader.read_to_string(&file_path)?;
    let toml_config: TomlConfig = toml::from_str(&content)?;
    Ok(toml_config)
}

pub fn save_to_file(
    file_reader: &dyn FileReader,
    file_path: String,
    toml_config: &TomlConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = toml::to_string(toml_config)?;
    file_reader.write(&file_path, &content)?;
    Ok(())
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TomlConfig {
    alias: AliasConfig,
}

#[cfg(test)]
impl TomlConfig {
    pub(crate) fn contains(&mut self, alias: &str, group_name: Option<&str>) -> bool {
        match self.get_group(group_name) {
            Some(alias_map) => {
                if alias_map.contains_key(alias) {
                    true
                } else {
                    false
                }
            }
            None => false
        }
    }
}

impl TomlConfig {
    fn get_group(&mut self, group_name: Option<&str>) -> Option<&mut HashMap<String, String>> {
        match group_name {
            Some(group) => {
                if let Some(group_map) = self.alias.group.as_mut() {
                    group_map.get_mut(group)
                } else {
                    None
                }
            }
            None => Some(&mut self.alias.general),
        }
    }

    pub fn add(&mut self, alias: &str, command: &str, group_name: Option<&str>) {
        let alias_map = match group_name {
            Some(group) => {
                let group_map = self.alias.group.get_or_insert_with(HashMap::new);
                group_map
                    .entry(group.to_string())
                    .or_insert_with(HashMap::new)
            }
            None => &mut self.alias.general,
        };

        if alias_map.contains_key(alias) {
            alias_map.get_mut(alias).map(|c| *c = command.to_string());
        } else {
            alias_map.insert(alias.to_string(), command.to_string());
        }
    }

    pub fn remove(&mut self, alias: &str, group_name: Option<&str>) {
        match self.get_group(group_name) {
            Some(alias_map) => {
                if alias_map.contains_key(alias) {
                    alias_map.remove(alias);
                } else {
                    println!("Alias not found");
                }
            }
            None => println!("Group not found"),
        }
    }

    pub fn list(&mut self, group_name: Option<&str>) {
        match self.get_group(group_name) {
            Some(alias_map) => {
                if alias_map.is_empty() {
                    println!("No aliases found");
                } else {
                    for (alias, command) in alias_map.iter() {
                        println!("{}: {}", alias, command);
                    }
                }
            }
            None => println!("Group not found"),
        }
    }

    pub fn show(&mut self, alias: &str, group_name: Option<&str>) {
        match self.get_group(group_name) {
            Some(alias_map) => {
                if alias_map.contains_key(alias) {
                    println!("{}: {}", alias, alias_map.get(alias).unwrap());
                } else {
                    println!("Alias not found");
                }
            }
            None => println!("Group not found"),
        }
    }

    pub fn execute(&mut self, alias: &str, group_name: Option<&str>) {
        match self.get_group(group_name) {
            Some(alias_map) => {
                if alias_map.contains_key(alias) {
                    alias_map.get_mut(alias).map(|c| run_command(c));
                } else {
                    println!("Alias not found");
                }
            }
            None => println!("Group not found"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::fs::MockFileReader;
    use super::*;
    use mockall::predicate::eq;

    static MOCK_GROUP_NAME: &str = "aws";

    fn mock_toml_config() -> TomlConfig {
        let mut general = HashMap::new();
        general.insert("ls".to_string(), "ls -l".to_string());
        general.insert("ll".to_string(), "ls -al".to_string());
        let mut aws = HashMap::new();
        let group_name = "aws";
        let mut group: HashMap<String, HashMap<String, String>> = HashMap::new();
        aws.insert("aws_help".to_string(), "aws --help".to_string());
        aws.insert("aws_version".to_string(), "aws --version".to_string());
        group.insert(group_name.to_string(), aws.clone());
        return TomlConfig {
            alias: AliasConfig {
                general,
                group: Some(group),
            },
        };
    }

    #[test]
    fn test_load_from_file() {
        let content = r#"
            [alias]
            [alias.general]
            "ls" = "ls -l"
        "#;
        let mut mock = MockFileReader::new();
        let file_path = "toml_config.toml".to_string();
        mock.expect_read_to_string()
            .with(eq(file_path.clone()))
            .times(1)
            .returning(|_| Ok(content.to_string()));

        let toml_config = load_from_file(&mock, file_path).unwrap();
        assert_eq!(
            toml_config.alias.general.get("ls"),
            Some(&"ls -l".to_string())
        );
    }

    #[test]
    fn test_save_to_file() {
        let mut mock = MockFileReader::new();
        let file_path = "toml_config.toml".to_string();
        let toml_config = TomlConfig {
            alias: AliasConfig {
                general: HashMap::new(),
                group: None,
            },
        };
        mock.expect_write().times(1).returning(|_, _| Ok(()));
        assert_eq!(
            (),
            save_to_file(&mock, file_path.clone(), &toml_config).unwrap()
        );
    }

    #[test]
    fn test_toml_config_get_group_with_no_group_name() {
        let mut _toml_config = mock_toml_config().clone();
        assert_eq!(
            _toml_config.get_group(None).unwrap(),
            &HashMap::from([
                ("ls".to_string(), "ls -l".to_string()),
                ("ll".to_string(), "ls -al".to_string()),
            ])
        );
    }

    #[test]
    fn test_toml_config_get_group_with_group_name() {
        let group_name = MOCK_GROUP_NAME;
        let mut _toml_config = mock_toml_config().clone();
        assert_eq!(
            _toml_config.get_group(Some(group_name)).unwrap(),
            &HashMap::from([
                ("aws_help".to_string(), "aws --help".to_string()),
                ("aws_version".to_string(), "aws --version".to_string()),
            ])
        );
    }

    #[test]
    fn test_toml_config_add() {
        let mut _toml_config = mock_toml_config().clone();
        _toml_config.add("ls", "ls -l", None);
        assert_eq!(
            _toml_config.alias.general.get("ls"),
            Some(&"ls -l".to_string())
        );
    }

    #[test]
    fn test_toml_config_add_with_group() {
        let mut _toml_config = mock_toml_config().clone();
        let group_name = MOCK_GROUP_NAME;
        _toml_config.add("ls", "ls -l", Some(group_name));
        assert_eq!(
            _toml_config
                .alias
                .group
                .unwrap()
                .get(group_name)
                .unwrap()
                .get("ls"),
            Some(&"ls -l".to_string())
        );
    }

    #[test]
    fn test_toml_config_add_exist_command() {
        let mut _toml_config = mock_toml_config().clone();
        let group_name = MOCK_GROUP_NAME;
        _toml_config.add("ls", "ls -al", Some(group_name));
        assert_eq!(
            _toml_config
                .alias
                .group
                .unwrap()
                .get(group_name)
                .unwrap()
                .get("ls"),
            Some(&"ls -al".to_string())
        );
    }

    #[test]
    fn test_toml_config_list() {
        let mut _toml_config = mock_toml_config().clone();
        _toml_config.list(None);
        assert_eq!(
            _toml_config.alias.general,
            HashMap::from([
                ("ls".to_string(), "ls -l".to_string()),
                ("ll".to_string(), "ls -al".to_string()),
            ])
        );
    }

    #[test]
    fn test_toml_config_list_with_group() {
        let mut _toml_config = mock_toml_config().clone();
        let group_name = MOCK_GROUP_NAME;
        _toml_config.list(Some(group_name));
        assert_eq!(
            _toml_config.alias.group.unwrap().get(group_name).unwrap(),
            &HashMap::from([
                ("aws_help".to_string(), "aws --help".to_string()),
                ("aws_version".to_string(), "aws --version".to_string()),
            ])
        );
    }

    #[test]
    fn test_toml_config_show() {
        let mut _toml_config = mock_toml_config().clone();
        _toml_config.list(None);
        assert_eq!(
            _toml_config.alias.general,
            HashMap::from([
                ("ls".to_string(), "ls -l".to_string()),
                ("ll".to_string(), "ls -al".to_string()),
            ])
        );
    }

    #[test]
    fn test_toml_config_show_with_group() {
        let mut _toml_config = mock_toml_config().clone();
        let group_name = MOCK_GROUP_NAME;
        _toml_config.list(Some(group_name));
        assert_eq!(
            _toml_config.alias.group.unwrap().get(group_name).unwrap(),
            &HashMap::from([
                ("aws_help".to_string(), "aws --help".to_string()),
                ("aws_version".to_string(), "aws --version".to_string()),
            ])
        );
    }

    #[test]
    fn test_toml_config_remove() {
        let mut _toml_config = mock_toml_config().clone();
        _toml_config.remove("ls", None);
        assert_eq!(_toml_config.alias.general.get("ls"), None);
    }

    #[test]
    fn test_toml_config_remove_with_group() {
        let group_name = MOCK_GROUP_NAME;
        let mut _toml_config = mock_toml_config().clone();
        _toml_config.remove("ls", Some(group_name));
        assert_eq!(
            _toml_config
                .alias
                .group
                .unwrap()
                .get(group_name)
                .unwrap()
                .get("ls"),
            None
        );
    }

    #[test]
    fn test_toml_config_execute() {
        let mut _toml_config = mock_toml_config().clone();
        _toml_config.execute("ls", None);
    }

    #[test]
    fn test_toml_config_execute_with_group() {
        let group_name = MOCK_GROUP_NAME;
        let mut _toml_config = mock_toml_config().clone();
        _toml_config.execute("ls", Some(group_name));
    }
}
