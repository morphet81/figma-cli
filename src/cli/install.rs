use clap::Subcommand;
use std::fs;
use std::path::Path;
use colored::Colorize;

const SKILL_MD: &str = include_str!("../../skills/figma/SKILL.md");
const REF_INSPECT: &str = include_str!("../../skills/figma/references/inspect.md");
const REF_EXPORT: &str = include_str!("../../skills/figma/references/export.md");
const REF_DESIGN_SYSTEM: &str = include_str!("../../skills/figma/references/design-system.md");
const REF_COMMENTS: &str = include_str!("../../skills/figma/references/comments.md");
const REF_IMPLEMENT: &str = include_str!("../../skills/figma/references/implement.md");

struct SkillFile {
    relative_path: &'static str,
    content: &'static str,
}

const SKILL_FILES: &[SkillFile] = &[
    SkillFile { relative_path: "figma/SKILL.md", content: SKILL_MD },
    SkillFile { relative_path: "figma/references/inspect.md", content: REF_INSPECT },
    SkillFile { relative_path: "figma/references/export.md", content: REF_EXPORT },
    SkillFile { relative_path: "figma/references/design-system.md", content: REF_DESIGN_SYSTEM },
    SkillFile { relative_path: "figma/references/comments.md", content: REF_COMMENTS },
    SkillFile { relative_path: "figma/references/implement.md", content: REF_IMPLEMENT },
];

#[derive(Subcommand, Debug)]
pub enum InstallCommands {
    /// Install the fcli AI skill for Claude and Cursor
    Skill {
        /// Install to current project instead of global user directory
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },
}

fn install_skill_to(base: &Path, tool_name: &str, skills_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let skills_path = base.join(skills_dir);

    for file in SKILL_FILES {
        let dest = skills_path.join(file.relative_path);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&dest, file.content)?;
    }

    println!("  {} Installed to {}", "✓".green(), skills_path.join("figma/").display());

    if tool_name == "Claude" {
        let settings_path = base.join("settings.json");
        update_claude_settings(&settings_path, &format!("{}/figma", skills_dir))?;
    }

    Ok(())
}

fn update_claude_settings(settings_path: &Path, skill_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut settings: serde_json::Value = if settings_path.exists() {
        let content = fs::read_to_string(settings_path)?;
        serde_json::from_str(&content).unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    let permissions = settings
        .as_object_mut()
        .ok_or("Invalid settings.json")?
        .entry("permissions")
        .or_insert(serde_json::json!({}));

    let allow = permissions
        .as_object_mut()
        .ok_or("Invalid permissions")?
        .entry("allow")
        .or_insert(serde_json::json!([]));

    let allow_arr = allow.as_array_mut().ok_or("Invalid allow array")?;

    let patterns = [
        "Bash(fcli:*)",
        &format!("Read({}/*)", skill_dir),
    ];

    for pattern in &patterns {
        let val = serde_json::Value::String(pattern.to_string());
        if !allow_arr.contains(&val) {
            allow_arr.push(val);
        }
    }

    if let Some(parent) = settings_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(settings_path, serde_json::to_string_pretty(&settings)?)?;

    Ok(())
}

pub async fn run(command: InstallCommands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        InstallCommands::Skill { scope } => {
            let is_project = scope.as_deref() == Some("project");

            println!("{}", "Installing fcli skill...".bold());
            println!();

            if is_project {
                let cwd = std::env::current_dir()?;
                println!("Scope: {} ({})", "project".cyan(), cwd.display());
                println!();

                let claude_base = cwd.join(".claude");
                let cursor_base = cwd.join(".cursor");

                install_skill_to(&claude_base, "Claude", "skills")?;
                install_skill_to(&cursor_base, "Cursor", "skills")?;
            } else {
                let home = dirs::home_dir().ok_or("Cannot determine home directory")?;
                println!("Scope: {} (~/.claude, ~/.cursor)", "global".cyan());
                println!();

                let claude_base = home.join(".claude");
                let cursor_base = home.join(".cursor");

                install_skill_to(&claude_base, "Claude", "skills")?;
                install_skill_to(&cursor_base, "Cursor", "skills")?;
            }

            println!();
            println!("{}", "Skill installed successfully!".green().bold());
            println!();
            println!("Usage:");
            println!("  Claude Code:  {}", "/figma <Figma URL>".cyan());
            println!("  Cursor:       reference the skill in your prompt or agent settings");
            println!();
            println!("The skill teaches AI agents how to use fcli to inspect designs,");
            println!("export assets, list components/styles, manage comments, and more.");

            Ok(())
        }
    }
}
