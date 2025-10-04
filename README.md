# X-Followers

Sometimes people follow me, I follow them back, and then they 
unfollow - leaving me clueless about who did it. To make catching the 
traitors easy, use this utility to save a snapshot of your followers, 
and later, after a betrayal, generate a new snapshot and compare the two 
with a diff - either the built-in Linux command or any online tool. 
Time for all the unfollowers to start sweating 😁

(I’m kidding, of course - but don’t unfollow me 🙄)

Example usage:

```bash
$ cli -help                 
Usage: cli [OPTIONS] --nickname <NICKNAME> --service <SERVICE>

Options:
  -n, --nickname <NICKNAME>        User nickname.
  -s, --service <SERVICE>          Service (Github, ..). [possible values: github]
  -d, --exclude-description        Exclude description from the first line.
  -o, --output-path <OUTPUT_PATH>  Path to the output folder where results will be saved.
  -f, --file-name <FILE_NAME>      Output file name.
  -h, --help                       Print help
  -V, --version                    Print version
```

Right now only the GitHub API integration is available, 
but I plan to expand to at least Instagram.