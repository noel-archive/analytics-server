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

using Serilog.Events;

namespace Noelware.Analytics.Server.Extensions;

public static class LogLevelExtensions
{
    public static LogEventLevel AsLevel(this string str)
    {
        switch (str)
        {
            case "information":
            case "info":
                return LogEventLevel.Information;

            case "debug":
                return LogEventLevel.Debug;
            
            case "verbose":
                return LogEventLevel.Verbose;
            
            case "warning":
            case "warn":
                return LogEventLevel.Warning;
            
            case "error":
                return LogEventLevel.Error;
            
            case "fatal":
                return LogEventLevel.Fatal;

            default: return LogEventLevel.Information;
        }
    }
}
