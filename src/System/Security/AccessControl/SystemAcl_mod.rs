#[cfg(feature = "System+Security+AccessControl+SystemAcl")]
#[repr(C)]
#[derive(Debug)]
pub struct SystemAcl {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Security::AccessControl::CommonAcl,
    >,
}
#[cfg(feature = "System+Security+AccessControl+SystemAcl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::AccessControl::SystemAcl =>
    "System.Security.AccessControl"."SystemAcl"
);
#[cfg(feature = "System+Security+AccessControl+SystemAcl")]
impl std::ops::Deref for crate::System::Security::AccessControl::SystemAcl {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Security::AccessControl::CommonAcl,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+SystemAcl")]
impl std::ops::DerefMut for crate::System::Security::AccessControl::SystemAcl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+SystemAcl")]
impl crate::System::Security::AccessControl::SystemAcl {
    pub fn ApplyCanonicalSortToExplicitAces(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyCanonicalSortToExplicitAces", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAceInsertPosition(
        &mut self,
        aceQualifier: crate::System::Security::AccessControl::AceQualifier,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetAceInsertPosition", (aceQualifier))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+AccessControl+SystemAcl")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::SystemAcl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
