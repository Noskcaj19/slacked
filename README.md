# Sled

A Slack client designed with an `ed`-like interface

# Usage

1. Get a Slack legacy token [here](https://api.slack.com/docs/oauth-test-tokens)
2. Create a `~/.config/sled/config.toml` file and add a token value:

```toml
token = "xoxp-....."
```

# Tips

## Identifiers

* `@` - Users
    * Muliple users seperated by a comma indicates a mpim or usergroup
* `#` - Channel
