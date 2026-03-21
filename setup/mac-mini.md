# Mac Mini Setup

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
