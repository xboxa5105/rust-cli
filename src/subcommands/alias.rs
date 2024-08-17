use crate::utils::TomlConfig;
use clap::Subcommand;
#[derive(Clone, Subcommand)]
pub enum Alias {
    #[clap(about = "Add alias")]
    Add {
        #[clap(short, long)]
        alias: String,
        #[clap(short, long)]
        command: String,
        #[clap(short, long, required = false)]
        group: Option<String>,
    },
    #[clap(about = "Remove alias")]
    Remove {
        #[clap(short, long)]
        alias: String,
        #[clap(short, long, required = false)]
        group: Option<String>,
    },
    #[clap(about = "List aliases")]
    List {
        #[clap(short, long, required = false)]
        group: Option<String>,
    },
    #[clap(about = "Show alias")]
    Show {
        #[clap(short, long)]
        alias: String,
        #[clap(short, long, required = false)]
        group: Option<String>,
    },
    #[clap(about = "Execute alias")]
    Exec {
        #[clap(short, long)]
        alias: String,
        #[clap(short, long, required = false)]
        group: Option<String>,
    },
}

pub struct AliasCommand {
    pub subcommand: Alias,
    pub toml_config: TomlConfig,
}

impl AliasCommand {
    pub fn new(subcommand: Alias, toml_config: TomlConfig) -> Self {
        AliasCommand {
            subcommand,
            toml_config,
        }
    }
    pub fn command_factory(&mut self) {
        let subcommand = self.subcommand.clone();
        match &subcommand {
            Alias::Add {
                alias,
                command,
                group,
            } => {
                self.add(alias.as_str(), command.as_str(), group.as_deref());
            }
            Alias::Remove { alias, group } => {
                self.remove(alias.as_str(), group.as_deref());
            }
            Alias::List { group } => {
                self.list(group.as_deref());
            }
            Alias::Show { alias, group } => {
                self.show(alias.as_str(), group.as_deref());
            }
            Alias::Exec { alias, group } => {
                self.execute(alias.as_str(), group.as_deref());
            }
        };
    }

    pub fn run(&mut self) -> () {
        self.command_factory()
    }

    fn add(&mut self, alias: &str, command: &str, group_name: Option<&str>) -> () {
        self.toml_config.add(alias, command, group_name);
    }

    fn remove(&mut self, alias: &str, group_name: Option<&str>) -> () {
        self.toml_config.remove(alias, group_name);
    }

    fn list(&mut self, group_name: Option<&str>) -> () {
        self.toml_config.list(group_name);
    }

    fn show(&mut self, alias: &str, group_name: Option<&str>) -> () {
        self.toml_config.show(alias, group_name);
    }

    fn execute(&mut self, alias: &str, group_name: Option<&str>) -> () {
        self.toml_config.execute(alias, group_name);
    }
}

mod test {
    use super::super::super::utils::TomlConfig;
    use super::*;
    use crate::utils::{load_from_file, MockFileReader};
    use mockall::predicate::eq;

    fn mock_toml_config() -> TomlConfig {
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

        load_from_file(&mock, file_path).unwrap()
    }

    #[test]
    fn test_add() {
        let toml_config = mock_toml_config();
        let mut alias_command = AliasCommand::new(
            Alias::Add {
                alias: "test_alias".to_string(),
                command: "test_command".to_string(),
                group: None,
            },
            toml_config,
        );
        alias_command.run();
        assert!(alias_command.toml_config.contains("test_alias", None));
    }

    #[test]
    fn test_add_with_group() {
        let toml_config = mock_toml_config();
        let mut alias_command = AliasCommand::new(
            Alias::Add {
                alias: "test_alias".to_string(),
                command: "test_command".to_string(),
                group: Some("test_group".to_string()),
            },
            toml_config,
        );
        alias_command.run();
        assert!(alias_command
            .toml_config
            .contains("test_alias", Some("test_group")));
    }

    #[test]
    fn test_remove() {
        let toml_config = mock_toml_config();
        let mut alias_command = AliasCommand::new(
            Alias::Add {
                alias: "test_alias".to_string(),
                command: "test_command".to_string(),
                group: None,
            },
            toml_config,
        );
        alias_command.run();
        alias_command.subcommand = Alias::Remove {
            alias: "test_alias".to_string(),
            group: None,
        };
        alias_command.run();
        assert!(!alias_command.toml_config.contains("test_alias", None));
    }

    #[test]
    fn test_remove_with_group() {
        let toml_config = mock_toml_config();
        let mut alias_command = AliasCommand::new(
            Alias::Add {
                alias: "test_alias".to_string(),
                command: "test_command".to_string(),
                group: Some("test_group".to_string()),
            },
            toml_config,
        );
        alias_command.run();
        alias_command.subcommand = Alias::Remove {
            alias: "test_alias".to_string(),
            group: Some("test_group".to_string()),
        };
        alias_command.run();
        alias_command.run();
        assert!(!alias_command
            .toml_config
            .contains("test_alias", Some("test_group")));
    }

    #[test]
    fn test_list() {
        let toml_config = mock_toml_config();
        let mut alias_command = AliasCommand::new(
            Alias::List {
                group: None,
            },
            toml_config,
        );
        alias_command.run();
    }

    #[test]
    fn test_list_with_group() {
        let toml_config = mock_toml_config();
        let mut alias_command = AliasCommand::new(
            Alias::List {
                group: Some("test_group".to_string()),
            },
            toml_config,
        );
        alias_command.run();
    }

    #[test]
    fn test_show() {
        let toml_config = mock_toml_config();
        let mut alias_command = AliasCommand::new(
            Alias::Show {
                alias: "test_alias".to_string(),
                group: None,
            },
            toml_config,
        );
        alias_command.run();
    }

    #[test]
    fn test_show_with_group() {
        let toml_config = mock_toml_config();
        let mut alias_command = AliasCommand::new(
            Alias::Show {
                alias: "test_alias".to_string(),
                group: Some("test_group".to_string()),
            },
            toml_config,
        );
        alias_command.run();
    }

    #[test]
    fn test_execute() {
        let toml_config = mock_toml_config();
        let mut alias_command = AliasCommand::new(
            Alias::Exec {
                alias: "test_alias".to_string(),
                group: None,
            },
            toml_config,
        );
        alias_command.run();
    }

    #[test]
    fn test_execute_with_group() {
        let toml_config = mock_toml_config();
        let mut alias_command = AliasCommand::new(
            Alias::Exec {
                alias: "test_alias".to_string(),
                group: Some("test_group".to_string()),
            },
            toml_config,
        );
        alias_command.run();
    }
}
