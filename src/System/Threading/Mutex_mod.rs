#[cfg(feature = "System+Threading+Mutex")]
#[repr(C)]
#[derive(Debug)]
pub struct Mutex {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
}
#[cfg(feature = "System+Threading+Mutex")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Mutex => "System.Threading"
    ."Mutex"
);
#[cfg(feature = "System+Threading+Mutex")]
impl std::ops::Deref for crate::System::Threading::Mutex {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Mutex")]
impl std::ops::DerefMut for crate::System::Threading::Mutex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Mutex")]
impl crate::System::Threading::Mutex {}
#[cfg(feature = "System+Threading+Mutex")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::Mutex {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
