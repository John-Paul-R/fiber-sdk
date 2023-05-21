# \ForeignModsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v10_foreign_mods_get**](ForeignModsApi.md#api_v10_foreign_mods_get) | **GET** /api/v1.0/ForeignMods | 
[**api_v10_foreign_mods_id_get**](ForeignModsApi.md#api_v10_foreign_mods_id_get) | **GET** /api/v1.0/ForeignMods/{id} | 



## api_v10_foreign_mods_get

> Vec<crate::models::ForeignModResponse> api_v10_foreign_mods_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ForeignModResponse>**](ForeignModResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v10_foreign_mods_id_get

> crate::models::ForeignModResponse api_v10_foreign_mods_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::ForeignModResponse**](ForeignModResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

