#[cfg(feature = "System+Security+SecurityManager")]
#[repr(C)]
#[derive(Debug)]
pub struct SecurityManager {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Security+SecurityManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::SecurityManager =>
    "System.Security"."SecurityManager"
);
#[cfg(feature = "System+Security+SecurityManager")]
impl std::ops::Deref for crate::System::Security::SecurityManager {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+SecurityManager")]
impl std::ops::DerefMut for crate::System::Security::SecurityManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+SecurityManager")]
impl crate::System::Security::SecurityManager {
    pub fn EnsureElevatedPermissions() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureElevatedPermissions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SecurityEnabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_SecurityEnabled", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+SecurityManager")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Security::SecurityManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
