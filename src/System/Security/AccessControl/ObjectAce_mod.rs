#[cfg(feature = "System+Security+AccessControl+ObjectAce")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectAce {
    __cordl_parent: crate::System::Security::AccessControl::QualifiedAce,
    pub object_ace_type: crate::System::Guid,
    pub inherited_object_type: crate::System::Guid,
    pub object_ace_flags: crate::System::Security::AccessControl::ObjectAceFlags,
}
#[cfg(feature = "System+Security+AccessControl+ObjectAce")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::AccessControl::ObjectAce {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.AccessControl";
    const CLASS_NAME: &'static str = "ObjectAce";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Security+AccessControl+ObjectAce")]
impl std::ops::Deref for crate::System::Security::AccessControl::ObjectAce {
    type Target = crate::System::Security::AccessControl::QualifiedAce;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+ObjectAce")]
impl std::ops::DerefMut for crate::System::Security::AccessControl::ObjectAce {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+ObjectAce")]
impl crate::System::Security::AccessControl::ObjectAce {
    pub fn ConvertType(
        qualifier: crate::System::Security::AccessControl::AceQualifier,
        isCallback: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::Security::AccessControl::AceType> {
        let __cordl_ret: crate::System::Security::AccessControl::AceType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertType", (qualifier, isCallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBinaryForm(
        &mut self,
        binaryForm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetBinaryForm", (binaryForm, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_AceFlags_AceQualifier_i32_SecurityIdentifier_ObjectAceFlags_Guid_Guid__cordl_bool_Il2CppArray0(
        aceFlags: crate::System::Security::AccessControl::AceFlags,
        qualifier: crate::System::Security::AccessControl::AceQualifier,
        accessMask: i32,
        sid: quest_hook::libil2cpp::Gc<
            crate::System::Security::Principal::SecurityIdentifier,
        >,
        flags: crate::System::Security::AccessControl::ObjectAceFlags,
        _cordl_type: crate::System::Guid,
        inheritedType: crate::System::Guid,
        isCallback: bool,
        opaque: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    aceFlags,
                    qualifier,
                    accessMask,
                    sid,
                    flags,
                    _cordl_type,
                    inheritedType,
                    isCallback,
                    opaque,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray_i32_1(
        binaryForm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (binaryForm, offset))?;
        Ok(__cordl_object.into())
    }
    pub fn ReadGuid(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Guid = __cordl_object
            .invoke("ReadGuid", (buffer, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteGuid(
        &mut self,
        val: crate::System::Guid,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteGuid", (val, buffer, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_AceFlags_AceQualifier_i32_SecurityIdentifier_ObjectAceFlags_Guid_Guid__cordl_bool_Il2CppArray0(
        &mut self,
        aceFlags: crate::System::Security::AccessControl::AceFlags,
        qualifier: crate::System::Security::AccessControl::AceQualifier,
        accessMask: i32,
        sid: quest_hook::libil2cpp::Gc<
            crate::System::Security::Principal::SecurityIdentifier,
        >,
        flags: crate::System::Security::AccessControl::ObjectAceFlags,
        _cordl_type: crate::System::Guid,
        inheritedType: crate::System::Guid,
        isCallback: bool,
        opaque: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    aceFlags,
                    qualifier,
                    accessMask,
                    sid,
                    flags,
                    _cordl_type,
                    inheritedType,
                    isCallback,
                    opaque,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray_i32_1(
        &mut self,
        binaryForm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (binaryForm, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BinaryLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_BinaryLength", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InheritedObjectAceType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Guid = __cordl_object
            .invoke("get_InheritedObjectAceType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InheritedObjectAceTypePresent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_InheritedObjectAceTypePresent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ObjectAceFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::AccessControl::ObjectAceFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::AccessControl::ObjectAceFlags = __cordl_object
            .invoke("get_ObjectAceFlags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ObjectAceType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Guid = __cordl_object
            .invoke("get_ObjectAceType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ObjectAceTypePresent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ObjectAceTypePresent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_InheritedObjectAceType(
        &mut self,
        value: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InheritedObjectAceType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ObjectAceFlags(
        &mut self,
        value: crate::System::Security::AccessControl::ObjectAceFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ObjectAceFlags", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ObjectAceType(
        &mut self,
        value: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ObjectAceType", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+AccessControl+ObjectAce")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::ObjectAce {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
