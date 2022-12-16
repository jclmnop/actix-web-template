# actix-web template

Just a very basic template which I plan to use for any future actix-web applications.

## CI

GitHub Action files stolen from [here](https://gist.github.com/LukeMathWalker/5ae1107432ce283310c3e601fac915f3#file-audit-on-push-yml).

Setup:

- Add protection rule for trunk
- Write unit and integration tests in `/tests` dir
  - Can also write embedded unit tests if something non-public needs to be tested
- Add workflow files from `.github/` dir as checks for PRs to master and commits to master
- Add github token to secrets
