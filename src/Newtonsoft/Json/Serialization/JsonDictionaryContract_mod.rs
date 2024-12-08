#[cfg(feature = "Newtonsoft+Json+Serialization+JsonDictionaryContract")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonDictionaryContract {
    __cordl_parent: crate::Newtonsoft::Json::Serialization::JsonContainerContract,
    pub _DictionaryKeyResolver_k__BackingField: *mut crate::System::Func_2<
        *mut crate::System::String,
        *mut crate::System::String,
    >,
    pub _DictionaryKeyType_k__BackingField: *mut crate::System::Type,
    pub _DictionaryValueType_k__BackingField: *mut crate::System::Type,
    pub _KeyContract_k__BackingField: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
    pub _genericCollectionDefinitionType: *mut crate::System::Type,
    pub _genericWrapperType: *mut crate::System::Type,
    pub _genericWrapperCreator: *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
        *mut crate::System::Object,
    >,
    pub _genericTemporaryDictionaryCreator: *mut crate::System::Func_1<
        *mut crate::System::Object,
    >,
    pub _ShouldCreateWrapper_k__BackingField: bool,
    pub _parameterizedConstructor: *mut crate::System::Reflection::ConstructorInfo,
    pub _overrideCreator: *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
        *mut crate::System::Object,
    >,
    pub _parameterizedCreator: *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
        *mut crate::System::Object,
    >,
    pub _HasParameterizedCreator_k__BackingField: bool,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonDictionaryContract")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::JsonDictionaryContract =>
    "Newtonsoft.Json.Serialization"."JsonDictionaryContract"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonDictionaryContract")]
impl std::ops::Deref for crate::Newtonsoft::Json::Serialization::JsonDictionaryContract {
    type Target = crate::Newtonsoft::Json::Serialization::JsonContainerContract;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonDictionaryContract")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::JsonDictionaryContract {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonDictionaryContract")]
impl crate::Newtonsoft::Json::Serialization::JsonDictionaryContract {
    pub fn CreateTemporaryDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IDictionary> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IDictionary = __cordl_object
            .invoke("CreateTemporaryDictionary", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateWrapper(
        &mut self,
        dictionary: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Utilities::IWrappedDictionary,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Utilities::IWrappedDictionary = __cordl_object
            .invoke("CreateWrapper", (dictionary))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        underlyingType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (underlyingType))?;
        Ok(__cordl_object)
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
    pub fn get_DictionaryKeyResolver(
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
        > = __cordl_object.invoke("get_DictionaryKeyResolver", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DictionaryKeyType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_DictionaryKeyType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DictionaryValueType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_DictionaryValueType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasParameterizedCreator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_HasParameterizedCreator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasParameterizedCreatorInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_HasParameterizedCreatorInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_KeyContract(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::JsonContract,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::JsonContract = __cordl_object
            .invoke("get_KeyContract", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_OverrideCreator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            *mut crate::System::Object,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            *mut crate::System::Object,
        > = __cordl_object.invoke("get_OverrideCreator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ParameterizedCreator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            *mut crate::System::Object,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            *mut crate::System::Object,
        > = __cordl_object.invoke("get_ParameterizedCreator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ShouldCreateWrapper(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ShouldCreateWrapper", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_DictionaryKeyResolver(
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
            .invoke("set_DictionaryKeyResolver", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_HasParameterizedCreator(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_HasParameterizedCreator", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_KeyContract(
        &mut self,
        value: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_KeyContract", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_OverrideCreator(
        &mut self,
        value: *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_OverrideCreator", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonDictionaryContract")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonDictionaryContract {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
