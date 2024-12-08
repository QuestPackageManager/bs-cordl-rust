#[cfg(feature = "System+Security+AccessControl+CommonAce")]
#[repr(C)]
#[derive(Debug)]
pub struct CommonAce {
    __cordl_parent: crate::System::Security::AccessControl::QualifiedAce,
}
#[cfg(feature = "System+Security+AccessControl+CommonAce")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::AccessControl::CommonAce =>
    "System.Security.AccessControl"."CommonAce"
);
#[cfg(feature = "System+Security+AccessControl+CommonAce")]
impl std::ops::Deref for crate::System::Security::AccessControl::CommonAce {
    type Target = crate::System::Security::AccessControl::QualifiedAce;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+CommonAce")]
impl std::ops::DerefMut for crate::System::Security::AccessControl::CommonAce {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+CommonAce")]
impl crate::System::Security::AccessControl::CommonAce {
    pub fn GetBinaryForm(
        &mut self,
        binaryForm: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetBinaryForm", (binaryForm, offset))?;
        Ok(__cordl_ret)
    }
    pub fn New_AceFlags_AceQualifier_i32_SecurityIdentifier__cordl_bool_Il2CppArray0(
        flags: crate::System::Security::AccessControl::AceFlags,
        qualifier: crate::System::Security::AccessControl::AceQualifier,
        accessMask: i32,
        sid: *mut crate::System::Security::Principal::SecurityIdentifier,
        isCallback: bool,
        opaque: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (flags, qualifier, accessMask, sid, isCallback, opaque),
            )?;
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
    pub fn _ctor_AceFlags_AceQualifier_i32_SecurityIdentifier__cordl_bool_Il2CppArray0(
        &mut self,
        flags: crate::System::Security::AccessControl::AceFlags,
        qualifier: crate::System::Security::AccessControl::AceQualifier,
        accessMask: i32,
        sid: *mut crate::System::Security::Principal::SecurityIdentifier,
        isCallback: bool,
        opaque: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (flags, qualifier, accessMask, sid, isCallback, opaque))?;
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
    pub fn get_BinaryLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_BinaryLength", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Security+AccessControl+CommonAce")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::CommonAce {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
