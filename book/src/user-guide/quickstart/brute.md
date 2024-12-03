# Brute Force

Use the `brute` command to start a brute force attack with a specific wordlist

```bash
~$ subscan brute -d example.com --wordlist file.txt
```

To specify wordlist into docker container, use `/data` directory

```bash
~$ docker run -it --rm \
    --volume="$PWD/wordlist.txt:/data/wordlist.txt" \
    eredotpkfr/subscan brute -d example.com \
    -w wordlist.txt --print
```
