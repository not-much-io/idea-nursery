# rdocker

rdocker is an enhancement to docker. Essentially rdocker adds new functionality to the docker command for managing remote environments for the purposes of managing development environments.

rdocker aims to make the fact that you are using a remote docker daemon opaque.

# Why?

It is very easy to have the default docker client use a remote docker daemon:
```
# All it takes!
$ export DOCKER_HOST=ssh://nmio@192.168.1.179
```

However this is only really useful for reading data from the remote docker daemon (ex. docker ps) or executing commands in already running containers (ex. docker exec container_name ls).
Specifically problems arise when building images, creating containers and using some functionalities:
1) The remote system doesn't have your source code for use as image build context and container run volumes
2) Even if you added your source code to the remote system manually, changes in remote/local filesystem wouldn't be mirrored back and forth while developing
3) Even if you set up two way file sync manually, volume sources would still point to your local filesystem paths and not remote ones
4) Even if you also modify the volume source paths your development environment might still not be equivalent to local system if you depend on a VPN or want ports to be forwarded locally
5) Even if you manually set up all network traffic on the remote to be routed through local and set up port forwarding you will now have to manage all the above manually

rdocker aims to easily manager all the above for you!

# Usage

## Installing

TODO
Setup ssh both ways (can be only client -> server?)

## Usage

To see what the new remote command does via an execution plan without executing anything use the --dry-run flag:
```
# Manage remote - setup remote
$ docker remote \
      --dry-run \
      --setup \
      --name RemoteName \
      --ip x.x.x.x \
      --user nmio \
      --dir /tmp

# Manage remote - teardown remote
$ docker remote \
      --dry-run \
      --teardown
```

## How does it work?

"rdocker" is composed of two components: rdocker and rdockerd

### rdocker

rdocker is a proxy program that installs itself between the users commands to the default docker client.

Essentially rdocker stores the path to the real docker binary and installs itself as the new `docker` command.
So now when executing a command like `docker ps` it is actually rdocker that accepts the ps command and forwards it to the original docker command and returns the output.

rdocker adds one command - `docker remote`. This command allows managing remote environments possible via one command, things like setting up source code on the remote host, start fs sync between the local and remote filesystems, start ssh tunneling etc.
See `Usage` for more.

Most of the normal docker commands are left alone but some are intercepted by rdocker for modification, for example:
```
$ docker ... --volume /some/local/path:/some/container/path
```

This wouldn't work as /some/local/path doesn't exist on the remote!
While rdocker did move the source code to the remote machine, it doesn't make sense to try to replicate the local filesystem path in the remote machine.
Instead rdocker catches volume parameters to the docker client, modifies them to match the path of the source code on the remote machine and passes the modified parameters to the docker client!

### rdockerd

rdockerd is a daemon that runs on the remote machine and that the rdocker client calls to execute the under the hood functionality of the `remote` command.
