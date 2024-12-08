#[cfg(feature = "System+Security+AccessControl+KnownAce")]
#[repr(C)]
#[derive(Debug)]
pub struct KnownAce {
    __cordl_parent: crate::System::Security::AccessControl::GenericAce,
    pub access_mask: i32,
    pub identifier: *mut crate::System::Security::Principal::SecurityIdentifier,
}
#[cfg(feature = "System+Security+AccessControl+KnownAce")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::AccessControl::KnownAce =>
    "System.Security.AccessControl"."KnownAce"
);
#[cfg(feature = "System+Security+AccessControl+KnownAce")]
impl std::ops::Deref for crate::System::Security::AccessControl::KnownAce {
    type Target = crate::System::Security::AccessControl::GenericAce;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+KnownAce")]
impl std::ops::DerefMut for crate::System::Security::AccessControl::KnownAce {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+KnownAce")]
impl crate::System::Security::AccessControl::KnownAce {
    pub fn New_AceType_AceFlags0(
        _cordl_type: crate::System::Security::AccessControl::AceType,
        flags: crate::System::Security::AccessControl::AceFlags,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, flags))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_i32_1(
        binaryForm: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (binaryForm, offset))?;
        Ok(__cordl_object)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_i32_1(
        &mut self,
        binaryForm: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (binaryForm, offset))?;
        Ok(__cordl_ret)
    }
    pub fn get_AccessMask(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_AccessMask", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SecurityIdentifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Principal::SecurityIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Principal::SecurityIdentifier = __cordl_object
            .invoke("get_SecurityIdentifier", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_AccessMask(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AccessMask", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SecurityIdentifier(
        &mut self,
        value: *mut crate::System::Security::Principal::SecurityIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SecurityIdentifier", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Security+AccessControl+KnownAce")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::KnownAce {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
