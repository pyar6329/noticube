# noticube

SMTP Server which is notified to Slack rather than sending Email

# Usage

download command

```bash
$ sudo curl -L -o /usr/local/sbin/noticube https://github.com/pyar6329/noticube/releases/download/0.1.1/noticube-Linux-x86_64
$ sudo chmod +x /usr/local/sbin/noticube
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

## systemd

```bash
$ sudo curl -L -o /etc/systemd/system/noticube.service https://raw.githubusercontent.com/pyar6329/noticube/refs/heads/main/systemd/system/noticube.service
$ sudo curl -L -o /etc/default/noticube-credentials https://raw.githubusercontent.com/pyar6329/noticube/refs/heads/main/systemd/etc/noticube-credentials
$ sudo chmod 600 /etc/default/noticube-credentials

# Please change SLACK_BOT_TOKEN, SLACK_CHANNEL_ID
$ sudo vim /etc/default/noticube-credentials

$ sudo systemctl daemon-reload
$ sudo systemctl enable --now noticube
$ sudo systemctl status noticube
```
