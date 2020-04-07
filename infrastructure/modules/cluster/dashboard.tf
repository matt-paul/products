resource "azurerm_dashboard" "doc-index-updater-dashbaord" {
  name                = "doc-index-updater-dashboard"
  resource_group_name = var.resource_group_name
  location            = var.location

  tags = {
    Environment = var.environment
  }

  dashboard_properties = <<DASH
{
  "lenses": {
    "0": {
      "order": 0,
      "parts": {
        "0": {
          "position": {
            "x": 0,
            "y": 0,
            "colSpan": 6,
            "rowSpan": 4
          },
          "metadata": {
            "inputs": [
              {
                "name": "ComponentId",
                "value": {
                  "SubscriptionId": "bec11470-1346-4cdd-af2e-ce1f360671a1",
                  "ResourceGroup": "mhra-products-development",
                  "Name": "mhra-products-cluster-analytics-gv55",
                  "ResourceId": "/subscriptions/bec11470-1346-4cdd-af2e-ce1f360671a1/resourcegroups/mhra-products-development/providers/microsoft.operationalinsights/workspaces/mhra-products-cluster-analytics-gv55"
                }
              },
              {
                "name": "Query",
                "value": "InsightsMetrics\n| where Namespace == \"prometheus\"\n| where Name == \"job:doc_index_updater_request_error_rate:mean\"\n| render timechart \n"
              },
              {
                "name": "TimeRange",
                "value": "PT30M"
              },
              {
                "name": "Dimensions",
                "value": {
                  "xAxis": {
                    "name": "TimeGenerated",
                    "type": "datetime"
                  },
                  "yAxis": [
                    {
                      "name": "Val",
                      "type": "real"
                    }
                  ],
                  "splitBy": [
                    {
                      "name": "TenantId",
                      "type": "string"
                    }
                  ],
                  "aggregation": "Sum"
                }
              },
              {
                "name": "Version",
                "value": "1.0"
              },
              {
                "name": "PartId",
                "value": "116c2a20-111c-4e5b-9620-590b13388ced"
              },
              {
                "name": "PartTitle",
                "value": "Analytics"
              },
              {
                "name": "PartSubTitle",
                "value": "mhra-products-cluster-analytics-gv55"
              },
              {
                "name": "resourceTypeMode",
                "value": "workspace"
              },
              {
                "name": "ControlType",
                "value": "AnalyticsChart"
              },
              {
                "name": "SpecificChart",
                "value": "Line"
              },
              {
                "name": "DashboardId",
                "isOptional": true
              }
            ],
            "type": "Extension/AppInsightsExtension/PartType/AnalyticsPart",
            "settings": {
              "content": {
                "PartTitle": "Request error rate",
                "PartSubTitle": "mhra-products-cluster-analytics-gv55"
              }
            },
            "asset": {
              "idInputName": "ComponentId",
              "type": "ApplicationInsights"
            }
          }
        },
        "1": {
          "position": {
            "x": 6,
            "y": 0,
            "colSpan": 6,
            "rowSpan": 4
          },
          "metadata": {
            "inputs": [
              {
                "name": "ComponentId",
                "value": {
                  "SubscriptionId": "bec11470-1346-4cdd-af2e-ce1f360671a1",
                  "ResourceGroup": "mhra-products-development",
                  "Name": "mhra-products-cluster-analytics-gv55",
                  "ResourceId": "/subscriptions/bec11470-1346-4cdd-af2e-ce1f360671a1/resourcegroups/mhra-products-development/providers/microsoft.operationalinsights/workspaces/mhra-products-cluster-analytics-gv55"
                }
              },
              {
                "name": "Query",
                "value": "InsightsMetrics\n| where Namespace == \"prometheus\"\n| where Name == \"job:doc_index_updater_success_response_latency_milliseconds:mean\"\n| render timechart \n"
              },
              {
                "name": "TimeRange",
                "value": "P1D"
              },
              {
                "name": "Dimensions",
                "value": {
                  "xAxis": {
                    "name": "TimeGenerated",
                    "type": "datetime"
                  },
                  "yAxis": [
                    {
                      "name": "Val",
                      "type": "real"
                    }
                  ],
                  "splitBy": [
                    {
                      "name": "TenantId",
                      "type": "string"
                    }
                  ],
                  "aggregation": "Sum"
                }
              },
              {
                "name": "Version",
                "value": "1.0"
              },
              {
                "name": "PartId",
                "value": "1035153e-0e55-406d-ab4f-64eec62e5be4"
              },
              {
                "name": "PartTitle",
                "value": "Analytics"
              },
              {
                "name": "PartSubTitle",
                "value": "mhra-products-cluster-analytics-gv55"
              },
              {
                "name": "resourceTypeMode",
                "value": "workspace"
              },
              {
                "name": "ControlType",
                "value": "AnalyticsChart"
              },
              {
                "name": "SpecificChart",
                "value": "Line"
              },
              {
                "name": "DashboardId",
                "isOptional": true
              }
            ],
            "type": "Extension/AppInsightsExtension/PartType/AnalyticsPart",
            "settings": {
              "content": {
                "PartTitle": "Request latency milliseconds",
                "PartSubTitle": "mhra-products-cluster-analytics-gv55"
              }
            },
            "asset": {
              "idInputName": "ComponentId",
              "type": "ApplicationInsights"
            }
          }
        }
      }
    }
  },
  "metadata": {
    "model": {
      "timeRange": {
        "value": {
          "relative": {
            "duration": 24,
            "timeUnit": 1
          }
        },
        "type": "MsPortalFx.Composition.Configuration.ValueTypes.TimeRange"
      },
      "filterLocale": {
        "value": "en-us"
      },
      "filters": {
        "value": {
          "MsPortalFx_TimeRange": {
            "model": {
              "format": "utc",
              "granularity": "auto",
              "relative": "24h"
            },
            "displayCache": {
              "name": "UTC Time",
              "value": "Past 24 hours"
            },
            "filteredPartIds": [
              "StartboardPart-AnalyticsPart-1a2ab4b7-0ec6-49ad-88c3-c118ca91303e",
              "StartboardPart-AnalyticsPart-1a2ab4b7-0ec6-49ad-88c3-c118ca913040"
            ]
          }
        }
      }
    }
  }
}
DASH
}
