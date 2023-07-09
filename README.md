# docker-foreground-container

A foreground process that starts an existing docker container when run, and stops it when terminated.

Allows control of a docker container via a process supervisor, 
e.g. [supervisord](http://supervisord.org/subprocess.html#nondaemonizing-of-subprocesses).