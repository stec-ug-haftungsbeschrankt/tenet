# tenet

Tenant, application and user mangement written in rust


The data of this service is stored in a postgresql database.


URL Schema to access application

> https://apps.stecug.de/<tenant-id>/<application-id>

## Testing

We use unit/integration tests. In order to run them you need `docker` running and have `cargo-nexttest` installed. You can do this with:

```bash
> cargo install cargo-nextest --locked
```

To test run the tests, use the following command:

```bash
> cargo nextest run
```