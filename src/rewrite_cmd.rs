use crate::discover::registry;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RewriteDecision {
    pub original_command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewritten_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_rule: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_reason: Option<String>,
    pub excluded_by_config: bool,
}

pub fn decide_rewrite(cmd: &str, config: &crate::config::Config) -> RewriteDecision {
    let trimmed = cmd.trim();
    let excluded = &config.hooks.exclude_commands;
    let already_rtk = trimmed.starts_with("rtk ") || trimmed == "rtk";
    let excluded_by_config = trimmed
        .split_whitespace()
        .next()
        .is_some_and(|base| excluded.iter().any(|item| item == base));

    let rewritten_command = registry::rewrite_command(cmd, excluded);
    let matched_rule = if let Some(ref rewritten) = rewritten_command {
        registry::matched_rule_id(trimmed).or_else(|| registry::matched_rule_id(rewritten))
    } else {
        None
    };

    let skip_reason = if rewritten_command.is_some() {
        None
    } else if trimmed.is_empty() {
        Some("empty_command".to_string())
    } else if excluded_by_config {
        Some("excluded_by_config".to_string())
    } else if already_rtk {
        Some("already_rtk".to_string())
    } else if trimmed.contains("<<") || trimmed.contains("$((") {
        Some("unsafe_shell_syntax".to_string())
    } else {
        Some("unsupported_or_passthrough".to_string())
    };

    RewriteDecision {
        original_command: cmd.to_string(),
        rewritten_command,
        matched_rule,
        skip_reason,
        excluded_by_config,
    }
}

pub fn render_decision_json(decision: &RewriteDecision) -> anyhow::Result<String> {
    Ok(serde_json::to_string_pretty(decision)?)
}

/// Run the `rtk rewrite` command.
///
/// Prints the RTK-rewritten command to stdout and exits 0.
/// Exits 1 (without output) if the command has no RTK equivalent.
///
/// Used by shell hooks to rewrite commands transparently:
/// ```bash
/// REWRITTEN=$(rtk rewrite "$CMD") || exit 0
/// [ "$CMD" = "$REWRITTEN" ] && exit 0  # already RTK, skip
/// ```
pub fn run(cmd: &str, json: bool) -> anyhow::Result<()> {
    let config = crate::config::Config::load().unwrap_or_default();
    let decision = decide_rewrite(cmd, &config);

    if json {
        print!("{}", render_decision_json(&decision)?);
        if decision.rewritten_command.is_some() {
            return Ok(());
        }
        std::process::exit(1);
    }

    match decision.rewritten_command {
        Some(rewritten) => {
            print!("{}", rewritten);
            Ok(())
        }
        None => {
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, HooksConfig};

    #[test]
    fn test_run_supported_command_succeeds() {
        assert!(registry::rewrite_command("git status", &[]).is_some());
    }

    #[test]
    fn test_run_unsupported_returns_none() {
        assert!(registry::rewrite_command("terraform plan", &[]).is_none());
    }

    #[test]
    fn test_run_already_rtk_returns_some() {
        assert_eq!(
            registry::rewrite_command("rtk git status", &[]),
            Some("rtk git status".into())
        );
    }

    #[test]
    fn test_decide_rewrite_supported_command() {
        let decision = decide_rewrite("git status", &Config::default());

        assert_eq!(decision.original_command, "git status");
        assert_eq!(
            decision.rewritten_command.as_deref(),
            Some("rtk git status")
        );
        assert_eq!(decision.matched_rule.as_deref(), Some("git.status"));
        assert_eq!(decision.skip_reason, None);
        assert!(!decision.excluded_by_config);
    }

    #[test]
    fn test_decide_rewrite_excluded_command() {
        let config = Config {
            hooks: HooksConfig {
                exclude_commands: vec!["gh".into()],
            },
            ..Config::default()
        };

        let decision = decide_rewrite("gh pr list", &config);

        assert_eq!(decision.rewritten_command, None);
        assert_eq!(decision.skip_reason.as_deref(), Some("excluded_by_config"));
        assert!(decision.excluded_by_config);
    }

    #[test]
    fn test_decide_rewrite_compound_command_skip_reason() {
        let decision = decide_rewrite("echo hi | grep hi", &Config::default());

        assert_eq!(decision.rewritten_command, None);
        assert_eq!(
            decision.skip_reason.as_deref(),
            Some("unsupported_or_passthrough")
        );
    }

    #[test]
    fn test_render_decision_json_contains_expected_fields() {
        let decision = RewriteDecision {
            original_command: "git status".into(),
            rewritten_command: Some("rtk git status".into()),
            matched_rule: Some("git.status".into()),
            skip_reason: None,
            excluded_by_config: false,
        };

        let json = render_decision_json(&decision).expect("json should serialize");

        assert!(json.contains("\"original_command\": \"git status\""));
        assert!(json.contains("\"rewritten_command\": \"rtk git status\""));
        assert!(json.contains("\"matched_rule\": \"git.status\""));
    }
}
