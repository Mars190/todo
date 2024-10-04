## Things to do before being able to deploy
1. Add a local runner: [Docs](https://docs.github.com/en/actions/hosting-your-own-runners/managing-self-hosted-runners/about-self-hosted-runners)
2. Change the ownership of the binary directory on your machine with the username that is used in the deployment
    ```bash
    sudo chown YOUR_USERNAME:YOUR_USERNAME /usr/local/bin
    sudo chmod u+w /usr/local/bin
    ```
3. [Add secrets](https://docs.github.com/en/actions/security-for-github-actions/security-guides/using-secrets-in-github-actions) for the environment variables in the `deploy.yml` action

## Keybinds

### General
- `ESC`, `Q`: Quit the program
- `H`: Help menu
- `A`: Add a todo
- `S`: Search
- `T`: Toggle between all, finished, unfinished
- `U`: Undo

### Navigation
- `0-9`, followed by space: Go to todo
- `Arrow up / Arrow down`: Go through list

### Specific when on a todo
- `C`: Mark as completed
- `E`: Edit a todo
- `D`, followed by space to confirm: Delete a todo

## Todo
- Basics: Add, Edit, Delete, Navigate