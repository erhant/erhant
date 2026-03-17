# Setup

Note to self because I'm dumb I should write down what I do when I change a PC (most probably a Mac):

1. Download Brave Browser (using Safari)
1. Install [GitHub CLI](https://cli.github.com/) & authenticate
1. Install Brave extensions: Rabby, MetaMask, Phantom
1. Download [Ghostty](https://ghostty.org/download) 👻
1. Setup `zsh` with <https://github.com/gustavohellwig/gh-zsh>
1. Configure terminal with `p10k configure`
1. Download [Spotify](https://www.spotify.com/download/), open up some jazz in the background
1. Import bookmarks (hopefully you have them)
1. Download VSCode & sign-in with GitHub and sync settings
1. Install [Homebrew](https://brew.sh/)
1. Install [FiraCode](https://fonts.google.com/specimen/Fira+Code) font
1. Download Discord
1. Download Zoom (always need it somewhere)
1. Download [Zotero](https://www.zotero.org/download/) and its connector extension
1. Download [Telegram](https://apps.apple.com/us/app/telegram-messenger/id686449807) & [Whatsapp](https://apps.apple.com/us/app/whatsapp-messenger/id310633997)
1. Download [Docker Desktop](https://www.docker.com/products/docker-desktop/)
1. Install [Rust](https://rust-lang.org/tools/install/)
1. Install [Uv](https://docs.astral.sh/uv/getting-started/installation/) (for Python)
1. Install [NodeJS](https://nodejs.org/en/download), and install `yarn` and `pnpm` as well
1. Install [BunJS](https://bun.com/docs/installation)
1. Install [Foundry](https://www.getfoundry.sh/)
1. Install [Circom](https://docs.circom.io/getting-started/installation/)

## Mac Mini Setup

I have a Mac Mini that I had setup separately. Then, I've placed it alongside the router with an Ethernet connection, and nothing else (no keyboard / mouse / monitor). I have allowed <kbd>Settings > Sharing > Remote Login</kbd> and <kbd>Settings > Sharing > Screen Sharing</kbd> to connect to it via SSH and Screen Sharing if needed, via local network.

I have SSH'ed into it, and did the following:

### Setup Terminal

```sh
# install: https://github.com/gustavohellwig/gh-zsh
sudo curl -fsSL https://raw.githubusercontent.com/gustavohellwig/gh-zsh/main/gh-zsh.sh | bash

# restart terminal (-l replaces current one)
exec zsh -l

# configure powerlevel10k
p10k configure
```

### Setup Brew (and friends)

```sh
# install brew: https://brew.sh/
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# restart terminal
exec zsh -l

# add btop for monitoring
brew install btop

# add neofetch because why not
brew install neofetch
```

### Setup Language Stuff

```sh
# python with Uv (https://docs.astral.sh/uv/getting-started/installation/)
curl -LsSf https://astral.sh/uv/install.sh | sh

# rust (https://rust-lang.org/tools/install/)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# typescript / javascript with Bun (https://bun.com/docs/installation)
curl -fsSL https://bun.com/install | bash
```

### Setup Git SSH

```sh
# https://docs.github.com/en/enterprise-cloud@latest/authentication/connecting-to-github-with-ssh/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent
# generate new key
ssh-keygen -t ed25519 -C "erhany96@gmail.com"

# start agent
eval "$(ssh-agent -s)"

# open (or touch first if needed) config, and paste the code below:
#
# Host github.com
#   AddKeysToAgent yes
#   UseKeychain yes
#   IdentityFile ~/.ssh/id_ed25519
open ~/.ssh/config

# add key
ssh-add ~/.ssh/id_ed25519

# https://docs.github.com/en/enterprise-cloud@latest/authentication/connecting-to-github-with-ssh/adding-a-new-ssh-key-to-your-github-account
# get public key to clipboard (or just cat it)
pbcopy < ~/.ssh/id_ed25519.pub

# add it to your account at https://github.com/settings/keys
```
