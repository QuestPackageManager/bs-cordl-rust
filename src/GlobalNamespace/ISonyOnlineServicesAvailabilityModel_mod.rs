#[cfg(feature = "ISonyOnlineServicesAvailabilityModel")]
#[repr(C)]
#[derive(Debug)]
pub struct ISonyOnlineServicesAvailabilityModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ISonyOnlineServicesAvailabilityModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ISonyOnlineServicesAvailabilityModel => ""
    ."ISonyOnlineServicesAvailabilityModel"
);
#[cfg(feature = "ISonyOnlineServicesAvailabilityModel")]
impl std::ops::Deref for ISonyOnlineServicesAvailabilityModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ISonyOnlineServicesAvailabilityModel")]
impl std::ops::DerefMut for ISonyOnlineServicesAvailabilityModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ISonyOnlineServicesAvailabilityModel")]
impl ISonyOnlineServicesAvailabilityModel {
    #[cfg(feature = "ISonyOnlineServicesAvailabilityModel+OnlineServicesAvailability")]
    pub type OnlineServicesAvailability = crate::GlobalNamespace::ISonyOnlineServicesAvailabilityModel_OnlineServicesAvailability;
    pub fn GetOnlineServicesAvailabilityAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::ISonyOnlineServicesAvailabilityModel_OnlineServicesAvailability,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::ISonyOnlineServicesAvailabilityModel_OnlineServicesAvailability,
        > = __cordl_object.invoke("GetOnlineServicesAvailabilityAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "ISonyOnlineServicesAvailabilityModel")]
impl quest_hook::libil2cpp::ObjectType for ISonyOnlineServicesAvailabilityModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ISonyOnlineServicesAvailabilityModel+OnlineServicesAvailability")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ISonyOnlineServicesAvailabilityModel_OnlineServicesAvailability {
    Available = 0i32,
    Unavailable = 1i32,
    UnknownError = 2i32,
}
#[cfg(feature = "ISonyOnlineServicesAvailabilityModel+OnlineServicesAvailability")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ISonyOnlineServicesAvailabilityModel_OnlineServicesAvailability =>
    ""."ISonyOnlineServicesAvailabilityModel/OnlineServicesAvailability"
);
