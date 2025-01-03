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
    pub fn Create(
        obj: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::DownloadHandlerAssetBundle,
        >,
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        crc: u32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (obj, url, crc))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCached(
        obj: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::DownloadHandlerAssetBundle,
        >,
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hash: crate::UnityEngine::Hash128,
        crc: u32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateCached", (obj, url, name, hash, crc))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCached_Injected(
        obj: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::DownloadHandlerAssetBundle,
        >,
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hash: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Hash128>,
        crc: u32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateCached_Injected", (obj, url, name, hash, crc))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetText", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalCreateAssetBundle(
        &mut self,
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        crc: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalCreateAssetBundle", (url, crc))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalCreateAssetBundleCached(
        &mut self,
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hash: crate::UnityEngine::Hash128,
        crc: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalCreateAssetBundleCached", (url, name, hash, crc))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_CachedAssetBundle_u32_1(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cachedBundle: crate::UnityEngine::CachedAssetBundle,
        crc: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (url, cachedBundle, crc))?;
        Ok(__cordl_object.into())
    }
    pub fn New_u32_0(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        crc: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (url, crc))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_CachedAssetBundle_u32_1(
        &mut self,
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cachedBundle: crate::UnityEngine::CachedAssetBundle,
        crc: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (url, cachedBundle, crc))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_0(
        &mut self,
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        crc: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (url, crc))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_assetBundle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle> = __cordl_object
            .invoke("get_assetBundle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_autoLoadAssetBundle(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_autoLoadAssetBundle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isDownloadComplete(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDownloadComplete", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
