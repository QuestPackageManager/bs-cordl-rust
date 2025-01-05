#[cfg(feature = "System+Security+AccessControl+AuthorizationRuleCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct AuthorizationRuleCollection {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ReadOnlyCollectionBase,
    >,
}
#[cfg(feature = "System+Security+AccessControl+AuthorizationRuleCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::AccessControl::AuthorizationRuleCollection =>
    "System.Security.AccessControl"."AuthorizationRuleCollection"
);
#[cfg(feature = "System+Security+AccessControl+AuthorizationRuleCollection")]
impl std::ops::Deref
for crate::System::Security::AccessControl::AuthorizationRuleCollection {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Collections::ReadOnlyCollectionBase,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+AuthorizationRuleCollection")]
impl std::ops::DerefMut
for crate::System::Security::AccessControl::AuthorizationRuleCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+AuthorizationRuleCollection")]
impl crate::System::Security::AccessControl::AuthorizationRuleCollection {
    pub fn New(
        rules: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::AccessControl::AuthorizationRule,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rules))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        rules: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::AccessControl::AuthorizationRule,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rules))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+AccessControl+AuthorizationRuleCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::AuthorizationRuleCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
