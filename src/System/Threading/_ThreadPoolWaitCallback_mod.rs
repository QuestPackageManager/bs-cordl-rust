#[cfg(feature = "System+Threading+_ThreadPoolWaitCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct _ThreadPoolWaitCallback {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+_ThreadPoolWaitCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::_ThreadPoolWaitCallback =>
    "System.Threading"."_ThreadPoolWaitCallback"
);
#[cfg(feature = "System+Threading+_ThreadPoolWaitCallback")]
impl std::ops::Deref for crate::System::Threading::_ThreadPoolWaitCallback {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+_ThreadPoolWaitCallback")]
impl std::ops::DerefMut for crate::System::Threading::_ThreadPoolWaitCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+_ThreadPoolWaitCallback")]
impl crate::System::Threading::_ThreadPoolWaitCallback {
    pub fn PerformWaitCallback() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PerformWaitCallback", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+_ThreadPoolWaitCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::_ThreadPoolWaitCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
