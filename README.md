# ShareBin

![Build](https://github.com/timshel/sharebin/actions/workflows/rust.yml/badge.svg)
[![Docker Pulls](https://img.shields.io/docker/pulls/timshel/sharebin?label=Docker%20pulls)](https://img.shields.io/docker/pulls/timshel/sharebin?label=Docker%20pulls)

ShareBin is a super tiny, feature rich, configurable, self-contained and self-hosted paste bin web application. It is very easy to set up and use, and will only require a few megabytes of memory and disk storage. It takes only a couple minutes to set it up, why not give it a try now?

## Forked

This a Fork of [MicroBin](https://github.com/szabodanika/microbin) made by Dániel Szabó.

Forked the project to update dependencies, make minor fixes and gauge interest in continuing it.
I have **not** reviewed the cryptography used.

If you need to share truly sensitive information I would recommand using a more established solution (Ex: https://privatebin.info/).

## Features

- Entirely self-contained executable, ShareBin is a single file!
- Server-side and client-side encryption
- File uploads (eg. `server.com/file/pig-dog-cat`)
- Raw text serving (eg. `server.com/raw/pig-dog-cat`)
- QR code support
- URL shortening and redirection
- Animal names instead of random numbers for upload identifiers (64 animals)
- SQLite and JSON database support
- Private and public, editable and uneditable, automatically and never expiring uploads
- Automatic dark mode and custom styling support with very little CSS and only vanilla JS (see [`water.css`](https://github.com/kognise/water.css))
- And much more!

## What is an upload?

In ShareBin, an upload can be:

- A text that you want to paste from one machine to another, eg. some code,
- A file that you want to share, eg. a video that is too large for Discord, a zip with a code project in it or an image,
- A URL redirection.

## When is ShareBin useful?

You can use ShareBin:

- To send long texts to other people,
- To send large files to other people,
- To share secrets or sensitive documents securely,
- As a URL shortener/redirect service,
- To serve content on the web, eg. configuration files for testing, images, or any other file content using the Raw functionality,
- To move files between your desktop and a server you access from the console,
- As a "postbox" service where people can upload their files or texts, but they cannot see or remove what others sent you,
- Or even to take quick notes.

...and many other things, why not get creative?

ShareBin is available under the [BSD 3-Clause License](LICENSE).
