# noticube

SMTP Server which is notified to Slack rather than sending Email

# Usage

download command

```bash
$ curl -o noticube https://github.com/pyar6329/noticube/releases/download/0.1.0/noticube-Linux-x86_64
$ sudo mv noticube /usr/local/bin/noticube
$ sudo chmod +x /usr/local/bin/noticube
```

and run it!

```bash
$ export SLACK_BOT_TOKEN="xxxxxx"
$ export SLACK_CHANNEL_ID="yyyyyy"
$ noticube
```

## Docker

```bash
$ docker run \
    -d \
    --restart=always \
    -e NOTICUBE_PORT="2525" \
    -e NOTICUBE_IP="0.0.0.0" \
    -e SLACK_BOT_TOKEN="xxxxxx" \
    -e SLACK_CHANNEL_ID="yyyyyy" \
    ghcr.io/pyar6329/noticube:0.1.1
```

## Kubernetes

Please change environment variables in [`kubernetes/deployment.yaml`](https://github.com/pyar6329/noticube/blob/main/kubernetes/deployment.yaml#L27-L28)

```bash
$ git clone git@github.com:pyar6329/noticube.git
$ cd noticube
$ kubectl apply -f kubernetes/deployment.yaml
```
