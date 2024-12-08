#[cfg(feature = "System+Security+AccessControl+DiscretionaryAcl")]
#[repr(C)]
#[derive(Debug)]
pub struct DiscretionaryAcl {
    __cordl_parent: crate::System::Security::AccessControl::CommonAcl,
}
#[cfg(feature = "System+Security+AccessControl+DiscretionaryAcl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::AccessControl::DiscretionaryAcl =>
    "System.Security.AccessControl"."DiscretionaryAcl"
);
#[cfg(feature = "System+Security+AccessControl+DiscretionaryAcl")]
impl std::ops::Deref for crate::System::Security::AccessControl::DiscretionaryAcl {
    type Target = crate::System::Security::AccessControl::CommonAcl;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+DiscretionaryAcl")]
impl std::ops::DerefMut for crate::System::Security::AccessControl::DiscretionaryAcl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+DiscretionaryAcl")]
impl crate::System::Security::AccessControl::DiscretionaryAcl {
    pub fn AddAccess(
        &mut self,
        accessType: crate::System::Security::AccessControl::AccessControlType,
        sid: *mut crate::System::Security::Principal::SecurityIdentifier,
        accessMask: i32,
        inheritanceFlags: crate::System::Security::AccessControl::InheritanceFlags,
        propagationFlags: crate::System::Security::AccessControl::PropagationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddAccess",
                (accessType, sid, accessMask, inheritanceFlags, propagationFlags),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ApplyCanonicalSortToExplicitAces(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyCanonicalSortToExplicitAces", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn IsAceMeaningless(
        &mut self,
        ace: *mut crate::System::Security::AccessControl::GenericAce,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsAceMeaningless", (ace))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        isContainer: bool,
        isDS: bool,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (isContainer, isDS, capacity))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        isContainer: bool,
        isDS: bool,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (isContainer, isDS, capacity))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Security+AccessControl+DiscretionaryAcl")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::DiscretionaryAcl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}