#[cfg(feature = "UnityEngine+Networking+UnityWebRequestAssetBundle")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityWebRequestAssetBundle {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestAssetBundle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Networking::UnityWebRequestAssetBundle => "UnityEngine.Networking"
    ."UnityWebRequestAssetBundle"
);
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestAssetBundle")]
impl std::ops::Deref for crate::UnityEngine::Networking::UnityWebRequestAssetBundle {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestAssetBundle")]
impl std::ops::DerefMut for crate::UnityEngine::Networking::UnityWebRequestAssetBundle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestAssetBundle")]
impl crate::UnityEngine::Networking::UnityWebRequestAssetBundle {
    pub fn GetAssetBundle_Il2CppString0(
        uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Networking::UnityWebRequest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssetBundle", (uri))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetBundle_Il2CppString_u32_2(
        uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        crc: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Networking::UnityWebRequest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssetBundle", (uri, crc))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetBundle_Uri1(
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Networking::UnityWebRequest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssetBundle", (uri))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetBundle_Uri_CachedAssetBundle_u32_4(
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        cachedAssetBundle: crate::UnityEngine::CachedAssetBundle,
        crc: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Networking::UnityWebRequest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssetBundle", (uri, cachedAssetBundle, crc))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetBundle_Uri_u32_3(
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        crc: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Networking::UnityWebRequest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssetBundle", (uri, crc))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestAssetBundle")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::UnityWebRequestAssetBundle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
