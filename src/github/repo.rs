use git2::{Repository, Signature};
use octocrab::Octocrab;
use std::fs;
use std::path::Path;

pub struct GitHubRepo {
    octocrab: Octocrab,
}

impl GitHubRepo {
    pub fn new(token: &str) -> Self {
        let octocrab = Octocrab::builder()
            .personal_token(token.to_string())
            .build()
            .expect("Failed to create Octocrab instance");

        Self { octocrab }
    }

    pub async fn create_repository(
        &self,
        name: &str,
        description: &str,
        private: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let repo = self
            .octocrab
            .repos()
            .create(name)
            .description(description)
            .private(private)
            .send()
            .await?;

        Ok(repo.html_url.unwrap_or_default())
    }

    pub fn initialize_and_push(
        &self,
        local_path: &Path,
        repo_url: &str,
        author_name: &str,
        author_email: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let repo = Repository::init(local_path)?;

        // Add all files
        let mut index = repo.index()?;
        index.add_path(Path::new("."))?;
        index.write()?;

        // Create initial commit
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        let signature = Signature::now(author_name, author_email)?;
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initial commit",
            &tree,
            &[],
        )?;

        // Add remote and push
        let mut remote = repo.remote("origin", repo_url)?;
        remote.push(&["refs/heads/main:refs/heads/main"], None)?;

        Ok(())
    }
}
