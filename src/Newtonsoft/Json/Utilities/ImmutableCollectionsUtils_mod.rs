#[cfg(
    feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils+ImmutableCollectionTypeInfo"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ImmutableCollectionsUtils_ImmutableCollectionTypeInfo {
    __cordl_parent: crate::System::Object,
    pub _ContractTypeName_k__BackingField: *mut crate::System::String,
    pub _CreatedTypeName_k__BackingField: *mut crate::System::String,
    pub _BuilderTypeName_k__BackingField: *mut crate::System::String,
}
#[cfg(
    feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils+ImmutableCollectionTypeInfo"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils_ImmutableCollectionTypeInfo
    => "Newtonsoft.Json.Utilities"
    ."ImmutableCollectionsUtils/ImmutableCollectionTypeInfo"
);
#[cfg(
    feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils+ImmutableCollectionTypeInfo"
)]
impl std::ops::Deref
for crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils_ImmutableCollectionTypeInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils+ImmutableCollectionTypeInfo"
)]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils_ImmutableCollectionTypeInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils+ImmutableCollectionTypeInfo"
)]
impl crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils_ImmutableCollectionTypeInfo {
    pub fn get_CreatedTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_CreatedTypeName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_BuilderTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_BuilderTypeName", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_BuilderTypeName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BuilderTypeName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        contractTypeName: *mut crate::System::String,
        createdTypeName: *mut crate::System::String,
        builderTypeName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (contractTypeName, createdTypeName, builderTypeName))?;
        Ok(__cordl_ret)
    }
    pub fn set_ContractTypeName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ContractTypeName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_ContractTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ContractTypeName", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_CreatedTypeName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CreatedTypeName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        contractTypeName: *mut crate::System::String,
        createdTypeName: *mut crate::System::String,
        builderTypeName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (contractTypeName, createdTypeName, builderTypeName))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils+ImmutableCollectionTypeInfo"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils_ImmutableCollectionTypeInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct ImmutableCollectionsUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils =>
    "Newtonsoft.Json.Utilities"."ImmutableCollectionsUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils")]
impl crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils {
    pub const ImmutableArrayGenericTypeName: &'static str = "System.Collections.Immutable.ImmutableArray`1";
    pub const ImmutableArrayTypeName: &'static str = "System.Collections.Immutable.ImmutableArray";
    pub const ImmutableDictionaryGenericInterfaceTypeName: &'static str = "System.Collections.Immutable.IImmutableDictionary`2";
    pub const ImmutableDictionaryGenericTypeName: &'static str = "System.Collections.Immutable.ImmutableDictionary`2";
    pub const ImmutableDictionaryTypeName: &'static str = "System.Collections.Immutable.ImmutableDictionary";
    pub const ImmutableHashSetGenericTypeName: &'static str = "System.Collections.Immutable.ImmutableHashSet`1";
    pub const ImmutableHashSetTypeName: &'static str = "System.Collections.Immutable.ImmutableHashSet";
    pub const ImmutableListGenericInterfaceTypeName: &'static str = "System.Collections.Immutable.IImmutableList`1";
    pub const ImmutableListGenericTypeName: &'static str = "System.Collections.Immutable.ImmutableList`1";
    pub const ImmutableListTypeName: &'static str = "System.Collections.Immutable.ImmutableList";
    pub const ImmutableQueueGenericInterfaceTypeName: &'static str = "System.Collections.Immutable.IImmutableQueue`1";
    pub const ImmutableQueueGenericTypeName: &'static str = "System.Collections.Immutable.ImmutableQueue`1";
    pub const ImmutableQueueTypeName: &'static str = "System.Collections.Immutable.ImmutableQueue";
    pub const ImmutableSetGenericInterfaceTypeName: &'static str = "System.Collections.Immutable.IImmutableSet`1";
    pub const ImmutableSortedDictionaryGenericTypeName: &'static str = "System.Collections.Immutable.ImmutableSortedDictionary`2";
    pub const ImmutableSortedDictionaryTypeName: &'static str = "System.Collections.Immutable.ImmutableSortedDictionary";
    pub const ImmutableSortedSetGenericTypeName: &'static str = "System.Collections.Immutable.ImmutableSortedSet`1";
    pub const ImmutableSortedSetTypeName: &'static str = "System.Collections.Immutable.ImmutableSortedSet";
    pub const ImmutableStackGenericInterfaceTypeName: &'static str = "System.Collections.Immutable.IImmutableStack`1";
    pub const ImmutableStackGenericTypeName: &'static str = "System.Collections.Immutable.ImmutableStack`1";
    pub const ImmutableStackTypeName: &'static str = "System.Collections.Immutable.ImmutableStack";
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils+__c__DisplayClass25_0"
    )]
    pub type __c__DisplayClass25_0 = crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils___c__DisplayClass25_0;
    #[cfg(feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils+__c")]
    pub type __c = crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils___c;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils+ImmutableCollectionTypeInfo"
    )]
    pub type ImmutableCollectionTypeInfo = crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils_ImmutableCollectionTypeInfo;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils+__c__DisplayClass24_0"
    )]
    pub type __c__DisplayClass24_0 = crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils___c__DisplayClass24_0;
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
