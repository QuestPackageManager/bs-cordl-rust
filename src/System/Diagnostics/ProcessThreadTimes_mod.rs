#[cfg(feature = "System+Diagnostics+ProcessThreadTimes")]
#[repr(C)]
#[derive(Debug)]
pub struct ProcessThreadTimes {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub create: i64,
    pub _cordl_exit: i64,
    pub kernel: i64,
    pub user: i64,
}
#[cfg(feature = "System+Diagnostics+ProcessThreadTimes")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::ProcessThreadTimes =>
    "System.Diagnostics"."ProcessThreadTimes"
);
#[cfg(feature = "System+Diagnostics+ProcessThreadTimes")]
impl std::ops::Deref for crate::System::Diagnostics::ProcessThreadTimes {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+ProcessThreadTimes")]
impl std::ops::DerefMut for crate::System::Diagnostics::ProcessThreadTimes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+ProcessThreadTimes")]
impl crate::System::Diagnostics::ProcessThreadTimes {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TotalProcessorTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke("get_TotalProcessorTime", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Diagnostics+ProcessThreadTimes")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Diagnostics::ProcessThreadTimes {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
