#[cfg(feature = "UnityEngine+Networking+DownloadHandlerAssetBundle")]
#[repr(C)]
#[derive(Debug)]
pub struct DownloadHandlerAssetBundle {
    __cordl_parent: crate::UnityEngine::Networking::DownloadHandler,
}
#[cfg(feature = "UnityEngine+Networking+DownloadHandlerAssetBundle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Networking::DownloadHandlerAssetBundle => "UnityEngine.Networking"
    ."DownloadHandlerAssetBundle"
);
#[cfg(feature = "UnityEngine+Networking+DownloadHandlerAssetBundle")]
impl std::ops::Deref for crate::UnityEngine::Networking::DownloadHandlerAssetBundle {
    type Target = crate::UnityEngine::Networking::DownloadHandler;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+DownloadHandlerAssetBundle")]
impl std::ops::DerefMut for crate::UnityEngine::Networking::DownloadHandlerAssetBundle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+DownloadHandlerAssetBundle")]
impl crate::UnityEngine::Networking::DownloadHandlerAssetBundle {
    pub fn GetData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetData", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetText", ())?;
        Ok(__cordl_ret)
    }
    pub fn InternalCreateAssetBundle(
        &mut self,
        url: *mut crate::System::String,
        crc: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalCreateAssetBundle", (url, crc))?;
        Ok(__cordl_ret)
    }
    pub fn InternalCreateAssetBundleCached(
        &mut self,
        url: *mut crate::System::String,
        name: *mut crate::System::String,
        hash: crate::UnityEngine::Hash128,
        crc: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalCreateAssetBundleCached", (url, name, hash, crc))?;
        Ok(__cordl_ret)
    }
    pub fn New_CachedAssetBundle_u32_1(
        url: *mut crate::System::String,
        cachedBundle: crate::UnityEngine::CachedAssetBundle,
        crc: u32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (url, cachedBundle, crc))?;
        Ok(__cordl_object)
    }
    pub fn New_u32_0(
        url: *mut crate::System::String,
        crc: u32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (url, crc))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_CachedAssetBundle_u32_1(
        &mut self,
        url: *mut crate::System::String,
        cachedBundle: crate::UnityEngine::CachedAssetBundle,
        crc: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (url, cachedBundle, crc))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_u32_0(
        &mut self,
        url: *mut crate::System::String,
        crc: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (url, crc))?;
        Ok(__cordl_ret)
    }
    pub fn get_assetBundle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AssetBundle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AssetBundle = __cordl_object
            .invoke("get_assetBundle", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_autoLoadAssetBundle(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_autoLoadAssetBundle", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isDownloadComplete(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDownloadComplete", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_autoLoadAssetBundle(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_autoLoadAssetBundle", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Networking+DownloadHandlerAssetBundle")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::DownloadHandlerAssetBundle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
