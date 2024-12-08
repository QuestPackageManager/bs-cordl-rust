#[cfg(feature = "MockPlatformEntitlement")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlatformEntitlement {
    __cordl_parent: crate::System::Object,
    pub _id: *mut crate::System::String,
    pub _status: EntitlementStatus,
}
#[cfg(feature = "MockPlatformEntitlement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MockPlatformEntitlement => ""."MockPlatformEntitlement"
);
#[cfg(feature = "MockPlatformEntitlement")]
impl std::ops::Deref for MockPlatformEntitlement {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlatformEntitlement")]
impl std::ops::DerefMut for MockPlatformEntitlement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlatformEntitlement")]
impl MockPlatformEntitlement {
    pub fn get_id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_id", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_status(&mut self) -> quest_hook::libil2cpp::Result<EntitlementStatus> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: EntitlementStatus = __cordl_object.invoke("get_status", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetIt(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetIt", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        id: *mut crate::System::String,
        status: EntitlementStatus,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (id, status))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        id: *mut crate::System::String,
        status: EntitlementStatus,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (id, status))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MockPlatformEntitlement")]
impl quest_hook::libil2cpp::ObjectType for MockPlatformEntitlement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
