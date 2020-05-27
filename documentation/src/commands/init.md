# `> git ventures init`

The `git ventures init` command sets up the `.gitventure/` directory for the repository.

Unlike the `.git/` directory, `.gitventure/` is indexed as part of the git repository.

```toml
<!-- Example Configuration File -->

[information]
organization = "your-organization"
name = "example-project"
open_source = true

[legal]
entity_type = "LLC"
domicile = "US"
state = "WA"
founding_date = "MM/DD/YYYY"

[[people]]
name = "Ryan Tate"
email = "ryan@emergent.financial"

```

