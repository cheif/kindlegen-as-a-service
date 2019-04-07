# Utilitiy for running [kindlegen](https://www.amazon.com/gp/feature.html?ie=UTF8&docId=1000765211<Paste>) as a service

This is a small wrapper for running kindlegen as a service, as opposed to running it as a CLI.

## Using

It's written in rust, and can be built using docker:

```
docker build -t service .
```

It can then be run with:

```
docker run -p 8080:8080 service
```
