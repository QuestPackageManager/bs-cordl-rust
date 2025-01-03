#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogData")]
#[repr(C)]
#[derive(Debug)]
pub struct ContentCatalogData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub localHash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub location: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    >,
    pub m_LocatorId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_BuildResultHash: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_InstanceProviderData: crate::UnityEngine::ResourceManagement::Util::ObjectInitializationData,
    pub m_SceneProviderData: crate::UnityEngine::ResourceManagement::Util::ObjectInitializationData,
    pub m_ResourceProviderData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::ResourceManagement::Util::ObjectInitializationData,
        >,
    >,
    pub m_Entries: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogDataEntry,
        >,
    >,
    pub m_ProviderIds: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub m_InternalIds: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub m_KeyDataString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_BucketDataString: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_EntryDataString: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_ExtraDataString: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_resourceTypes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::ResourceManagement::Util::SerializedType,
        >,
    >,
    pub m_InternalIdPrefixes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData =>
    "UnityEngine.AddressableAssets.ResourceLocators"."ContentCatalogData"
);
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogData")]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogData")]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogData")]
impl crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData {
    pub const kBytesPerInt32: i32 = 4i32;
    pub const kVersion: i32 = 1i32;
    pub const k_EntryDataItemPerEntry: i32 = 7i32;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogData+Bucket"
    )]
    pub type Bucket = crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData_Bucket;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogData+CompactLocation"
    )]
    pub type CompactLocation = crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData_CompactLocation;
    pub fn CleanData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCustomLocator(
        &mut self,
        overrideId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        providerSuffix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap,
        > = __cordl_object.invoke("CreateCustomLocator", (overrideId, providerSuffix))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateLocator(
        &mut self,
        providerSuffix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap,
        > = __cordl_object.invoke("CreateLocator", (providerSuffix))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpandInternalId(
        internalIdPrefixes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        v: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExpandInternalId", (internalIdPrefixes, v))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogDataEntry,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogDataEntry,
            >,
        > = __cordl_object.invoke("GetData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFromFile(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cacheSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadFromFile", (path, cacheSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_1() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString0(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (id))?;
        Ok(__cordl_object.into())
    }
    pub fn SaveToFile(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveToFile", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString0(
        &mut self,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InstanceProviderData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::Util::ObjectInitializationData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::Util::ObjectInitializationData = __cordl_object
            .invoke("get_InstanceProviderData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_InternalIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ProviderId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ProviderId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ProviderIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_ProviderIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ResourceProviderData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ResourceManagement::Util::ObjectInitializationData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ResourceManagement::Util::ObjectInitializationData,
            >,
        > = __cordl_object.invoke("get_ResourceProviderData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SceneProviderData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::Util::ObjectInitializationData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::Util::ObjectInitializationData = __cordl_object
            .invoke("get_SceneProviderData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_InstanceProviderData(
        &mut self,
        value: crate::UnityEngine::ResourceManagement::Util::ObjectInitializationData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InstanceProviderData", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ProviderId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ProviderId", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ResourceProviderData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ResourceManagement::Util::ObjectInitializationData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ResourceProviderData", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SceneProviderData(
        &mut self,
        value: crate::UnityEngine::ResourceManagement::Util::ObjectInitializationData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SceneProviderData", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogData")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogData+Bucket"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ContentCatalogData_Bucket {
    pub dataOffset: i32,
    pub entries: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogData+Bucket"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData_Bucket =>
    "UnityEngine.AddressableAssets.ResourceLocators"."ContentCatalogData/Bucket"
);
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogData+Bucket"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData_Bucket {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogData+Bucket"
)]
impl crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData_Bucket {}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogData+CompactLocation"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ContentCatalogData_CompactLocation {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Locator: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap,
    >,
    pub m_InternalId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_ProviderId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Dependency: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_Data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_HashCode: i32,
    pub m_DependencyHashCode: i32,
    pub m_PrimaryKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Type: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogData+CompactLocation"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData_CompactLocation
    => "UnityEngine.AddressableAssets.ResourceLocators"
    ."ContentCatalogData/CompactLocation"
);
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogData+CompactLocation"
)]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData_CompactLocation {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogData+CompactLocation"
)]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData_CompactLocation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogData+CompactLocation"
)]
impl crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData_CompactLocation {
    pub fn Hash(
        &mut self,
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Hash", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        locator: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap,
        >,
        internalId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        providerId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dependencyKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        depHash: i32,
        primaryKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    locator,
                    internalId,
                    providerId,
                    dependencyKey,
                    data,
                    depHash,
                    primaryKey,
                    _cordl_type,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        locator: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap,
        >,
        internalId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        providerId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dependencyKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        depHash: i32,
        primaryKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    locator,
                    internalId,
                    providerId,
                    dependencyKey,
                    data,
                    depHash,
                    primaryKey,
                    _cordl_type,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Data(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_Data", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Dependencies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            >,
        > = __cordl_object.invoke("get_Dependencies", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DependencyHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_DependencyHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasDependencies(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasDependencies", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_InternalId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PrimaryKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_PrimaryKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ProviderId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ProviderId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ResourceType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_ResourceType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PrimaryKey(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PrimaryKey", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogData+CompactLocation"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData_CompactLocation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogData+CompactLocation"
)]
impl AsRef<crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation>
for crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData_CompactLocation {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceLocators+ContentCatalogData+CompactLocation"
)]
impl AsMut<crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation>
for crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData_CompactLocation {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation {
        unsafe { std::mem::transmute(self) }
    }
}
