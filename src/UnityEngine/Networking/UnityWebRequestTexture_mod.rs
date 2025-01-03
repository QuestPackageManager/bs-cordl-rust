#[cfg(feature = "UnityEngine+Networking+UnityWebRequestTexture")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityWebRequestTexture {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestTexture")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Networking::UnityWebRequestTexture
    => "UnityEngine.Networking"."UnityWebRequestTexture"
);
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestTexture")]
impl std::ops::Deref for crate::UnityEngine::Networking::UnityWebRequestTexture {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestTexture")]
impl std::ops::DerefMut for crate::UnityEngine::Networking::UnityWebRequestTexture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestTexture")]
impl crate::UnityEngine::Networking::UnityWebRequestTexture {
    pub fn GetTexture_Il2CppString0(
        uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Networking::UnityWebRequest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequest,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetTexture", (uri))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTexture__cordl_bool1(
        uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nonReadable: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Networking::UnityWebRequest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTexture", (uri, nonReadable))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestTexture")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::UnityWebRequestTexture {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
