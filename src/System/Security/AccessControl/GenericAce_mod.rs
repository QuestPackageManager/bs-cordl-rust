#[cfg(feature = "System+Security+AccessControl+GenericAce")]
#[repr(C)]
#[derive(Debug)]
pub struct GenericAce {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ace_flags: crate::System::Security::AccessControl::AceFlags,
    pub ace_type: crate::System::Security::AccessControl::AceType,
}
#[cfg(feature = "System+Security+AccessControl+GenericAce")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::AccessControl::GenericAce {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.AccessControl";
    const CLASS_NAME: &'static str = "GenericAce";
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
#[cfg(feature = "System+Security+AccessControl+GenericAce")]
impl std::ops::Deref for crate::System::Security::AccessControl::GenericAce {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+GenericAce")]
impl std::ops::DerefMut for crate::System::Security::AccessControl::GenericAce {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+GenericAce")]
impl crate::System::Security::AccessControl::GenericAce {
    pub fn CreateFromBinaryForm(
        binaryForm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::AccessControl::GenericAce>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::GenericAce,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateFromBinaryForm", (binaryForm, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (o))?;
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
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsObjectType(
        _cordl_type: crate::System::Security::AccessControl::AceType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsObjectType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_AceType_AceFlags0(
        _cordl_type: crate::System::Security::AccessControl::AceType,
        flags: crate::System::Security::AccessControl::AceFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, flags))?;
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
    pub fn ReadInt(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadInt", (buffer, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadUShort(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadUShort", (buffer, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteInt(
        val: i32,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteInt", (val, buffer, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUShort(
        val: u16,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUShort", (val, buffer, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_AceType_AceFlags0(
        &mut self,
        _cordl_type: crate::System::Security::AccessControl::AceType,
        flags: crate::System::Security::AccessControl::AceFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, flags))?;
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
    pub fn get_AceFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::AccessControl::AceFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::AccessControl::AceFlags = __cordl_object
            .invoke("get_AceFlags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AceType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Security::AccessControl::AceType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::AccessControl::AceType = __cordl_object
            .invoke("get_AceType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AuditFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::AccessControl::AuditFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::AccessControl::AuditFlags = __cordl_object
            .invoke("get_AuditFlags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BinaryLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_BinaryLength", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InheritanceFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::AccessControl::InheritanceFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::AccessControl::InheritanceFlags = __cordl_object
            .invoke("get_InheritanceFlags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsInherited(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsInherited", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PropagationFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::AccessControl::PropagationFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::AccessControl::PropagationFlags = __cordl_object
            .invoke("get_PropagationFlags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::GenericAce,
        >,
        right: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::GenericAce,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        left: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::GenericAce,
        >,
        right: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::GenericAce,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+AccessControl+GenericAce")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::GenericAce {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
