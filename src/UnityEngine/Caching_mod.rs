#[cfg(feature = "UnityEngine+Caching")]
#[repr(C)]
#[derive(Debug)]
pub struct Caching {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Caching")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Caching => "UnityEngine"."Caching"
);
#[cfg(feature = "UnityEngine+Caching")]
impl std::ops::Deref for crate::UnityEngine::Caching {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Caching")]
impl std::ops::DerefMut for crate::UnityEngine::Caching {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Caching")]
impl crate::UnityEngine::Caching {
    pub fn AddCache_Gc0(
        cachePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Cache> {
        let __cordl_ret: crate::UnityEngine::Cache = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddCache", (cachePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddCache_Injected(
        cachePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isReadonly: bool,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Cache>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddCache_Injected", (cachePath, isReadonly, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddCache__cordl_bool1(
        cachePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isReadonly: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Cache> {
        let __cordl_ret: crate::UnityEngine::Cache = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddCache", (cachePath, isReadonly))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearAllCachedVersions(
        assetBundleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearAllCachedVersions", (assetBundleName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearCachedVersion(
        assetBundleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hash: crate::UnityEngine::Hash128,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearCachedVersion", (assetBundleName, hash))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearCachedVersionInternal(
        assetBundleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hash: crate::UnityEngine::Hash128,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearCachedVersionInternal", (assetBundleName, hash))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearCachedVersionInternal_Injected(
        assetBundleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hash: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Hash128>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearCachedVersionInternal_Injected", (assetBundleName, hash))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearCachedVersions(
        assetBundleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hash: crate::UnityEngine::Hash128,
        keepInputVersion: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearCachedVersions", (assetBundleName, hash, keepInputVersion))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearCachedVersions_Injected(
        assetBundleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hash: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Hash128>,
        keepInputVersion: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ClearCachedVersions_Injected",
                (assetBundleName, hash, keepInputVersion),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearOtherCachedVersions(
        assetBundleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hash: crate::UnityEngine::Hash128,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearOtherCachedVersions", (assetBundleName, hash))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCacheByPath(
        cachePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Cache> {
        let __cordl_ret: crate::UnityEngine::Cache = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCacheByPath", (cachePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCacheByPath_Injected(
        cachePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Cache>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCacheByPath_Injected", (cachePath, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsVersionCached_CachedAssetBundle0(
        cachedBundle: crate::UnityEngine::CachedAssetBundle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsVersionCached", (cachedBundle))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsVersionCached_Gc_Gc_Hash128_1(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assetBundleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hash: crate::UnityEngine::Hash128,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsVersionCached", (url, assetBundleName, hash))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsVersionCached_Injected(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assetBundleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hash: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Hash128>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsVersionCached_Injected", (url, assetBundleName, hash))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentCacheForWriting() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Cache,
    > {
        let __cordl_ret: crate::UnityEngine::Cache = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_currentCacheForWriting", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentCacheForWriting_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Cache>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_currentCacheForWriting_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultCache() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Cache,
    > {
        let __cordl_ret: crate::UnityEngine::Cache = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultCache", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultCache_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Cache>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultCache_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ready() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ready", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_compressionEnabled(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_compressionEnabled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_currentCacheForWriting(
        value: crate::UnityEngine::Cache,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_currentCacheForWriting", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_currentCacheForWriting_Injected(
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Cache>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_currentCacheForWriting_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Caching")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Caching {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
