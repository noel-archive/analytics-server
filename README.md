# 📊 Noelware Analytics
> *Noelware Analytics enables analytical and monitoring services for self-hosted Noelware products.*

## What is this?
**Noelware Analytics** is a project to collect analytical data in Noelware's products (only applicable to self-hosting) to have a metric visualisation of what is going on.

The analytics server will collect metrics of:

- Is it running via **Docker** or **Kubernetes**?
- CPU usage, memory usage, hostname
- Errors handled when the **gRPC** gateway is enabled
- Instance UUID

If you enable the `Keep Track of Data` setting under the dashboard settings, we will store the history of whats collected under **ClickHouse** so you can keep a old history of the data that was tracked. You can also export the data in **JSON** format when you hit the `Export Data` button under the dashboard settings.

## How does it work?
It works by hitting an internal **gRPC** server that the products can run IF the analytics engine is enabled, usually it's not ran by default, since
we do not want to enable other Noelware services if the end user doesn't want to by default!

You can register your instance [here](https://analytics.noelware.org/register), you can collect the instance UUID by hitting `GET /version`, which is a unique identifier of your instance. All of the instances you run are connected to your **Noelware** account, you can invite other people with their Noelware account to view and edit your instance for your system administrators.

You can also setup **Alerting** to hit a webhook service, send an email, and more to monitor it in the background, you can read more about
the [Alerting](https://analytics.noelware.org/docs/alerting) feature.

## Supported Products
- [charted-server](https://github.com/charted-dev/charted)
- [Arisu](https://github.com/arisuland/Tsubaki)
- [gitjb daemon](https://github.com/gitjb-dev/daemon)

## Installation
You can install **Noelware Analytics** through [Docker](#docker), the [Helm Chart](#helm-chart), or locally with [Git](#git).

### Prerequisities
You don't need much to run **Noelware Analytics** if you're trying to contribute to make the service better, but you will need:

- **.NET Core** >=6 (if building from source)
- **Visual Studio 2022** or **JetBrains Rider** (if contributing)

The Docker and Helm Chart installations come with the .NET SDK in the container image itself, so it's not needed.

### Docker
You can run **Noelware Analytics** with the images provided on [Docker Hub](https://hub.docker.com/r/noelware/analytics-server) or on
the [GitHub Container Registry]().

We support Linux container running on x86_64 or ARM64-based CPU architectures and Windows on a x86_64 CPU architecture.

You will need an instance of a **ClickHouse** cluster running before running **analytics-server**.

```shell
# 1. Pull the image from Docker Hub or GitHub Container Registry.
# <version> refers to the version from the specification below.
# <registry> refers to the Docker registry to use. You can omit this as empty if using Docker Hub.
$ docker pull <registry>/noelware/analytics-server:<version>

# 2. Run the image!
$ docker run -d -p 9898:9898 --name analytics-server -v ~/config.toml:/app/noelware/analytics/server/config.toml <registry>/noelware/analytics-server:<version>
```

#### Version Specification
**Noelware Analytics** supports a unofficial specification for versioning for Docker images. The versions can look like:

- **latest** | **latest-[arch]** | **latest-[arch][-os]**
- **[major].[minor]** | **[major].[minor][-arch]** | **[major].[minor][-arch][-os]**
- **[major].[minor].[patch]** | **[major].[minor].[patch][-arch]** | **[major].[minor].[patch][-arch][-os]**

| Image Version       | Acceptable | Why?                                                                            |
| ------------------- | ---------- | ------------------------------------------------------------------------------- |
| `latest`            | 💚         | defines as **linux/amd64** or **linux/arm64** with the latest release.          |
| `latest-amd64`      | 💚         | defines as **linux/amd64** with the latest release.                             |
| `latest-windows`    | 💚         | defines as **windows/amd64** with the latest release.                           |
| `0.2`               | 💚         | defines as **linux/amd64** or **linux/amd64** with the **0.2** release.         |
| `0.2-windows`       | 💚         | defines as **windows/amd64** with the **0.2** release.                          |
| `0.2-arm64-linux`   | 💚         | defines as **linux/arm64** with the **0.2** release.                            |
| `latest-linux`      | ❤️         | Linux releases do not need a `-os` appended.                                    |
| `0.2-amd64-windows` | ❤️         | Windows releases do not need an architecture since it only uses **amd64** only. |
| `linux-amd64`       | ❤️         | What version do we need to run? We only know the OS and Architecture.           |
| `amd64`             | ❤️         | What version or operating system to run? We only know the architecture.         |

### Helm Chart
You can install **analytics-server** on Kubernetes using the Helm Chart provided by Noelware. You can view the chart source [here](./charts). :)

You are required to have a Kubernetes cluster with version >=**1.23** and Helm **3**.

```shell
# 1. We need to index the `Noelware` repositories from the organization.
$ helm repo add noelware https://charts.noelware.org/noelware

# 2. We can install the Helm Chart!
# This will run a ClickHouse cluster (if `values.clickhouse.enabled` is true), or you can provide a
# cluster if you installed one on your Kubernetes cluster.
$ helm install noelware-analytics-server noelware/analytics-server
```

### Locally with Git
This is the guide if you plan to contribute or want to build the server from the GitHub repository. You are required the **.NET SDK** 6 :)

```shell
# 1. Pull the repository to your computer.
$ git clone https://github.com/Noelware/analytics-server && cd analytics-server

# 2. Restore the dependencies that Noelware.Analytics.Server requires.
$ dotnet restore

# 3. Build a .dll file to run it.
$ dotnet publish Noelware.Analytics.Server -c Release

# 4. Run the server!
$ dotnet src/Noelware.Analytics.Server/bin/debug/net6.0/Noelware.Analytics.Server.dll
```

## Configuration
Coming soon!

## Contributing
Thanks for considering contributing to **Noelware Analytics**! Before you boop your heart out on your keyboard ✧ ─=≡Σ((( つ•̀ω•́)つ, we recommend you to do the following:

- Read the [Code of Conduct](./.github/CODE_OF_CONDUCT.md)
- Read the [Contributing Guide](./.github/CONTRIBUTING.md)

If you read both if you're a new time contributor, now you can do the following:

- [Fork me! ＊*♡( ⁎ᵕᴗᵕ⁎ ）](https://github.com/Noelware/analytics-server/fork)
- Clone your fork on your machine: `git clone https://github.com/your-username/analytics-server`
- Create a new branch: `git checkout -b some-branch-name`
- BOOP THAT KEYBOARD!!!! ♡┉ˏ͛ (❛ 〰 ❛)ˊˎ┉♡ ✧ ─=≡Σ((( つ•̀ω•́)つ
- Commit your changes onto your branch: `git commit -am "add features （｡>‿‿<｡ ）"`
- Push it to the fork you created: `git push -u origin some-branch-name`
- Submit a Pull Request and then cry! ｡･ﾟﾟ･(థ Д థ。)･ﾟﾟ･｡

## License
**analytics-server** is made with love 💜 and released under the [Apache 2.0](/LICENSE) License by Noelware. :3
