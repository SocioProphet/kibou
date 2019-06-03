# Kibou

## What is Kibou?
Kibou is a lightweight federated social networking server based on open protocols. It is
written in Rust, utilizes Rocket as it's web framework and Diesel as it's database driver.

The project's objective is to provide a highly customizable multi-protocol social networking server. Currently supported is the commonly used [ActivityPub](https://activitypub.rocks) protocol.

Furthermore Kibou implements [Mastodon's REST API](https://docs.joinmastodon.org/api). This means that all applications for [Mastodon](https://joinmastodon.org) should also work with Kibou.

Kibou ships with it's own user interface called Raito-FE. In it's standard configuration it's completely based on static components and does not use any JavaScript. Although dynamic components (such as automatically refreshing timelines and dynamic routing) can optionally be enabled. A `minimal mode` can also be enabled in it's settings which reduces network traffic and only shows Raito-FE's core components.

**Notice**: Kibou is *not* considered stable yet. This project's development is still
work-in-progress. Using the development branch is not recommended for production usage.

## Federation with other software
Federation is known to work with [Pleroma](https://pleroma.social), [Misskey](https://joinmisskey.github.io) and [Mastodon](https://joinmastodon.org) which are also the main projects being tested against. But federation with other software should work as well.

## Get in touch
Join the IRC channel `#kibou` on freenode.net
