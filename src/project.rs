use octocrab::Octocrab;

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
}

impl Project
{
    pub fn new(
        url: &str,
        name: &str,
        description: &str,
        stars: u32,
        forks: u32,
        issues: u32,
        commits: u32,
        pull_requests: u32,
    ) -> Self
    {
        Self {
            url: url.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            stars,
            forks,
            issues,
            commits,
            pull_requests,
        }
    }

    pub async fn get_repos(
    ) -> octocrab::Result<Vec<Project>>
    {
        let token = std::env::var("GITHUB_TOKEN")
            .expect("GITHUB_TOKEN env variable is required");

        let octocrab = Octocrab::builder().personal_token(token).build()?;
    let my_repos = octocrab
        .current()
        .list_repos_for_authenticated_user()
        .type_("owner")
        .sort("updated")
        .per_page(100)
        .send()
        .await?;

    let mut projects = vec![];
    for repo in my_repos {
        let project = Project::new(
            &repo.url.as_str(),
            &repo.name,
            &repo.description.unwrap_or(format!("No description for {}", repo.name)),
            repo.stargazers_count.unwrap_or(0),
            repo.forks_count.unwrap_or_default(),
            repo.open_issues_count.unwrap_or_default(),
            0,
            0,
        );
        projects.push(project);

    }
        Ok(projects)
    }
}
