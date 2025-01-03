#[cfg(feature = "Newtonsoft+Json+Serialization+JsonObjectContract")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonObjectContract {
    __cordl_parent: crate::Newtonsoft::Json::Serialization::JsonContainerContract,
    pub _MemberSerialization_k__BackingField: crate::Newtonsoft::Json::MemberSerialization,
    pub _MissingMemberHandling_k__BackingField: crate::System::Nullable_1<
        crate::Newtonsoft::Json::MissingMemberHandling,
    >,
    pub _ItemRequired_k__BackingField: crate::System::Nullable_1<
        crate::Newtonsoft::Json::Required,
    >,
    pub _ItemNullValueHandling_k__BackingField: crate::System::Nullable_1<
        crate::Newtonsoft::Json::NullValueHandling,
    >,
    pub _Properties_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::JsonPropertyCollection,
    >,
    pub _ExtensionDataSetter_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::ExtensionDataSetter,
    >,
    pub _ExtensionDataGetter_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::ExtensionDataGetter,
    >,
    pub _ExtensionDataNameResolver_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub ExtensionDataIsJToken: bool,
    pub _hasRequiredOrDefaultValueProperties: crate::System::Nullable_1<bool>,
    pub _overrideCreator: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    >,
    pub _parameterizedCreator: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    >,
    pub _creatorParameters: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::JsonPropertyCollection,
    >,
    pub _extensionDataValueType: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonObjectContract")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::JsonObjectContract =>
    "Newtonsoft.Json.Serialization"."JsonObjectContract"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonObjectContract")]
impl std::ops::Deref for crate::Newtonsoft::Json::Serialization::JsonObjectContract {
    type Target = crate::Newtonsoft::Json::Serialization::JsonContainerContract;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonObjectContract")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Serialization::JsonObjectContract {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonObjectContract")]
impl crate::Newtonsoft::Json::Serialization::JsonObjectContract {
    pub fn GetUninitializedObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetUninitializedObject", ())?;
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
    pub fn get_CreatorParameters(
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
        > = __cordl_object.invoke("get_CreatorParameters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ExtensionDataGetter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ExtensionDataGetter,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ExtensionDataGetter,
        > = __cordl_object.invoke("get_ExtensionDataGetter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ExtensionDataNameResolver(
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
        > = __cordl_object.invoke("get_ExtensionDataNameResolver", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ExtensionDataSetter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ExtensionDataSetter,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ExtensionDataSetter,
        > = __cordl_object.invoke("get_ExtensionDataSetter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ExtensionDataValueType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_ExtensionDataValueType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasRequiredOrDefaultValueProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_HasRequiredOrDefaultValueProperties", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ItemNullValueHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::Newtonsoft::Json::NullValueHandling>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<
            crate::Newtonsoft::Json::NullValueHandling,
        > = __cordl_object.invoke("get_ItemNullValueHandling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ItemRequired(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::Newtonsoft::Json::Required>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<crate::Newtonsoft::Json::Required> = __cordl_object
            .invoke("get_ItemRequired", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MemberSerialization(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::MemberSerialization> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::MemberSerialization = __cordl_object
            .invoke("get_MemberSerialization", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MissingMemberHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::Newtonsoft::Json::MissingMemberHandling>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<
            crate::Newtonsoft::Json::MissingMemberHandling,
        > = __cordl_object.invoke("get_MissingMemberHandling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OverrideCreator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object.invoke("get_OverrideCreator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParameterizedCreator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object.invoke("get_ParameterizedCreator", ())?;
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
    pub fn set_ExtensionDataGetter(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ExtensionDataGetter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ExtensionDataGetter", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ExtensionDataNameResolver(
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
            .invoke("set_ExtensionDataNameResolver", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ExtensionDataSetter(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ExtensionDataSetter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ExtensionDataSetter", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ExtensionDataValueType(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ExtensionDataValueType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ItemNullValueHandling(
        &mut self,
        value: crate::System::Nullable_1<crate::Newtonsoft::Json::NullValueHandling>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ItemNullValueHandling", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ItemRequired(
        &mut self,
        value: crate::System::Nullable_1<crate::Newtonsoft::Json::Required>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ItemRequired", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MemberSerialization(
        &mut self,
        value: crate::Newtonsoft::Json::MemberSerialization,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MemberSerialization", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MissingMemberHandling(
        &mut self,
        value: crate::System::Nullable_1<crate::Newtonsoft::Json::MissingMemberHandling>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MissingMemberHandling", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_OverrideCreator(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_OverrideCreator", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ParameterizedCreator(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ParameterizedCreator", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonObjectContract")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonObjectContract {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
