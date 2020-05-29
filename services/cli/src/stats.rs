use super::contributor::*;

use anyhow::Error;
use git2::{Commit, Oid, Delta, Diff, DiffBinary, DiffDelta, DiffHunk, DiffLine, Repository};
use log::debug;
use regex::Regex;
use std::collections::HashMap;
use std::str::from_utf8;
use whatlang::{detect, Lang, Script};

type Email = String;

#[derive(Debug, Default)]
pub struct CommitStats {
    email: Email,
    total_size: usize
}

impl CommitStats {
    pub fn new(email: Option<Email>) -> Self {
        Self {
            email: email.unwrap_or(String::new()),
            ..Default::default()
        }
    }
}



#[derive(Debug, Default)]
pub struct Stats {
    pub contributors: HashMap<Email, Contributor>,
    pub commits: HashMap<Oid, CommitStats>,
    pub total_size: usize
}

impl Stats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process_diff(&mut self, repo: &Repository, commit: &Commit<'_>, diff: Diff) -> Result<(), Error> {
        let Contributor { email, .. } = commit.clone().try_into()?;
        let mut cs = CommitStats::new(email);
        let stats = diff.stats()?;
        let files_changed = stats.files_changed();
        let insertions = stats.insertions();
        let deletions = stats.deletions();
        
        debug!("Files Changed: {:?}", files_changed);
        debug!("Insertions: {:?}", insertions);
        debug!("Deletions: {:?}", deletions);

        for delta in diff.deltas() {
            match delta.status() {
                Delta::Unmodified => {
                    debug!("Delta Status: {:?}", Delta::Unmodified);
                    // unimplemented!();
                },
                Delta::Added => {
                    let new_blob = repo.find_blob(delta.new_file().id())?;
                    debug!("Delta Status: {:?}", Delta::Added);
                    debug!("Size: {:?}", new_blob.size());
                    // Strip the syntax from the source file;
                    let rgx = Regex::new(r"[^a-zA-Z]")?;
                    let content = rgx.replace_all(&from_utf8(new_blob.content())?, " ");

                    // Ignore binary files;
                    if !new_blob.is_binary() {
                        if let Some(lang) = detect(&content) {

                            debug!("Language Detected: {:?}", lang);
                            cs.total_size += new_blob.size();
                            self.total_size += new_blob.size();
                        }
                    }
                    // unimplemented!();
                },
                Delta::Deleted => {
                    debug!("Delta Status: {:?}", Delta::Deleted);
                    // unimplemented!();
                },
                Delta::Modified => {
                    let old_blob = repo.find_blob(delta.old_file().id())?;
                    let new_blob = repo.find_blob(delta.new_file().id())?;

                    let old_size = old_blob.size();
                    let new_size = new_blob.size();

                    let rgx = Regex::new(r"[^a-zA-Z]")?;
                    let new_content = rgx.replace_all(&from_utf8(new_blob.content())?, " ");

                    // Ignore binary files;
                    if !new_blob.is_binary() {
                        if let Some(lang) = detect(&new_content) {
                            
                            debug!("Language Detected: {:?}", lang);
                            if new_size > old_size {
                                cs.total_size += new_size - old_size;
                                self.total_size += new_size - old_size;
                            }
                        }
                    }
                },
                Delta::Renamed => {
                    debug!("Delta Status: {:?}", Delta::Renamed);
                    // unimplemented!();
                },
                Delta::Copied => {
                    debug!("Delta Status: {:?}", Delta::Copied);
                    // unimplemented!();
                },
                Delta::Ignored => {
                    debug!("Delta Status: {:?}", Delta::Ignored);
                    // unimplemented!();
                },
                Delta::Untracked => {
                    debug!("Delta Status: {:?}", Delta::Untracked);
                    // unimplemented!();
                },
                Delta::Typechange => {
                    debug!("Delta Status: {:?}", Delta::Typechange);
                    // unimplemented!();
                },
                Delta::Unreadable => {
                    debug!("Delta Status: {:?}", Delta::Unreadable);
                    // unimplemented!();
                },
                Delta::Conflicted => {
                    debug!("Delta Status: {:?}", Delta::Conflicted);
                    // unimplemented!();
                },
            }
        }

        let commit_id = commit.id();
        self.commits.insert(commit_id, cs);
        // unimplemented!();
        Ok(())
    }

    pub fn process_commit(&mut self, repo: &Repository, commit: Commit<'_>) -> Result<(), Error> {
        // Insert contributor into contributors
        self.insert_contributor_from_commit(&commit)?;
        
        // Compare the parent tree to the current tree;
        let tree = commit.tree()?;

        if commit.parent_count() > 0 {
            for parent in commit.parents() {
                let pt = parent.tree()?;
                let diff = repo.diff_tree_to_tree(Some(&pt), Some(&tree), None)?;
                self.process_diff(repo, &parent, diff)?;
            }
        } else {
            let diff = repo.diff_tree_to_tree(None, Some(&tree), None)?;
            self.process_diff(repo, &commit, diff)?;
        }

        Ok(())
    }

    pub fn insert_contributor_from_commit(
        &mut self,
        commit: &Commit<'_>,
    ) -> Result<Contributor, Error> {
        let contributor: Contributor = commit.clone().try_into()?;
        if let Some(email) = &contributor.email {
            // Insert a contributor based on `email`;
            self.contributors
                .insert(email.to_owned(), contributor.clone());
        } else if let Some(_name) = &contributor.name {
            // TODO: Search all known contributors for a matching name;
            // Possibly use fuzzy matching algorithm for determining potential
            // authors
            debug!("Found Contributor Name!");
        } else {
            // Unable to assign attribute for this commit;
            debug!("No Commit Contributor Found!");
        }

        Ok(contributor)
    }

    pub fn get_contributor_by_email(&self, email: &str) -> Option<&Contributor> {
        self.contributors.get(email)
    }

    pub fn get_contributor_score(&self, email: &str) -> f64 {
        self.contributors.get(email).map(|c| c.score).unwrap_or(0.0)
    }

    pub fn update_contributor_score(&mut self, email: &str, points: f64) -> f64 {
        self.contributors
            .get_mut(email)
            .map(|c| c.increase_score(points))
            .unwrap_or(0.0)
    }

    pub fn calculate_rewards(&mut self) -> Result<(), Error> {
        for oid in self.commits.keys() {
            let weight = self.get_commit_weight(&oid);
            debug!("Weight {:?}", weight);
            if let Some(CommitStats { email, total_size }) = self.commits.get(oid) {
                if let Some(Contributor { score, .. }) = self.contributors.get_mut(email) {
                    let nscore = (*total_size as f64) * weight;
                    debug!("New Score: {:?}", nscore);
                    *score += nscore;
                }
            }
        }
        
        Ok(())
    }

    pub fn get_commit_weight(&self, oid: &Oid) -> f64 {
        if let Some(cs) = self.commits.get(oid) {
            (cs.total_size as f64) / (self.total_size as f64)
        } else {
            0.0
        }
    }
}
