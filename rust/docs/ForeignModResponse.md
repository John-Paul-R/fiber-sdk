# ForeignModResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**platform** | Option<[**crate::models::ForeignPlatform**](ForeignPlatform.md)> |  | [optional]
**platform_mod_id** | Option<**String**> |  | [optional]
**fiber_mod_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**platform_mod_slug** | **String** |  | 
**name** | **String** |  | 
**summary** | **String** |  | 
**download_count** | **i32** |  | 
**date_released** | Option<**String**> |  | [optional]
**date_modified** | **String** |  | 
**deleted_auto** | Option<**bool**> |  | [optional]
**deleted_manual** | Option<**bool**> |  | [optional]
**authors** | [**Vec<crate::models::ForeignAuthorResponse>**](ForeignAuthorResponse.md) |  | 
**minecraft_versions** | **Vec<String>** |  | 
**fabric_versions** | Option<**Vec<String>**> |  | [optional]
**categories** | **Vec<String>** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


