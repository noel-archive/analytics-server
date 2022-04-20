// 📊✏️ analytics-server: Enable analytical and monitoring service for self-hosted Noelware products.
// Copyright 2022 Noelware <team@noelware.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

using ClickHouse.Client.ADO;
using Microsoft.Extensions.Configuration;
using Serilog;

namespace Noelware.Analytics.Server.Services;

public class ClickHouseService
{
    private readonly ILogger _log = Log.ForContext<ClickHouseService>();
    private readonly IConfigurationRoot _config;
    private ClickHouseConnection? _connection;

    public ClickHouseService(IConfigurationRoot config)
    {
        _config = config;
    }

    public async Task Connect()
    {
        _log.Information("Now connecting to ClickHouse...");

        var clickhouseSettings = _config.GetSection("clickhouse");
        if (clickhouseSettings == null || clickhouseSettings?.Exists() == false)
        {
            _log.Fatal("Missing `clickhouse` configuration. (https://analytics.noelware.org/docs/self-hosting/configuration#clickhouse)");
            Environment.Exit(1);
        }

        var connectionString = $"Host={clickhouseSettings!["host"]};Port={clickhouseSettings!["port"]};Username={clickhouseSettings!["username"]};Password={clickhouseSettings!["password"]};Database={clickhouseSettings!["database"]}";
        _connection = new ClickHouseConnection(connectionString);
        await _connection.OpenAsync();
        
        _log.Information("Connection *should* be established! Using v{ServerVersion}", _connection.ServerVersion);
    }

    public void Dispose()
    {
        _log.Warning("Disposing ClickHouse connection...");
        _connection.Dispose();
    }
}
