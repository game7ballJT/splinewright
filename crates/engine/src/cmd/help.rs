//! Help → project links (this app's page, its GitHub repository and the upstream VectorCraft project
//! it is a fork of, which the fork credits). The commands return the URL; the frontend opens it.

use serde_json::{Value, json};

use super::*;

/// The app's id on the website and GitHub.
pub const APP_ID: &str = "splinewright";
pub const WEBSITE_URL: &str = "https://www.jonmcgee.online";
/// The project this fork is based on.
pub const UPSTREAM_URL: &str = "https://github.com/storytold/vectorcraft";

/// This app's page on the website.
pub fn app_page_url() -> String {
    format!("{WEBSITE_URL}/apps/{APP_ID}/")
}

/// This app's source repository.
pub fn github_url() -> String {
    format!("https://github.com/game7ballJT/{APP_ID}")
}

/// The documentation folder of the repository (Help › Splinewright Help…).
pub fn docs_url() -> String {
    format!("{}/tree/main/docs", github_url())
}

/// Where to report a problem.
pub fn issues_url() -> String {
    format!("{}/issues", github_url())
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(query "help.website", "Splinewright Website", ["Help"], None, "{} → {url} this app's page", always, |_, _| url(app_page_url())),
        cmd!(query "help.docs", "Splinewright Help…", ["Help"], Some("F1"), "{} → {url} the documentation", always, |_, _| url(docs_url())),
        cmd!(query "help.github", "Splinewright on GitHub", ["Help"], None, "{} → {url} source code and releases", always, |_, _| url(github_url())),
        cmd!(query "help.upstream", "Based on VectorCraft…", ["Help"], None, "{} → {url} the upstream project this fork is based on", always, |_, _| url(UPSTREAM_URL.into())),
        cmd!(query "help.reportIssue", "Report an Issue…", ["Help"], None, "{} → {url} the issue tracker", always, |_, _| url(issues_url())),
        cmd!(query "help.links", "Links", [], None, "{} → {website, github, upstream, issues, docs}", always, |_, _| {
            Ok(json!({ "website": app_page_url(), "github": github_url(), "upstream": UPSTREAM_URL, "issues": issues_url(), "docs": docs_url() }))
        }),
    ]
}

fn url(u: String) -> Result<Value> {
    Ok(json!({ "url": u }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links() {
        let mut s = Session::new();
        assert_eq!(s.execute("help.website", &json!({})).unwrap()["url"], "https://www.jonmcgee.online/apps/splinewright/");
        assert_eq!(s.execute("help.github", &json!({})).unwrap()["url"], "https://github.com/game7ballJT/splinewright");
        assert_eq!(s.execute("help.upstream", &json!({})).unwrap()["url"], "https://github.com/storytold/vectorcraft");
        assert_eq!(s.execute("help.reportIssue", &json!({})).unwrap()["url"], "https://github.com/game7ballJT/splinewright/issues");
        assert_eq!(s.execute("help.docs", &json!({})).unwrap()["url"], "https://github.com/game7ballJT/splinewright/tree/main/docs");
        assert_eq!(s.execute("help.links", &json!({})).unwrap()["upstream"], "https://github.com/storytold/vectorcraft");
    }
}
