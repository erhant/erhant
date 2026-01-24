# Bun Aliases

I like Bun & use some aliases for quick commands, as below:

```sh
## json printer
alias bunjson='bun -p "Bun.inspect(await Bun.stdin.json(), { colors: true })"'

## json-parser & printer
alias bunjsonparse='bun -p "Bun.inspect(JSON.parse(await Bun.stdin.text()), { colors: true })"'

## word counter
alias bunwords='bun -p "(await Bun.stdin.text()).trim().split(/\s+/).filter(Boolean).length"'

## random UUIDv7
alias bunuuid='bun -p "Bun.randomUUIDv7()"'

## escape HTML characters
alias buneschtml='bun -p "Bun.escapeHTML(await Bun.stdin.text())"'

## random hex (via SHA256 of UUIDv7)
alias bunhex='bun -p "Bun.SHA256().update(Bun.randomUUIDv7()).digest(\"hex\")"'
```
