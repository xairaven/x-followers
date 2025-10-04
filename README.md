# X-Followers

Sometimes people follow me, I follow them back, 
and then they unfollow - and I have no idea who it was. 
To make it easy to find the traitors, you can use this 
utility. Time for all the unfollowers to 
start sweating 😁 

(I’m kidding, of course - but don’t unfollow me 🙄)

Example usage:

```bash
$ cli -help                 
Usage: cli [OPTIONS] --nickname <NICKNAME> --service <SERVICE>

Options:
  -n, --nickname <NICKNAME>        User nickname.
  -s, --service <SERVICE>          Service (Github, ..). [possible values: github]
  -d, --exclude-description        Exclude description from the first line.
  -o, --output-path <OUTPUT_PATH>  Path to the output file where results will be saved.
  -f, --file-name <FILE_NAME>      Filename.
  -h, --help                       Print help
  -V, --version                    Print version
```

Right now only the GitHub API integration is available, 
but I plan to expand to at least Instagram.