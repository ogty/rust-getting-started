# Zenn Article Initializer

## Usage

```bash
$ cargo build
```

Place `target/debug/zai.exe` directly under the folder where you keep your Zenn articles, and run the following command.

```bash
$ zai <title> <topics>
```

The following three patterns can be executed.

- Nothing
- Title only
- With topics

```bash
$ zai title one two three
```

If you run the above command, you will get the following results.

**`xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx.md`**

```md
---
title: title
emoji: 🐒
type: tech
topics: [one, two, three]
published: false
---
```
