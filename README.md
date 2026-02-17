# Pandora Launcher

Work in progress

## Features
- Instance management
- Cross-instance file syncing (options, saves, etc.) (https://youtu.be/wb5EY2VsMKg)
- Mod deduplication when installed through launcher (using hard links)
- Secure account credential management using platform keyrings
- Custom game output window
- Mod browser using Modrinth's API
- Automatic redaction of sensitive information (i.e. access tokens) in logs
- Unique approach to modpack management (https://youtu.be/cdRVqd7b2BQ)

## FAQ

### Where can I suggest a feature/report a bug?

Please use GitHub issues.

### Why should I use Pandora over other launchers?

At this point, you probably shouldn't since it doesn't have feature parity with other launchers.

### Will Pandora be monetized?

Unlikely, for a few reasons:
- I believe that it is wrong for launchers to be monetized without distributing revenue back to mod creators that give the launcher value in the first place. Since I don't have the infrastructure to be able to redistribute revenue to mod creators, this is a big barrier.
- Dealing with monetization takes a lot of (ongoing) work, probably more work than creating the launcher itself.
- I personally dislike advertisements.

## Instance Page
![Instance Page](https://raw.githubusercontent.com/Moulberry/PandoraLauncher/refs/heads/master/screenshots/instance.png)

Note from the forker "SAMXRA1": There is an easy way to bypass the need for being connected to / logged into a Mojang account to create an offline account, by just commenting out one part of crates/frontend/src/ui.rs @ 331. However, when I do that, and build the binary, I run into some warnings and/or errors. I am NOT a developer. I have NEVER coded in Rust. I have no experience whatsoever. For those reasons, I have not commented it out as of yet, but I will in the near future, when I deem myself to be ready, and with a solution/alternative to the warnings. Thank you. Thanks to Moulberry for creating PandoraLauncher. I'll add the code change needed as a file in the main branch and add it to .gitignore for an unhindered workflow. Also, thanks to that random guy who gave me the idea and the actual code change required to make it work. You are GOATED bruh!

PLEASE DON'T REPORT ME😭
PLEASE MOULBERRY I NEED THIS!
