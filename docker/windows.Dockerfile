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
FROM mcr.microsoft.com/dotnet/sdk:6.0 AS builder

# TODO: build thing and install Git
# using: https://github.com/git-for-windows/git/releases/download/v2.35.3.windows.1/PortableGit-2.35.3-64-bit.7z.exe
