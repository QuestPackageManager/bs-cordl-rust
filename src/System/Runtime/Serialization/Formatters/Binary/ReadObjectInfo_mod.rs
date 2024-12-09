#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ReadObjectInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct ReadObjectInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub objectInfoId: i32,
    pub objectType: *mut crate::System::Type,
    pub objectManager: *mut crate::System::Runtime::Serialization::ObjectManager,
    pub count: i32,
    pub isSi: bool,
    pub isNamed: bool,
    pub isTyped: bool,
    pub bSimpleAssembly: bool,
    pub cache: *mut crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoCache,
    pub wireMemberNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub wireMemberTypes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Type,
    >,
    pub lastPosition: i32,
    pub serializationSurrogate: *mut crate::System::Runtime::Serialization::ISerializationSurrogate,
    pub context: crate::System::Runtime::Serialization::StreamingContext,
    pub memberTypesList: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Type,
    >,
    pub serObjectInfoInit: *mut crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoInit,
    pub formatterConverter: *mut crate::System::Runtime::Serialization::IFormatterConverter,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ReadObjectInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::ReadObjectInfo =>
    "System.Runtime.Serialization.Formatters.Binary"."ReadObjectInfo"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ReadObjectInfo")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::ReadObjectInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ReadObjectInfo")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::ReadObjectInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ReadObjectInfo")]
impl crate::System::Runtime::Serialization::Formatters::Binary::ReadObjectInfo {
    pub fn AddValue(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
        si: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Runtime::Serialization::SerializationInfo,
        >,
        memberData: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddValue", (name, value, si, memberData))?;
        Ok(__cordl_ret)
    }
    pub fn GetMemberInfo(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MemberInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MemberInfo = __cordl_object
            .invoke("GetMemberInfo", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetMemberType(
        &mut self,
        objMember: *mut crate::System::Reflection::MemberInfo,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetMemberType", (objMember))?;
        Ok(__cordl_ret)
    }
    pub fn GetMemberTypes(
        &mut self,
        inMemberNames: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        objectType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Type,
        > = __cordl_object.invoke("GetMemberTypes", (inMemberNames, objectType))?;
        Ok(__cordl_ret)
    }
    pub fn GetType(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetType", (name))?;
        Ok(__cordl_ret)
    }
    pub fn InitDataStore(
        &mut self,
        si: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Runtime::Serialization::SerializationInfo,
        >,
        memberData: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitDataStore", (si, memberData))?;
        Ok(__cordl_ret)
    }
    pub fn InitMemberInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitMemberInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitNoMembers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitNoMembers", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitReadConstructor(
        &mut self,
        objectType: *mut crate::System::Type,
        surrogateSelector: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitReadConstructor", (objectType, surrogateSelector, context))?;
        Ok(__cordl_ret)
    }
    pub fn InitSiRead(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitSiRead", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init_ISurrogateSelector_StreamingContext_ObjectManager_SerObjectInfoInit_IFormatterConverter__cordl_bool0(
        &mut self,
        objectType: *mut crate::System::Type,
        surrogateSelector: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
        context: crate::System::Runtime::Serialization::StreamingContext,
        objectManager: *mut crate::System::Runtime::Serialization::ObjectManager,
        serObjectInfoInit: *mut crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoInit,
        converter: *mut crate::System::Runtime::Serialization::IFormatterConverter,
        bSimpleAssembly: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    objectType,
                    surrogateSelector,
                    context,
                    objectManager,
                    serObjectInfoInit,
                    converter,
                    bSimpleAssembly,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Init_Il2CppArray_Il2CppArray_ISurrogateSelector_StreamingContext_ObjectManager_SerObjectInfoInit_IFormatterConverter__cordl_bool1(
        &mut self,
        objectType: *mut crate::System::Type,
        memberNames: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        memberTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        surrogateSelector: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
        context: crate::System::Runtime::Serialization::StreamingContext,
        objectManager: *mut crate::System::Runtime::Serialization::ObjectManager,
        serObjectInfoInit: *mut crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoInit,
        converter: *mut crate::System::Runtime::Serialization::IFormatterConverter,
        bSimpleAssembly: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    objectType,
                    memberNames,
                    memberTypes,
                    surrogateSelector,
                    context,
                    objectManager,
                    serObjectInfoInit,
                    converter,
                    bSimpleAssembly,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ObjectEnd(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ObjectEnd", ())?;
        Ok(__cordl_ret)
    }
    pub fn PopulateObjectMembers(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
        memberData: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopulateObjectMembers", (obj, memberData))?;
        Ok(__cordl_ret)
    }
    pub fn Position(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Position", (name))?;
        Ok(__cordl_ret)
    }
    pub fn PrepareForReuse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrepareForReuse", ())?;
        Ok(__cordl_ret)
    }
    pub fn RecordFixup(
        &mut self,
        objectId: i64,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        idRef: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecordFixup", (objectId, name, idRef))?;
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
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ReadObjectInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::ReadObjectInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
