# Setup

Note to self because I'm dumb I should write down what I do when I change a PC (most probably a Mac):

1. Download Brave Browser (using Safari)
1. Install Brave extensions: Rabby, MetaMask, Phantom
1. Download Spotify, open up some jazz in the background
1. Import bookmarks (hopefully you have them)
1. Download VSCode & sign-in with GitHub and sync settings
1. Install Homebrew
1. Install FiraCode font
1. Download Discord
1. Download Zoom (always need it somewhere)
1. Download Zotero (paper reading etc.)
1. Download Telegram & Whatsapp Desktop
1. Download Ghostty 👻
1. Setup `zsh` with <https://github.com/gustavohellwig/gh-zsh>
1. Configure terminal with `p10k configure` inside VSCode integrated terminal
1. Login to stuff in your bookmarks, especially Google account
1. Download Docker Desktop
1. Install Rust
1. Install Python, and install `uv`
1. Install NodeJS, and install `yarn` and `pnpm` as well
1. Install BunJS
1. Install Foundry
1. Install Circom
1. Setup GitHub SSH keys
1. Download Ollama

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

### Setup Brew

```sh
# install brew: https://brew.sh/
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# restart terminal
exec zsh -l
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

