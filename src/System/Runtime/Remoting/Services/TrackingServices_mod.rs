#[cfg(feature = "System+Runtime+Remoting+Services+TrackingServices")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackingServices {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Runtime+Remoting+Services+TrackingServices")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Services::TrackingServices =>
    "System.Runtime.Remoting.Services"."TrackingServices"
);
#[cfg(feature = "System+Runtime+Remoting+Services+TrackingServices")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Services::TrackingServices {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Services+TrackingServices")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Services::TrackingServices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Services+TrackingServices")]
impl crate::System::Runtime::Remoting::Services::TrackingServices {}
#[cfg(feature = "System+Runtime+Remoting+Services+TrackingServices")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Services::TrackingServices {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}