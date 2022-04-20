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

using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Server.Kestrel.Core;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Noelware.Analytics.Server.Controllers;
using Noelware.Analytics.Server.Extensions;
using Noelware.Analytics.Server.Services;
using Serilog;
using Serilog.Events;
using Serilog.Sinks.Tcp;
using Serilog.Sinks.Tcp.Formatters;

namespace Noelware.Analytics.Server;

public class Program
{
    static async Task Main(string[] args)
    {
        Thread.CurrentThread.Name = "main";
        
       // Setup configuration here
       var config = setupConfig(args);
       
       // Setup Serilog
       setupSerilog(config);
       
       // Setup ASP.Net Core here
       var builder = WebApplication.CreateBuilder(args);

       builder.WebHost.ConfigureKestrel(kestrel =>
       {
           var httpConfig = config.GetSection("http");
           int port;

           if (httpConfig.Exists())
           {
               port = httpConfig.GetValue<int>("port");
           }
           else
           {
               port = 3321;
           }
           
           kestrel.ListenAnyIP(port, c =>
           {
               c.UseConnectionLogging();
               c.Protocols = HttpProtocols.Http1AndHttp2;
           });
       });

       builder.Services.AddApiVersioning(opts =>
       {
           opts.DefaultApiVersion = new ApiVersion(1, 0);
           opts.AssumeDefaultVersionWhenUnspecified = true;
           opts.Conventions.Controller<ApiV1Controller>().HasApiVersion(new ApiVersion(1, 0));
       });
       
       builder.Services.AddSingleton(config);
       builder.Services.AddSingleton<ClickHouseService>();
       builder.Services.AddControllers();

       builder.Host.UseSerilog(Log.Logger, true);

       var log = Log.ForContext<Program>();
       var app = builder.Build();
       var clickhouse = app.Services.GetService<ClickHouseService>()!;

       await clickhouse.Connect();

       if (builder.Environment.IsDevelopment())
       {
           app.UseDeveloperExceptionPage();
       }

       log.Information("Starting up HTTP service...");

       app.UseRouting();
       app.UseSerilogRequestLogging();
       app.UseEndpoints(opts =>
       {
           opts.MapControllers();
       });
       
       app.UseCors();
       app.UseHsts();
       await app.RunAsync();
    }

    private static IConfigurationRoot setupConfig(string[] args)
    {
        var yamlPath = Environment.GetEnvironmentVariable("ANALYTICS_SERVER_CONFIG_PATH") ?? "./config.yml";
        var builder = new ConfigurationBuilder()
            .AddYamlFile(yamlPath, true)
            .AddCommandLine(args)
            .AddEnvironmentVariables("ANALYTICS_SERVER_");

        return builder.Build();
    }

    private static void setupSerilog(IConfigurationRoot config)
    {
        var log = config.GetSection("logging");
        var cfg = new LoggerConfiguration()
            .Enrich.WithAssemblyName()
            .Enrich.WithProcessId()
            .Enrich.WithThreadName()
            .Enrich.WithThreadId();

        if (log.Exists())
        {
            // Check the log level and convert it
            var defaultFormatter =
                "{Timestamp:yyyy-MM-dd hh:MM:ss.sss} [{Level:u5} - {SourceContext:l} ({AssemblyName} ~ {ProcessId} ~ {ThreadName} #{ThreadId})] {Message}{NewLine}{Exception}";
            
            var level = log.GetValue<string>("level").AsLevel();
            cfg.WriteTo.Console(level, log.GetValue<string>("format").IfEmpty(() => defaultFormatter));
            
            // Check if logstash is enabled
            var logstashCfg = log.GetSection("logstash");
            if (logstashCfg.Exists())
            {
                var endpoint = logstashCfg.GetValue<string>("endpoint");
                cfg.WriteTo.TcpSink(endpoint, new LogstashJsonFormatter(), restrictedToMinimumLevel: level);
            }
        }
        else
        {
            cfg.WriteTo.Console(LogEventLevel.Information, "{Timestamp:yyyy-MM-dd hh:MM:ss.sss} [{Level:u5} - {SourceContext:l} ({AssemblyName} ~ {ProcessId} ~ {ThreadName} #{ThreadId})] {Message}{NewLine}{Exception}");
        }

        Log.Logger = cfg.CreateLogger();
    }
}
