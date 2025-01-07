#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocatorInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceLocatorInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Locator_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
    >,
    pub _LocalHash_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _CatalogLocation_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    >,
    pub _ContentUpdateAvailable_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocatorInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::AddressableAssets::ResourceLocatorInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.AddressableAssets";
    const CLASS_NAME: &'static str = "ResourceLocatorInfo";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocatorInfo")]
impl std::ops::Deref for crate::UnityEngine::AddressableAssets::ResourceLocatorInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocatorInfo")]
impl std::ops::DerefMut for crate::UnityEngine::AddressableAssets::ResourceLocatorInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocatorInfo")]
impl crate::UnityEngine::AddressableAssets::ResourceLocatorInfo {
    pub fn New(
        loc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
        localHash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        remoteCatalogLocation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (loc, localHash, remoteCatalogLocation))?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateContent(
        &mut self,
        locator: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
        hash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateContent", (locator, hash, loc))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        loc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
        localHash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        remoteCatalogLocation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (loc, localHash, remoteCatalogLocation))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CanUpdateContent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanUpdateContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CatalogLocation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        > = __cordl_object.invoke("get_CatalogLocation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ContentUpdateAvailable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ContentUpdateAvailable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HashLocation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        > = __cordl_object.invoke("get_HashLocation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LocalHash(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_LocalHash", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Locator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        > = __cordl_object.invoke("get_Locator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CatalogLocation(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CatalogLocation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ContentUpdateAvailable(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ContentUpdateAvailable", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LocalHash(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LocalHash", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Locator(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Locator", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocatorInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::ResourceLocatorInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
