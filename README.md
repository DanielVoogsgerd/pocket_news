Pocket News
===========

Sync various news sources to pocket.

## Why?

Syncing news sources to pocket allows them to be read easily on a Kobo ereader.

## Stability

As of this moment the project in a very early alpha. Parts will be broken and APIs will be changed. Use at your own risk.

## Configuration

A config file must be placed in `$XDG_CONFIG_HOME/pocket_news/config.toml`. Its contents must be:

```toml
consumer_key = "CONSUMER KEY"
```

A consumer key can be created on the [pocket developer website](https://getpocket.com/developer/apps/new)

During the first run an access key is requested via OAuth which is stored in `$XDG_DATA_HOME/pocket_news/datafile.toml`.
