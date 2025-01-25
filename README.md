<h1 align="center" id="title">Git Navigator</h1>

<p align="center"><img src="https://socialify.git.ci/skiupace/GitNav/image?description=1&amp;font=KoHo&amp;forks=1&amp;issues=1&amp;language=1&amp;name=1&amp;owner=1&amp;pattern=Charlie+Brown&amp;stargazers=1&amp;theme=Dark" alt="project-image"></p>

<p id="description">a Simple TUI App to Navigate Through Git Repositories</p>

  
<h2>🧐 Features</h2>

Here're some of the project's best features :
*   Syntax Highlighting
*   Edit Files In Neovim

<h2>🛠️ Installation Steps</h2>

<p>1. Clone the repo</p>

```
git clone https://github.com/skiupace/Git-Nav.git
```

<p>2. Make sure rust is installed</p>

<p>3. Build the project</p>

```
cargo build --release
```

<p>4. Run the project</p>

```
cargo run
```


<h2>📸 Screenshots</h2>
<!-- for pypi only -->
<div style="text-align: center;">
   <p align="center"> <a href="https://ibb.co/0rqCSGV"><img src="https://i.ibb.co/Yh2cMLT/1737829538-grim.png" alt="1737829538-grim" border="0"></a> </p>
<p align="center"> <a href="https://ibb.co/s1MD3rJ"><img src="https://i.ibb.co/T0Fnh5q/1737833406-grim.png" alt="1737833406-grim" border="0"></a> </p>
</div>

## 👨‍💻 Usage

### Main Usage
```bash
git-nav [path to the git repo]
```

#### Shortcuts

| Shortcut | Description                                                                                                          |
| --------- | -------------------------------------------------------------------------------------------------------------------- |
| `Q`     | Quit the program                          |
| `Enter`     | Opens the File Inside Neovim                          |
| `Left Arrow`    | Open the folder |
| `Right Arrow`    | Go back on step in the folders tree |

  
<h2>💻 Built with</h2>
<p>Language: Rust.</p>

Dependencies used in the project :
*   crossterm
*   git2
*   ignore
*   ratatui
*   syntect
*   walkdir


<h2>🍰 Contribution Guidelines:</h2>
Pull requests are welcome. For major changes, please open an issue first to discuss what you want to change.
please follow the <a href="https://github.com/skiupace/GitNav/blob/main/CONTRIBUTING.md">contributing guidelines


<h2>🛡️ License</h2>

This project is licensed under the <a href="https://github.com/skiupace/GitNav/blob/main/LICENSE">MIT License
