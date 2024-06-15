use octocrab::{models::Repository, Octocrab, Page};
use chrono::{Datelike, Utc};

pub struct Project
{
    pub url: String,
    pub name: String,
    pub description: String,
    pub stars: u32,
    pub forks: u32,
    pub issues: u32,
    pub commits: u32,
    pub pull_requests: u32,
    pub last_commit: String,
    pub new: bool,
}

impl Project
{
    pub fn _new(url: &str) -> Self
    {
        Self {
            url: url.to_string(),
            name: "".to_string(),
            description: "".to_string(),
            stars: 0,
            forks: 0,
            issues: 0,
            commits: 0,
            pull_requests: 0,
            last_commit: "".to_string(),
            new: false,
        }
    }

    fn formatted_date(date: chrono::DateTime<Utc>) -> String
    {
        format!("{:02}-{:02}-{:04}", date.day(), date.month(), date.year())
    }

    fn is_repo_new(date: chrono::DateTime<Utc>) -> bool
    {
        let now = Utc::now();
        let days = now.signed_duration_since(date).num_days();
        days <= 30
    }

    fn repo_url_format(
        owner: &str,
        name: &str,
    ) -> String
    {
        format!("https://github.com/{}/{}", owner, name)
    }

    pub async fn get_repositories(
        octo: Octocrab
    ) -> octocrab::Result<Vec<Project>>
    {
        let repos = Project::fetch_user_repositories(octo).await?;
        let projects = Project::process_repositories(repos);
        Ok(projects)
    }

    pub async fn fetch_user_repositories(
        octo: Octocrab
    ) -> octocrab::Result<Page<Repository>>
    {
        octo.current()
            .list_repos_for_authenticated_user()
            .type_("owner")
            .sort("updated")
            .per_page(100)
            .send()
            .await
    }

    pub fn process_repositories(repos: Page<Repository>) -> Vec<Project>
    {
        repos
            .into_iter()
            .map(|repo| {
                let last_commit = repo.updated_at.unwrap_or_default();
                let last_commit = Project::formatted_date(last_commit);
                let new =
                    Project::is_repo_new(repo.created_at.unwrap_or_default());
                let url = Project::repo_url_format(
                    &repo.owner.unwrap().login,
                    &repo.name,
                );
                Project {
                    url,
                    name: repo.name,
                    description: repo.description.unwrap_or("".to_string()),
                    stars: repo.stargazers_count.unwrap_or_default(),
                    forks: repo.forks_count.unwrap_or_default(),
                    issues: repo.open_issues_count.unwrap_or_default(),
                    commits: 0,
                    pull_requests: 0,
                    last_commit,
                    new,
                }
            })
            .collect()
    }

    pub async fn get_repositories_liked(
        octo: Octocrab
    ) -> octocrab::Result<Vec<Project>>
    {
        let repos = Project::fetch_user_repositories(octo).await?;
        let projects = Project::process_repositories(repos);
        Ok(projects)
    }
}
