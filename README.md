# hn-rs

hn-rs is a Hacker News TUI (Terminal User Interface) client, written in Rust.
You can browse Hacker News articles by topic, read article content, and view comments — all from your terminal.

![screenshot](./assets/hn-rs.gif)

# Features
- Browse stories from different Hacker News sections
- View full article content in a readable layout
- Read nested comments
- Smooth navigation with familiar keybindings
- Fully terminal-based, perfect for neovim + tmux workflows

Planned: user profiles and more...

# Why This Project

As someone who works heavily in the terminal using tools like Neovim and Tmux, I wanted to reduce context switching between GUI apps and the terminal.
Building a terminal-first Hacker News client felt like a great way to practice Rust while solving a personal itch.

# usage

List panel:  
- `Tab`/`S-Tab` - switch topic
- `j`/`k` - navigate between the articles  
- `l`/`Enter` - switch focus to the Article panel 

Article panel:  
- `j`/`k` - scroll the view  
- `h`/`Esc` - switch focus to the List panel  
- `c` - toggle to focus the comments panel  
- `o` - open link in browser

Comments Panel (WIP)
- Same navigation as article panel (scroll with `j`/`k`)