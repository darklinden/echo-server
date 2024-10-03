# Echo Server

This is a simple echo server that echoes back whatever it receives.

## Simple Usage

-   To run the server, run the following command:

```
echo-server [port number, default is 3000]
```

## Advanced Usage

-   <https://github.com/darklinden/h5-serve-local> is a simple nginx docker server with ssl support.
-   You can use [h5-serve](https://github.com/darklinden/h5-serve-local) with [test/index.html](./test/index.html) to test the server.
    -   1. Run the echo server, listening on port 3000.
    -   2. Cd to the [test](./test) directory, and run the following command:
    ```
    h5-serve
    ```
    -   3. By default, it will open the browser and navigate to <https://local.darklinden.site>
    -   4. You can connect to <wss://local-ws.darklinden.site> to test the websocket connection.
