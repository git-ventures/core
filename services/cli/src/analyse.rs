use anyhow::Error;
use git2::{Repository, BranchType, Oid};
use std::env;

pub struct Contributor {
    name: &'static str,
    email: &'static str,
    // contributor score,
    // based on LOC, code frequency
    score: u64,
}

pub struct Stats {
    contributors: Vec<Contributor>
}

pub struct Analyse {
    repo: Repository,
    // Struct for calculated statistics
    stats: Option<Stats>
}

impl Analyse {
    pub fn new() -> Result<Analyse, Error> {
        //
        Ok(Analyse {
            repo: Repository::discover(env::current_dir()?)?,
            stats: None
        })
    }

    /// If the commit is `None`, then attempt to find the Oid for the
    /// `HEAD` commit.
    pub fn commit(&mut self, commit: Option<&str>) -> Result<(), Error> {
        let oid = match commit {
            Some(c) => Oid::from_str(c)?,
            // By default, attempt to analyse the head commit;
            None => match self.repo.head()?.target() {
                Some(oid) => oid,
                None => return Err(Error::msg("Failed to find HEAD commit"))
            }
        };

        let commit = self.repo.find_commit(oid)?;
        println!("Found commit {:?}", commit.author().when().seconds());
        let diff = self.repo.diff_tree_to_workdir(Some(&commit.tree()?), None)?;
        for delta in diff.deltas() {
            println!("Delta {:?}", delta.status());
            println!("Files Changes {:?}", delta.nfiles());
            println!("Old File {:?}", delta.old_file().path());
            println!("New File {:?}", delta.new_file().path());
        }
        Ok(())
    }

    /// Analyse a branch
    pub fn branch(&mut self, branch: &str) -> Result<(), Error> {
        println!("Find Branch {:?}", branch);
        let branch = self.repo.find_branch(branch, BranchType::Local)?;



        if branch.is_head() {
            println!("Found HEAD Branch");
        }

        Ok(())
        // unimplemented!()
    }
}
