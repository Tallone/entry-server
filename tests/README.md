### We Using [HURL](https://hurl.dev/docs/installation.html) to test Entry server api.

Most APIs require a token, so you need to first obtain the token and then use `--variable token=token` to pass the variable.

```Shell

hurl --error-format=long --variable token=token xxx.hurl
```

## Make a token

1. using `create.hurl` create a new user
2. using `login.hurl` to login and get a token