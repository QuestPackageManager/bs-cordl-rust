#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+ResourceLocationMap")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceLocationMap {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _LocatorId_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub locations: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                    >,
                >,
            >,
        >,
    >,
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+ResourceLocationMap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap =>
    "UnityEngine.AddressableAssets.ResourceLocators"."ResourceLocationMap"
);
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+ResourceLocationMap")]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+ResourceLocationMap")]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+ResourceLocationMap")]
impl crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap {
    pub fn Add_IList_1_1(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        locations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (key, locations))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_IResourceLocation0(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (key, location))?;
        Ok(__cordl_ret.into())
    }
    pub fn Locate(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        locations: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Locate", (key, _cordl_type, locations))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_IList_1_1(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        locations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationData,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (id, locations))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_0(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (id, capacity))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_IList_1_1(
        &mut self,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        locations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationData,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (id, locations))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_0(
        &mut self,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (id, capacity))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Keys(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = __cordl_object.invoke("get_Keys", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Locations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IList_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                        >,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IList_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                        >,
                    >,
                >,
            >,
        > = __cordl_object.invoke("get_Locations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LocatorId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_LocatorId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LocatorId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LocatorId", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+ResourceLocationMap")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+ResourceLocationMap")]
impl AsRef<crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator>
for crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+ResourceLocationMap")]
impl AsMut<crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator>
for crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator {
        unsafe { std::mem::transmute(self) }
    }
}
