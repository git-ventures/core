use super::stats::*;

use anyhow::Error;
use git2::{Oid, Repository, Sort};
use log::debug;
use std::env;

pub struct Analyze {
    repo: Repository,
    // Struct for calculated statistics
    stats: Stats,
}

impl Analyze {
    pub fn new() -> Result<Analyze, Error> {
        //
        Ok(Analyze {
            repo: Repository::discover(env::current_dir()?)?,
            stats: Stats::new(),
        })
    }

    /// If the commit is `None`, then attempt to find the Oid for the
    /// `HEAD` commit.
    pub fn commit(&mut self, oid: Option<&str>) -> Result<(), Error> {
        let oid = match oid {
            Some(c) => Oid::from_str(c)?,
            // By default, attempt to Analyze the head commit;
            None => match self.repo.head()?.target() {
                Some(oid) => oid,
                None => return Err(Error::msg("Failed to find HEAD commit")),
            },
        };

        let mut walker = self.repo.revwalk()?;

        // Reverse the order of parsing;
        // This will Analyze the commit history from its first commit
        // to the target commit;
        walker.set_sorting(Sort::REVERSE)?;

        // Push the commit in question as the parent of the
        // ancestor commits to include in the analysis
        walker.push(oid)?;

        for obj in walker {
            if let Ok(oid) = obj {
                let commit = self.repo.find_commit(oid)?;
                self.stats.process_commit(&self.repo, commit)?;
            }
        }

        // Process all commits;
        self.stats.calculate_rewards()?;

        for email in self.stats.contributors.keys() {
            debug!(
                "Found Contributor: {:?}",
                self.stats.contributors.get(email)
            );
        }

        Ok(())
    }
}

// println!("Found commit {:?}", commit.author().when().seconds());
// let tree = &commit.tree()?;

// tree.walk(TreeWalkMode::PreOrder, |_, entry: &TreeEntry| {
//     if let Ok(obj) = entry.to_object(&self.repo) {
//         println!("Found Entry in Tree Walk {:?}", obj.as_commit());
//         // match entry.kind() {
//         //     Some(ObjectType::Any) => {},
//         //     Some(ObjectType::Commit) => {},
//         //     Some(ObjectType::Tree) => {
//         //         if let Ok(tree) = obj.peel_to_tree() {
//         //             println!("Found Tree Entry Tree {:?}", tree.id())
//         //         }
//         //     },
//         //     Some(ObjectType::Blob) => {
//         //         if let Ok(blob) = obj.peel_to_blob() {
//         //             println!("Found Tree Entry Blob {:?}", blob.id())
//         //         }
//         //     },
//         //     Some(ObjectType::Tag) => {},
//         //     None => {}
//         // };
//     }

//     TreeWalkResult::Ok
// })?;

// Review Diff
// let diff = self.repo.diff_tree_to_workdir(Some(tree), None)?;
// for delta in diff.deltas() {
//     println!("Delta {:?}", delta.status());
//     println!("Files Changes {:?}", delta.nfiles());
//     println!("Old File {:?}", delta.old_file().path());
//     println!("New File {:?}", delta.new_file().path());
// }
