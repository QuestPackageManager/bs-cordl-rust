#[cfg(feature = "System+Security+AccessControl+QualifiedAce")]
#[repr(C)]
#[derive(Debug)]
pub struct QualifiedAce {
    __cordl_parent: crate::System::Security::AccessControl::KnownAce,
    pub opaque: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "System+Security+AccessControl+QualifiedAce")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::AccessControl::QualifiedAce =>
    "System.Security.AccessControl"."QualifiedAce"
);
#[cfg(feature = "System+Security+AccessControl+QualifiedAce")]
impl std::ops::Deref for crate::System::Security::AccessControl::QualifiedAce {
    type Target = crate::System::Security::AccessControl::KnownAce;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+QualifiedAce")]
impl std::ops::DerefMut for crate::System::Security::AccessControl::QualifiedAce {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+QualifiedAce")]
impl crate::System::Security::AccessControl::QualifiedAce {
    pub fn GetOpaque(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetOpaque", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_AceType_AceFlags_Il2CppArray0(
        _cordl_type: crate::System::Security::AccessControl::AceType,
        flags: crate::System::Security::AccessControl::AceFlags,
        opaque: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, flags, opaque))?;
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
    pub fn SetOpaque(
        &mut self,
        opaque: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOpaque", (opaque))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_AceType_AceFlags_Il2CppArray0(
        &mut self,
        _cordl_type: crate::System::Security::AccessControl::AceType,
        flags: crate::System::Security::AccessControl::AceFlags,
        opaque: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, flags, opaque))?;
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
    pub fn get_AceQualifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::AccessControl::AceQualifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::AccessControl::AceQualifier = __cordl_object
            .invoke("get_AceQualifier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCallback(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OpaqueLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_OpaqueLength", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+AccessControl+QualifiedAce")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::QualifiedAce {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
