# 📊✏️ analytics-server: Enable analytical and monitoring service for self-hosted Noelware products.
# Copyright 2022 Noelware <team@noelware.org>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# Create the builder stage so we can build the server.
FROM mcr.microsoft.com/dotnet/sdk:6.0-alpine AS builder

RUN apk upgrade && apk add --no-cache git ca-certificates
WORKDIR /build/analytics/server

# Copy csproj and restore as distinct layers
COPY *.sln .
COPY src/Noelware.Analytics.Server/Noelware.Analytics.Server.csproj ./src/Noelware.Analytics.Server/Noelware.Analytics.Server.csproj
RUN dotnet restore -r linux-musl-arm64

# Copy everything else and build the application
COPY src/Noelware.Analytics.Server ./src/Noelware.Analytics.Server
RUN dotnet publish -c Release -o /app -r linux-musl-arm64 --self-contained false --no-restore

# Final stage: the runtime image!
FROM mcr.microsoft.com/dotnet/aspnet:6.0-alpine-arm64v8

# Installs bash so we can execute the Docker scripts
# This image installs `tini`, a valid "init" for containers.
RUN apk upgrade && apk add --no-cache bash tini icu-libs

# Set the working directory to /app/noelware/analytics/server
WORKDIR /app/noelware/analytics/server

# Copy the build source to this image
COPY --from=builder /app .

# Copy the Docker scripts into this image.
COPY ./docker/scripts /app/noelware/analytics/server/scripts

# Enable globalization APIs (for no reason, just for fun >:3)
# English (British) > English AU / US, fight me.
ENV \
    DOTNET_SYSTEM_GLOBALIZATION_INVARIANT=false \
    LC_ALL=en_GB.UTF-8 \
    LANG=en_GB.UTF-8

# Make sure the scripts can be executed
RUN chmod +x /app/noelware/analytics/server/scripts/docker-entrypoint.sh /app/noelware/analytics/server/scripts/run.sh

# Do not run as root!
USER 1001

# Set the Docker scripts for the entrypoint and command line interface.
ENTRYPOINT ["/app/noelware/analytics/server/scripts/docker-entrypoint.sh"]
CMD ["/app/noelware/analytics/server/scripts/run.sh"]
