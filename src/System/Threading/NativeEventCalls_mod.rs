#[cfg(feature = "System+Threading+NativeEventCalls")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeEventCalls {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Threading+NativeEventCalls")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::NativeEventCalls =>
    "System.Threading"."NativeEventCalls"
);
#[cfg(feature = "System+Threading+NativeEventCalls")]
impl std::ops::Deref for crate::System::Threading::NativeEventCalls {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+NativeEventCalls")]
impl std::ops::DerefMut for crate::System::Threading::NativeEventCalls {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+NativeEventCalls")]
impl crate::System::Threading::NativeEventCalls {}
#[cfg(feature = "System+Threading+NativeEventCalls")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::NativeEventCalls {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
