#[cfg(feature = "cordl_class_UnityEditor+Analytics+AssetDatabaseRefreshAnalytic")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct AssetDatabaseRefreshAnalytic {
    __cordl_parent: crate::UnityEngine::Analytics::AnalyticsEventBase,
    pub isV2: bool,
    pub Imports_Imported: i64,
    pub Imports_ImportedInProcess: i64,
    pub Imports_ImportedOutOfProcess: i64,
    pub Imports_Refresh: i64,
    pub Imports_DomainReload: i64,
    pub CacheServer_MetadataRequested: i64,
    pub CacheServer_MetadataDownloaded: i64,
    pub CacheServer_MetadataFailedToDownload: i64,
    pub CacheServer_MetadataUploaded: i64,
    pub CacheServer_ArtifactsFailedToUpload: i64,
    pub CacheServer_MetadataVersionsDownloaded: i64,
    pub CacheServer_MetadataMatched: i64,
    pub CacheServer_ArtifactsDownloaded: i64,
    pub CacheServer_ArtifactFilesDownloaded: i64,
    pub CacheServer_ArtifactFilesFailedToDownload: i64,
    pub CacheServer_ArtifactsUploaded: i64,
    pub CacheServer_ArtifactFilesUploaded: i64,
    pub CacheServer_ArtifactFilesFailedToUpload: i64,
    pub CacheServer_Connects: i64,
    pub CacheServer_Disconnects: i64,
}
#[cfg(feature = "cordl_class_UnityEditor+Analytics+AssetDatabaseRefreshAnalytic")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEditor::Analytics::AssetDatabaseRefreshAnalytic
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEditor.Analytics";
    const CLASS_NAME: &'static str = "AssetDatabaseRefreshAnalytic";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEditor+Analytics+AssetDatabaseRefreshAnalytic")]
impl std::ops::Deref for crate::UnityEditor::Analytics::AssetDatabaseRefreshAnalytic {
    type Target = crate::UnityEngine::Analytics::AnalyticsEventBase;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEditor+Analytics+AssetDatabaseRefreshAnalytic")]
impl std::ops::DerefMut for crate::UnityEditor::Analytics::AssetDatabaseRefreshAnalytic {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEditor+Analytics+AssetDatabaseRefreshAnalytic")]
impl crate::UnityEditor::Analytics::AssetDatabaseRefreshAnalytic {
    pub fn CreateAssetDatabaseRefreshAnalytic() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEditor::Analytics::AssetDatabaseRefreshAnalytic>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEditor::Analytics::AssetDatabaseRefreshAnalytic,
                    >, 0usize>("CreateAssetDatabaseRefreshAnalytic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateAssetDatabaseRefreshAnalytic",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEditor::Analytics::AssetDatabaseRefreshAnalytic,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEditor+Analytics+AssetDatabaseRefreshAnalytic")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEditor::Analytics::AssetDatabaseRefreshAnalytic
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
