use inquire::{InquireError, MultiSelect, Select, Text};

#[derive(Debug, Default)]
pub struct CreateAppInput {
    pub name: String,
    pub template: String,
    pub init_git: bool,
    pub extra_libs: Vec<String>,
}

impl CreateAppInput {
    pub fn has_extra_libs(&self) -> bool {
        !self.extra_libs.is_empty()
    }

    pub fn collect() -> CreateAppInput {
        let mut config = CreateAppInput::default();
        let project_name = Text::new("What is your project name?")
            .with_placeholder("my-sword-app")
            .prompt();

        let Ok(project_name) = &project_name else {
            println!("❌Failed to read project name. Aborting...");
            std::process::exit(1);
        };

        config.name = project_name.to_owned();

        println!();

        let options: Vec<&str> = vec![
            "None",
            "Basic Application",
            "Todo App (json storage)",
            "Database Integration (sqlx postgres)",
            "Mail Microservice (lettre)",
            "Dependency Injection (shaku)",
            "Authentication (jsonwebtoken)",
        ];

        let ans: Result<&str, InquireError> =
            Select::new("Want to start with a template?", options).prompt();

        config.template = match &ans {
            Ok(answer) => answer.to_string(),
            Err(_) => "None".to_string(),
        };

        println!();

        let git_init =
            Select::new("Initialize a git repository?", vec!["Yes", "No"]).prompt();

        config.init_git = match &git_init {
            Ok(answer) => answer == &"Yes",
            Err(_) => false,
        };

        println!();

        let libs: Vec<&str> = vec![
            "lettre (Email)",
            "jsonwebtoken (JWT Authentication)",
            "shaku (Dependency Injection)",
            "tera (Templating)",
            "validator (Input Validation)",
            "bcrypt (Password Hashing)",
            "uuid (Unique IDs)",
            "chrono (Date and Time)",
            "reqwest (HTTP Requests)",
            "redis (Redis Client)",
            "regex (Regular Expressions)",
            "thiserror (Error Handling)",
        ];

        let extra_libs: Result<Vec<&str>, InquireError> =
            MultiSelect::new("Want to include any extra libraries?", libs).prompt();

        config.extra_libs = match &extra_libs {
            Ok(choices) => choices.iter().map(|s| s.to_string()).collect(),
            Err(_) => vec![],
        };

        config
    }
}
