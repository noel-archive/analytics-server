# 📊 Noelware Analytics • Helm Chart
This is the source of the helm chart for Noelware Analytics.

## Requirements
- Kubernetes cluster >=**1.23**
- Helm **3**
- ClickHouse cluster (if `clickhouse.enabled` is set to **false**)

## Installation
```shell
# 1. We need to index the `Noelware` repositories from the organization.
$ helm repo add noelware https://charts.noelware.org/noelware

# 2. We can install the Helm Chart!
# This will run a ClickHouse cluster (if `values.clickhouse.enabled` is true), or you can provide a
# cluster if you installed one on your Kubernetes cluster.
$ helm install noelware-analytics-server noelware/analytics-server
```

## Values
; ^o^ - coming soon~ ^o^ ;
