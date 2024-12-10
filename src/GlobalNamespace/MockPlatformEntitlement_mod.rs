#[cfg(feature = "MockPlatformEntitlement")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlatformEntitlement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _id: *mut quest_hook::libil2cpp::Il2CppString,
    pub _status: crate::GlobalNamespace::EntitlementStatus,
}
#[cfg(feature = "MockPlatformEntitlement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MockPlatformEntitlement => ""
    ."MockPlatformEntitlement"
);
#[cfg(feature = "MockPlatformEntitlement")]
impl std::ops::Deref for crate::GlobalNamespace::MockPlatformEntitlement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlatformEntitlement")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockPlatformEntitlement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlatformEntitlement")]
impl crate::GlobalNamespace::MockPlatformEntitlement {
    pub fn GetIt(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetIt", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        status: crate::GlobalNamespace::EntitlementStatus,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (id, status))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        status: crate::GlobalNamespace::EntitlementStatus,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (id, status))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_id", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_status(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::EntitlementStatus> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::EntitlementStatus = __cordl_object
            .invoke("get_status", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MockPlatformEntitlement")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MockPlatformEntitlement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
