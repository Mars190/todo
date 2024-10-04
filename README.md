## Things to do before being able to deploy
1. Add a local runner: [Docs](https://docs.github.com/en/actions/hosting-your-own-runners/managing-self-hosted-runners/about-self-hosted-runners)
2. Change the ownership of the binary directory on your machine with the username that is used in the deployment
    ```bash
    sudo chown YOUR_USERNAME:YOUR_USERNAME /usr/local/bin
    sudo chmod u+w /usr/local/bin
    ```
3. [Add secrets](https://docs.github.com/en/actions/security-for-github-actions/security-guides/using-secrets-in-github-actions) for the environment variables in the `deploy.yml` action