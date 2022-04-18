# analytics-server - Docker Images
This is the directory where all the Docker images pushed to Docker Hub or the GitHub Container Registry are. The files are listed
as `<arch|os>.Dockerfile`, so:

- linux/amd64 -> [amd64.Dockerfile](./amd64.Dockerfile)
- linux/arm64 -> [arm64.Dockerfile](./arm64.Dockerfile)
- windows/arm64 -> [windows.Dockerfile](./windows.Dockerfile)

The [scripts](./scripts) directory are only for the Linux containers because I do not want to write init scripts in PowerShell or .bat to keep sake of my sanity. :(
