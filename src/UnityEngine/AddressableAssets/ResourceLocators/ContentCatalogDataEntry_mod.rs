#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogDataEntry"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ContentCatalogDataEntry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _InternalId_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _Provider_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _Keys_k__BackingField: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _Dependencies_k__BackingField: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _Data_k__BackingField: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _ResourceType_k__BackingField: *mut crate::System::Type,
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogDataEntry"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogDataEntry =>
    "UnityEngine.AddressableAssets.ResourceLocators"."ContentCatalogDataEntry"
);
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogDataEntry"
)]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogDataEntry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogDataEntry"
)]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogDataEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogDataEntry"
)]
impl crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogDataEntry {
    pub fn New(
        _cordl_type: *mut crate::System::Type,
        internalId: *mut quest_hook::libil2cpp::Il2CppString,
        provider: *mut quest_hook::libil2cpp::Il2CppString,
        keys: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        dependencies: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        extraData: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_type, internalId, provider, keys, dependencies, extraData),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        internalId: *mut quest_hook::libil2cpp::Il2CppString,
        provider: *mut quest_hook::libil2cpp::Il2CppString,
        keys: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        dependencies: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        extraData: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (_cordl_type, internalId, provider, keys, dependencies, extraData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_Data(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("get_Data", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Dependencies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_Dependencies", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InternalId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_InternalId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Keys(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_Keys", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Provider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Provider", ())?;
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
    pub fn set_Data(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Data", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Dependencies(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Dependencies", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_InternalId(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InternalId", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Keys(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Keys", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Provider(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Provider", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ResourceType(
        &mut self,
        value: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ResourceType", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogDataEntry"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogDataEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
