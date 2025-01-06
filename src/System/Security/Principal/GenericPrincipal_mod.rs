#[cfg(feature = "System+Security+Principal+GenericPrincipal")]
#[repr(C)]
#[derive(Debug)]
pub struct GenericPrincipal {
    __cordl_parent: crate::System::Security::Claims::ClaimsPrincipal,
    pub m_identity: quest_hook::libil2cpp::Gc<
        crate::System::Security::Principal::IIdentity,
    >,
    pub m_roles: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
}
#[cfg(feature = "System+Security+Principal+GenericPrincipal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Principal::GenericPrincipal =>
    "System.Security.Principal"."GenericPrincipal"
);
#[cfg(feature = "System+Security+Principal+GenericPrincipal")]
impl std::ops::Deref for crate::System::Security::Principal::GenericPrincipal {
    type Target = crate::System::Security::Claims::ClaimsPrincipal;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Principal+GenericPrincipal")]
impl std::ops::DerefMut for crate::System::Security::Principal::GenericPrincipal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Principal+GenericPrincipal")]
impl crate::System::Security::Principal::GenericPrincipal {
    pub fn New(
        identity: quest_hook::libil2cpp::Gc<
            crate::System::Security::Principal::IIdentity,
        >,
        roles: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (identity, roles))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        identity: quest_hook::libil2cpp::Gc<
            crate::System::Security::Principal::IIdentity,
        >,
        roles: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (identity, roles))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Principal+GenericPrincipal")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Principal::GenericPrincipal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
