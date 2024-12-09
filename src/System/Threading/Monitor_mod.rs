#[cfg(feature = "System+Threading+Monitor")]
#[repr(C)]
#[derive(Debug)]
pub struct Monitor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+Monitor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Monitor => "System.Threading"
    ."Monitor"
);
#[cfg(feature = "System+Threading+Monitor")]
impl std::ops::Deref for crate::System::Threading::Monitor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Monitor")]
impl std::ops::DerefMut for crate::System::Threading::Monitor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Monitor")]
impl crate::System::Threading::Monitor {}
#[cfg(feature = "System+Threading+Monitor")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::Monitor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
