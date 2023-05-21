# \ModsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v10_mods_get**](ModsApi.md#api_v10_mods_get) | **GET** /api/v1.0/Mods | 
[**api_v10_mods_id_get**](ModsApi.md#api_v10_mods_id_get) | **GET** /api/v1.0/Mods/{id} | 
[**api_v10_mods_id_stats_get**](ModsApi.md#api_v10_mods_id_stats_get) | **GET** /api/v1.0/Mods/{id}/Stats | 
[**api_v10_mods_paged_get**](ModsApi.md#api_v10_mods_paged_get) | **GET** /api/v1.0/Mods/paged | 



## api_v10_mods_get

> Vec<crate::models::ModResponse> api_v10_mods_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ModResponse>**](ModResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v10_mods_id_get

> crate::models::ModResponse api_v10_mods_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::ModResponse**](ModResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v10_mods_id_stats_get

> crate::models::ModStatsResponse api_v10_mods_id_stats_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::ModStatsResponse**](ModStatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v10_mods_paged_get

> crate::models::ModResponsePagedResults api_v10_mods_paged_get(offset, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**crate::models::ModResponsePagedResults**](ModResponsePagedResults.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

