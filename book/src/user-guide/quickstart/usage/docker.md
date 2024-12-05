# Docker Usage

Once you’ve [pulled](/user-guide/quickstart/install.md#pull-docker-image) the pre-built image from [Docker Hub](https://hub.docker.com/), you can easily run the container to perform subdomain enumeration

```bash
~$ docker run -it --rm eredotpkfr/subscan scan -d example.com
```

Specify environment variable via docker `--env`

```bash
~$ docker run -it --rm \
    --env SUBSCAN_VIRUSTOTAL_APIKEY=foo \
    eredotpkfr/subscan scan -d example.com --modules=virustotal
```

Specify `.env` file from your host machine, use `/data` folder

```bash
~$ docker run -it --rm \
    --volume="$PWD/.env:/data/.env" \
    eredotpkfr/subscan scan -d example.com --skips=commoncrawl
```

Saving output reports to host machine, use `/data` folder

```bash
~$ docker run -it --rm \
    --volume="$PWD/data:/data" \
    eredotpkfr/subscan scan -d example.com
```

To specify wordlist into docker container, use `/data` directory

```bash
~$ docker run -it --rm \
    --volume="$PWD/wordlist.txt:/data/wordlist.txt" \
    eredotpkfr/subscan brute -d example.com \
    -w wordlist.txt --print
```
