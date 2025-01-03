#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+WriteObjectInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct WriteObjectInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub objectInfoId: i32,
    pub obj: *mut quest_hook::libil2cpp::Il2CppObject,
    pub objectType: *mut crate::System::Type,
    pub isSi: bool,
    pub isNamed: bool,
    pub isTyped: bool,
    pub isArray: bool,
    pub si: *mut crate::System::Runtime::Serialization::SerializationInfo,
    pub cache: *mut crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoCache,
    pub memberData: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub serializationSurrogate: *mut crate::System::Runtime::Serialization::ISerializationSurrogate,
    pub context: crate::System::Runtime::Serialization::StreamingContext,
    pub serObjectInfoInit: *mut crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoInit,
    pub objectId: i64,
    pub assemId: i64,
    pub binderTypeName: *mut quest_hook::libil2cpp::Il2CppString,
    pub binderAssemblyString: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+WriteObjectInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo =>
    "System.Runtime.Serialization.Formatters.Binary"."WriteObjectInfo"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+WriteObjectInfo")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+WriteObjectInfo")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+WriteObjectInfo")]
impl crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo {
    pub fn CheckTypeForwardedFrom(
        cache: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoCache,
        >,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        binderAssemblyString: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CheckTypeForwardedFrom",
                (cache, objectType, binderAssemblyString),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssemblyString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetAssemblyString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMemberInfo(
        &mut self,
        outMemberNames: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
        outMemberTypes: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
        outMemberData: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetMemberInfo", (outMemberNames, outMemberTypes, outMemberData))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMemberType(
        &mut self,
        objMember: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("GetMemberType", (objMember))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectInfo(
        serObjectInfoInit: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoInit,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetObjectInfo", (serObjectInfoInit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeFullName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetTypeFullName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitMemberInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitMemberInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitNoMembers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitNoMembers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitSerialize_Il2CppObject_ObjectWriter_SerializationBinder0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        surrogateSelector: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISurrogateSelector,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
        serObjectInfoInit: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoInit,
        >,
        converter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IFormatterConverter,
        >,
        objectWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::ObjectWriter,
        >,
        binder: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationBinder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InitSerialize",
                (
                    obj,
                    surrogateSelector,
                    context,
                    serObjectInfoInit,
                    converter,
                    objectWriter,
                    binder,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InitSerialize_Type_SerializationBinder1(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        surrogateSelector: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISurrogateSelector,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
        serObjectInfoInit: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoInit,
        >,
        converter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IFormatterConverter,
        >,
        binder: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationBinder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InitSerialize",
                (
                    objectType,
                    surrogateSelector,
                    context,
                    serObjectInfoInit,
                    converter,
                    binder,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InitSiWrite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitSiWrite", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalInit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalInit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeSerializationBinder(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationBinder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeSerializationBinder", (binder))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ObjectEnd(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ObjectEnd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PutObjectInfo(
        serObjectInfoInit: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoInit,
        >,
        objectInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PutObjectInfo", (serObjectInfoInit, objectInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize_Il2CppObject_ObjectWriter_SerializationBinder0(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        surrogateSelector: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISurrogateSelector,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
        serObjectInfoInit: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoInit,
        >,
        converter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IFormatterConverter,
        >,
        objectWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::ObjectWriter,
        >,
        binder: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationBinder,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Serialize",
                (
                    obj,
                    surrogateSelector,
                    context,
                    serObjectInfoInit,
                    converter,
                    objectWriter,
                    binder,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize_Type_SerializationBinder1(
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        surrogateSelector: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISurrogateSelector,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
        serObjectInfoInit: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoInit,
        >,
        converter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::IFormatterConverter,
        >,
        binder: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationBinder,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Serialize",
                (
                    objectType,
                    surrogateSelector,
                    context,
                    serObjectInfoInit,
                    converter,
                    binder,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+WriteObjectInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
