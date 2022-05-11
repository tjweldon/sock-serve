# Sock-Serve

This a super simple client & server for one way communication (client -> server) over unix sockets.

## Usage

The compiled executable has two modes, `client` and `server`.

For the purposes of this README we're going to assume the executable is called `sock-serve`

You should start this one first:

```bash
$ ./sock-serve server

```

This will run in the foreground, so open up another shell and run

```bash
$ ./sock-serve client

```

Your shell where you sarted the server mode should now look like this:

```bash
$ ./sock-serve server
Got a client: UnixStream { fd: FileDesc(OwnedFd { fd: 4 }), local: "/tmp/rst.sock" (pathname), peer: (unnamed) } - (unnamed)
```

Now switch back to your client session and try typing something in and pressing enter.


```bash
$ ./sock-serve client
hello
world

```

Your server session should now show

```bash
$ ./sock-serve server
Got a client: UnixStream { fd: FileDesc(OwnedFd { fd: 4 }), local: "/tmp/rst.sock" (pathname), peer: (unnamed) } - (unnamed)
hello
world
```


