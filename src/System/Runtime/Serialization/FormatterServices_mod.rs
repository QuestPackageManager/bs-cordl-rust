#[cfg(feature = "System+Runtime+Serialization+FormatterServices")]
#[repr(C)]
#[derive(Debug)]
pub struct FormatterServices {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Serialization+FormatterServices")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::FormatterServices => "System.Runtime.Serialization"
    ."FormatterServices"
);
#[cfg(feature = "System+Runtime+Serialization+FormatterServices")]
impl std::ops::Deref for crate::System::Runtime::Serialization::FormatterServices {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+FormatterServices")]
impl std::ops::DerefMut for crate::System::Runtime::Serialization::FormatterServices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+FormatterServices")]
impl crate::System::Runtime::Serialization::FormatterServices {
    pub fn CheckSerializable(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckSerializable", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClrAssemblyName(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        hasTypeForwardedFrom: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetClrAssemblyName", (_cordl_type, hasTypeForwardedFrom))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClrTypeFullName(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetClrTypeFullName", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClrTypeFullNameForArray(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetClrTypeFullNameForArray", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClrTypeFullNameForNonArrayTypes(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetClrTypeFullNameForNonArrayTypes", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnableUnsafeTypeForwarders() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEnableUnsafeTypeForwarders", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectData(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        members: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::MemberInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetObjectData", (obj, members))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParentTypes(
        parentType: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
        parentTypes: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<*mut crate::System::RuntimeType>,
            >,
        >,
        parentTypeCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetParentTypes", (parentType, parentTypes, parentTypeCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSerializableMembers_RuntimeType0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::MemberInfo,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::MemberInfo,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSerializableMembers", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSerializableMembers_Type_StreamingContext1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::MemberInfo,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::MemberInfo,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSerializableMembers", (_cordl_type, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeFromAssembly(
        assem: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTypeFromAssembly", (assem, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUninitializedObject(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUninitializedObject", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetSerializableMembers(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::MemberInfo,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::MemberInfo,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalGetSerializableMembers", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssemblyFromString(
        assemblyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::Assembly,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAssemblyFromString", (assemblyName))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssemblyFromStringNoThrow(
        assemblyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::Assembly,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAssemblyFromStringNoThrow", (assemblyName))?;
        Ok(__cordl_ret.into())
    }
    pub fn PopulateObjectMembers(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        members: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::MemberInfo,
            >,
        >,
        data: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PopulateObjectMembers", (obj, members, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializationSetValue(
        fi: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializationSetValue", (fi, target, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeTypeForwardersIsEnabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnsafeTypeForwardersIsEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn nativeGetUninitializedObject(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("nativeGetUninitializedObject", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Serialization+FormatterServices")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::FormatterServices {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
