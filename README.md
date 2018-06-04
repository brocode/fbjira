# fbjira

[![Crates](https://img.shields.io/crates/v/fbjira.svg)](https://crates.io/crates/fbjira)

[WIP] get more stuff working...

* Config lives @ `~/.fbjira.toml`:

  ```
  jira_domain = "https://<your-account>.atlassian.net"
  jira_user = "<your-jira-user>"
  jira_token = "<your-jira-token>"
  ```

* `fbjira issue list-open <PROJECT>` to list all open issues for `<PROJECT>`
* `fbjira issue summary <ISSUE>` to get summary for `<ISSUE>`

## Useful alias

```bash
alias jira="fbjira issue list-open <project-of-choice> | fzf --preview \"echo {} | cut -d' ' -f1 | xargs fbjira issue summary\" | cut -d ' ' -f1 | xargs -I {} xdg-open 'https://<your-account>.atlassian.net/browse/{}'"
```
