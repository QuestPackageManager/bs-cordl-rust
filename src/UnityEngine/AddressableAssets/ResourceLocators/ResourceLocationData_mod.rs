#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+ResourceLocationData")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceLocationData {
    __cordl_parent: crate::System::Object,
    pub m_Keys: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub m_InternalId: *mut crate::System::String,
    pub m_Provider: *mut crate::System::String,
    pub m_Dependencies: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub m_ResourceType: crate::UnityEngine::ResourceManagement::Util::SerializedType,
    pub SerializedData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _Data: *mut crate::System::Object,
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+ResourceLocationData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationData =>
    "UnityEngine.AddressableAssets.ResourceLocators"."ResourceLocationData"
);
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+ResourceLocationData")]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+ResourceLocationData")]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+ResourceLocationData")]
impl crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationData {
    pub fn _ctor(
        &mut self,
        keys: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        id: *mut crate::System::String,
        provider: *mut crate::System::Type,
        t: *mut crate::System::Type,
        dependencies: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keys, id, provider, t, dependencies))?;
        Ok(__cordl_ret)
    }
    pub fn get_Keys(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_Keys", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Provider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Provider", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InternalId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_InternalId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Data(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_Data", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Data(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Data", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Dependencies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_Dependencies", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ResourceType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_ResourceType", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        keys: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        id: *mut crate::System::String,
        provider: *mut crate::System::Type,
        t: *mut crate::System::Type,
        dependencies: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (keys, id, provider, t, dependencies))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+ResourceLocationData")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
