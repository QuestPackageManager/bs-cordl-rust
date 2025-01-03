#[cfg(feature = "Newtonsoft+Json+Serialization+JsonDynamicContract")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonDynamicContract {
    __cordl_parent: crate::Newtonsoft::Json::Serialization::JsonContainerContract,
    pub _Properties_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::JsonPropertyCollection,
    >,
    pub _PropertyNameResolver_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub _callSiteGetters: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Utilities::ThreadSafeStore_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::System::Runtime::CompilerServices::CallSite_1<
                *mut crate::System::Func_3<
                    *mut crate::System::Runtime::CompilerServices::CallSite,
                    *mut quest_hook::libil2cpp::Il2CppObject,
                    *mut quest_hook::libil2cpp::Il2CppObject,
                >,
            >,
        >,
    >,
    pub _callSiteSetters: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Utilities::ThreadSafeStore_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::System::Runtime::CompilerServices::CallSite_1<
                *mut crate::System::Func_4<
                    *mut crate::System::Runtime::CompilerServices::CallSite,
                    *mut quest_hook::libil2cpp::Il2CppObject,
                    *mut quest_hook::libil2cpp::Il2CppObject,
                    *mut quest_hook::libil2cpp::Il2CppObject,
                >,
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
    pub fn CreateCallSiteGetter(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSite_1<
                *mut crate::System::Func_3<
                    *mut crate::System::Runtime::CompilerServices::CallSite,
                    *mut quest_hook::libil2cpp::Il2CppObject,
                    *mut quest_hook::libil2cpp::Il2CppObject,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSite_1<
                *mut crate::System::Func_3<
                    *mut crate::System::Runtime::CompilerServices::CallSite,
                    *mut quest_hook::libil2cpp::Il2CppObject,
                    *mut quest_hook::libil2cpp::Il2CppObject,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateCallSiteGetter", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCallSiteSetter(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSite_1<
                *mut crate::System::Func_4<
                    *mut crate::System::Runtime::CompilerServices::CallSite,
                    *mut quest_hook::libil2cpp::Il2CppObject,
                    *mut quest_hook::libil2cpp::Il2CppObject,
                    *mut quest_hook::libil2cpp::Il2CppObject,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSite_1<
                *mut crate::System::Func_4<
                    *mut crate::System::Runtime::CompilerServices::CallSite,
                    *mut quest_hook::libil2cpp::Il2CppObject,
                    *mut quest_hook::libil2cpp::Il2CppObject,
                    *mut quest_hook::libil2cpp::Il2CppObject,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateCallSiteSetter", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        underlyingType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (underlyingType))?;
        Ok(__cordl_object.into())
    }
    pub fn TryGetMember(
        &mut self,
        dynamicProvider: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::IDynamicMetaObjectProvider,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetMember", (dynamicProvider, name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrySetMember(
        &mut self,
        dynamicProvider: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::IDynamicMetaObjectProvider,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TrySetMember", (dynamicProvider, name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        underlyingType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (underlyingType))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Properties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonPropertyCollection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonPropertyCollection,
        > = __cordl_object.invoke("get_Properties", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PropertyNameResolver(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object.invoke("get_PropertyNameResolver", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PropertyNameResolver(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PropertyNameResolver", (value))?;
        Ok(__cordl_ret.into())
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
