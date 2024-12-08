#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSerializerInternalBase {
    __cordl_parent: crate::System::Object,
    pub _currentErrorContext: *mut crate::Newtonsoft::Json::Serialization::ErrorContext,
    pub _mappings: *mut crate::Newtonsoft::Json::Utilities::BidirectionalDictionary_2<
        *mut crate::System::String,
        *mut crate::System::Object,
    >,
    pub Serializer: *mut crate::Newtonsoft::Json::JsonSerializer,
    pub TraceWriter: *mut crate::Newtonsoft::Json::Serialization::ITraceWriter,
    pub InternalSerializer: *mut crate::Newtonsoft::Json::Serialization::JsonSerializerProxy,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::JsonSerializerInternalBase =>
    "Newtonsoft.Json.Serialization"."JsonSerializerInternalBase"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase")]
impl crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase {
    #[cfg(
        feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase+ReferenceEqualsEqualityComparer"
    )]
    pub type ReferenceEqualsEqualityComparer = crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase_ReferenceEqualsEqualityComparer;
    pub fn ClearErrorContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearErrorContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetErrorContext(
        &mut self,
        currentObject: *mut crate::System::Object,
        member: *mut crate::System::Object,
        path: *mut crate::System::String,
        error: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::ErrorContext,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::ErrorContext = __cordl_object
            .invoke("GetErrorContext", (currentObject, member, path, error))?;
        Ok(__cordl_ret)
    }
    pub fn IsErrorHandled(
        &mut self,
        currentObject: *mut crate::System::Object,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        keyValue: *mut crate::System::Object,
        lineInfo: *mut crate::Newtonsoft::Json::IJsonLineInfo,
        path: *mut crate::System::String,
        ex: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "IsErrorHandled",
                (currentObject, contract, keyValue, lineInfo, path, ex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        serializer: *mut crate::Newtonsoft::Json::JsonSerializer,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serializer))?;
        Ok(__cordl_object)
    }
    pub fn ResolvedNullValueHandling(
        &mut self,
        containerContract: *mut crate::Newtonsoft::Json::Serialization::JsonObjectContract,
        property: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::NullValueHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::NullValueHandling = __cordl_object
            .invoke("ResolvedNullValueHandling", (containerContract, property))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        serializer: *mut crate::Newtonsoft::Json::JsonSerializer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (serializer))?;
        Ok(__cordl_ret)
    }
    pub fn get_DefaultReferenceMappings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Utilities::BidirectionalDictionary_2<
            *mut crate::System::String,
            *mut crate::System::Object,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Utilities::BidirectionalDictionary_2<
            *mut crate::System::String,
            *mut crate::System::Object,
        > = __cordl_object.invoke("get_DefaultReferenceMappings", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase+ReferenceEqualsEqualityComparer"
)]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSerializerInternalBase_ReferenceEqualsEqualityComparer {
    __cordl_parent: crate::System::Object,
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase+ReferenceEqualsEqualityComparer"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::JsonSerializerInternalBase_ReferenceEqualsEqualityComparer
    => "Newtonsoft.Json.Serialization"
    ."JsonSerializerInternalBase/ReferenceEqualsEqualityComparer"
);
#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase+ReferenceEqualsEqualityComparer"
)]
impl std::ops::Deref
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase_ReferenceEqualsEqualityComparer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase+ReferenceEqualsEqualityComparer"
)]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase_ReferenceEqualsEqualityComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase+ReferenceEqualsEqualityComparer"
)]
impl crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase_ReferenceEqualsEqualityComparer {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn System_Collections_Generic_IEqualityComparer_System_Object__Equals(
        &mut self,
        x: *mut crate::System::Object,
        y: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "System.Collections.Generic.IEqualityComparer<System.Object>.Equals",
                (x, y),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IEqualityComparer_System_Object__GetHashCode(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "System.Collections.Generic.IEqualityComparer<System.Object>.GetHashCode",
                (obj),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalBase+ReferenceEqualsEqualityComparer"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase_ReferenceEqualsEqualityComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
