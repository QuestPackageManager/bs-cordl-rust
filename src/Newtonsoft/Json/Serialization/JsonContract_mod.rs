#[cfg(feature = "Newtonsoft+Json+Serialization+JsonContract")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonContract {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub IsNullable: bool,
    pub IsConvertable: bool,
    pub IsEnum: bool,
    pub NonNullableUnderlyingType: *mut crate::System::Type,
    pub InternalReadType: crate::Newtonsoft::Json::ReadType,
    pub ContractType: crate::Newtonsoft::Json::Serialization::JsonContractType,
    pub IsReadOnlyOrFixedSize: bool,
    pub IsSealed: bool,
    pub IsInstantiable: bool,
    pub _onDeserializedCallbacks: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Newtonsoft::Json::Serialization::SerializationCallback,
    >,
    pub _onDeserializingCallbacks: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Newtonsoft::Json::Serialization::SerializationCallback,
    >,
    pub _onSerializedCallbacks: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Newtonsoft::Json::Serialization::SerializationCallback,
    >,
    pub _onSerializingCallbacks: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Newtonsoft::Json::Serialization::SerializationCallback,
    >,
    pub _onErrorCallbacks: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Newtonsoft::Json::Serialization::SerializationErrorCallback,
    >,
    pub _createdType: *mut crate::System::Type,
    pub _UnderlyingType_k__BackingField: *mut crate::System::Type,
    pub _IsReference_k__BackingField: crate::System::Nullable_1<bool>,
    pub _Converter_k__BackingField: *mut crate::Newtonsoft::Json::JsonConverter,
    pub _InternalConverter_k__BackingField: *mut crate::Newtonsoft::Json::JsonConverter,
    pub _DefaultCreator_k__BackingField: *mut crate::System::Func_1<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _DefaultCreatorNonPublic_k__BackingField: bool,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonContract")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Serialization::JsonContract =>
    "Newtonsoft.Json.Serialization"."JsonContract"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonContract")]
impl std::ops::Deref for crate::Newtonsoft::Json::Serialization::JsonContract {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonContract")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Serialization::JsonContract {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonContract")]
impl crate::Newtonsoft::Json::Serialization::JsonContract {
    pub fn CreateSerializationCallback(
        callbackMethodInfo: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::SerializationCallback,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::SerializationCallback,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateSerializationCallback", (callbackMethodInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSerializationErrorCallback(
        callbackMethodInfo: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::SerializationErrorCallback,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::SerializationErrorCallback,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateSerializationErrorCallback", (callbackMethodInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeOnDeserialized(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeOnDeserialized", (o, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeOnDeserializing(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeOnDeserializing", (o, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeOnError(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        context: crate::System::Runtime::Serialization::StreamingContext,
        errorContext: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ErrorContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeOnError", (o, context, errorContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeOnSerialized(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeOnSerialized", (o, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeOnSerializing(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeOnSerializing", (o, context))?;
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
    pub fn get_Converter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonConverter>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonConverter,
        > = __cordl_object.invoke("get_Converter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CreatedType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_CreatedType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultCreator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_1<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<*mut quest_hook::libil2cpp::Il2CppObject>,
        > = __cordl_object.invoke("get_DefaultCreator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultCreatorNonPublic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_DefaultCreatorNonPublic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalConverter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonConverter>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonConverter,
        > = __cordl_object.invoke("get_InternalConverter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<bool> = __cordl_object
            .invoke("get_IsReference", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OnDeserializedCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::Newtonsoft::Json::Serialization::SerializationCallback,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::Newtonsoft::Json::Serialization::SerializationCallback,
            >,
        > = __cordl_object.invoke("get_OnDeserializedCallbacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OnDeserializingCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::Newtonsoft::Json::Serialization::SerializationCallback,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::Newtonsoft::Json::Serialization::SerializationCallback,
            >,
        > = __cordl_object.invoke("get_OnDeserializingCallbacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OnErrorCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::Newtonsoft::Json::Serialization::SerializationErrorCallback,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::Newtonsoft::Json::Serialization::SerializationErrorCallback,
            >,
        > = __cordl_object.invoke("get_OnErrorCallbacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OnSerializedCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::Newtonsoft::Json::Serialization::SerializationCallback,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::Newtonsoft::Json::Serialization::SerializationCallback,
            >,
        > = __cordl_object.invoke("get_OnSerializedCallbacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OnSerializingCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::Newtonsoft::Json::Serialization::SerializationCallback,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::Newtonsoft::Json::Serialization::SerializationCallback,
            >,
        > = __cordl_object.invoke("get_OnSerializingCallbacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnderlyingType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_UnderlyingType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Converter(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonConverter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Converter", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CreatedType(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CreatedType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DefaultCreator(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DefaultCreator", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DefaultCreatorNonPublic(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DefaultCreatorNonPublic", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_InternalConverter(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonConverter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InternalConverter", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsReference(
        &mut self,
        value: crate::System::Nullable_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsReference", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonContract")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonContract {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
