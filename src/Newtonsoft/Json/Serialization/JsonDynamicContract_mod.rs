#[cfg(feature = "Newtonsoft+Json+Serialization+JsonDynamicContract")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonDynamicContract {
    __cordl_parent: crate::Newtonsoft::Json::Serialization::JsonContainerContract,
    pub _Properties_k__BackingField: *mut crate::Newtonsoft::Json::Serialization::JsonPropertyCollection,
    pub _PropertyNameResolver_k__BackingField: *mut crate::System::Func_2<
        *mut crate::System::String,
        *mut crate::System::String,
    >,
    pub _callSiteGetters: *mut crate::Newtonsoft::Json::Utilities::ThreadSafeStore_2<
        *mut crate::System::String,
        *mut crate::System::Runtime::CompilerServices::CallSite_1<
            *mut crate::System::Func_3<
                *mut crate::System::Runtime::CompilerServices::CallSite,
                *mut crate::System::Object,
                *mut crate::System::Object,
            >,
        >,
    >,
    pub _callSiteSetters: *mut crate::Newtonsoft::Json::Utilities::ThreadSafeStore_2<
        *mut crate::System::String,
        *mut crate::System::Runtime::CompilerServices::CallSite_1<
            *mut crate::System::Func_4<
                *mut crate::System::Runtime::CompilerServices::CallSite,
                *mut crate::System::Object,
                *mut crate::System::Object,
                *mut crate::System::Object,
            >,
        >,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonDynamicContract")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::JsonDynamicContract =>
    "Newtonsoft.Json.Serialization"."JsonDynamicContract"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonDynamicContract")]
impl std::ops::Deref for crate::Newtonsoft::Json::Serialization::JsonDynamicContract {
    type Target = crate::Newtonsoft::Json::Serialization::JsonContainerContract;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonDynamicContract")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Serialization::JsonDynamicContract {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonDynamicContract")]
impl crate::Newtonsoft::Json::Serialization::JsonDynamicContract {
    pub fn New(
        underlyingType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (underlyingType))?;
        Ok(__cordl_object)
    }
    pub fn TryGetMember(
        &mut self,
        dynamicProvider: *mut crate::System::Dynamic::IDynamicMetaObjectProvider,
        name: *mut crate::System::String,
        value: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetMember", (dynamicProvider, name, value))?;
        Ok(__cordl_ret)
    }
    pub fn TrySetMember(
        &mut self,
        dynamicProvider: *mut crate::System::Dynamic::IDynamicMetaObjectProvider,
        name: *mut crate::System::String,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TrySetMember", (dynamicProvider, name, value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        underlyingType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (underlyingType))?;
        Ok(__cordl_ret)
    }
    pub fn get_Properties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::JsonPropertyCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::JsonPropertyCollection = __cordl_object
            .invoke("get_Properties", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PropertyNameResolver(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Func_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Func_2<
            *mut crate::System::String,
            *mut crate::System::String,
        > = __cordl_object.invoke("get_PropertyNameResolver", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_PropertyNameResolver(
        &mut self,
        value: *mut crate::System::Func_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PropertyNameResolver", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonDynamicContract")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonDynamicContract {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
