# Emoji Support

Emoji allows you to use common `:smile:` :smile: emoji markup in your Markdown files.
This implementation has been inspired from the [markdown-it/markdown-it-emoji](https://github.com/markdown-it/markdown-it-emoji) library on NPM.

## Examples

- `:warning:` :warning:
- `:white_check_mark:` :white_check_mark:
- `:negative_squared_cross_mark:` :negative_squared_cross_mark:
- `:information_source:` :information_source:
- `:registered:` :registered:
- `:copyright:` :copyright:
- `:+1:` :+1:

## Emoji Sets and Config

In your `book.toml`, you will need to specify the emoji_sets you wish to use.
```
[output.emojis]
emoji_sets = [full, light]
```

There are 2 emoji sets to choose from.
1. full `emoji_sets = [full]`
1. or light `emoji_sets = [light]`

To mix and match them, you can use 
1. full `emoji_sets = [full, light]`

Precendence order will be right to left. For example `[full, light]`; all `light` emojis will overwrite the `full` emoji set, where there is a clash on name.