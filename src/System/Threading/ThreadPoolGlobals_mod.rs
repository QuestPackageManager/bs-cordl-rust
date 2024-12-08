#[cfg(feature = "System+Threading+ThreadPoolGlobals")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreadPoolGlobals {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Threading+ThreadPoolGlobals")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::ThreadPoolGlobals =>
    "System.Threading"."ThreadPoolGlobals"
);
#[cfg(feature = "System+Threading+ThreadPoolGlobals")]
impl std::ops::Deref for crate::System::Threading::ThreadPoolGlobals {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ThreadPoolGlobals")]
impl std::ops::DerefMut for crate::System::Threading::ThreadPoolGlobals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ThreadPoolGlobals")]
impl crate::System::Threading::ThreadPoolGlobals {}
#[cfg(feature = "System+Threading+ThreadPoolGlobals")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::ThreadPoolGlobals {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
