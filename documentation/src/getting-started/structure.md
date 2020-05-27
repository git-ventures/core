# Repository Structure

## Monolithic vs. Micro

Each `git venture` is self-contained within its own git repository, and it does not track git sub-modules or external repositories.

An organization with several repositories may wish to consolidate their repositories into a `monolithic` or `mono` repository, which contains all source code relevant to the project, e.g. libraries, services, deployment scripts, documentation, etc.

> <i>It is possible to merge git histories from multiple repositories, without losing the history. See this <a href="https://medium.com/altcampus/how-to-merge-two-or-multiple-git-repositories-into-one-9f8a5209913f">article</a> for more details.</i>

The benefit for using a `mono` repo structure for `git ventures` is to capture all commits within a codebase, simplifying the problem of needing to aggregate data from multiple repositories. 

It is up to the organization how it chooses to structure the repository. An organization may choose to initialize more than one `git venture`, but must manage each repository separately. 

There may be a benefit to doing this if the organization wishes to encapsulate a specific high-risk R&D project on its own, outside of its main repository, in the event the project fails and becomes distressed.



