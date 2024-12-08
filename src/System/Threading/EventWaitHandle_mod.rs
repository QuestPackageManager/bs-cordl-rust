#[cfg(feature = "System+Threading+EventWaitHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct EventWaitHandle {
    __cordl_parent: crate::System::Threading::WaitHandle,
}
#[cfg(feature = "System+Threading+EventWaitHandle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::EventWaitHandle =>
    "System.Threading"."EventWaitHandle"
);
#[cfg(feature = "System+Threading+EventWaitHandle")]
impl std::ops::Deref for crate::System::Threading::EventWaitHandle {
    type Target = crate::System::Threading::WaitHandle;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+EventWaitHandle")]
impl std::ops::DerefMut for crate::System::Threading::EventWaitHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+EventWaitHandle")]
impl crate::System::Threading::EventWaitHandle {
    pub fn Reset(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool_EventResetMode0(
        &mut self,
        initialState: bool,
        mode: crate::System::Threading::EventResetMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialState, mode))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        initialState: bool,
        mode: crate::System::Threading::EventResetMode,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialState, mode, name))?;
        Ok(__cordl_ret)
    }
    pub fn Set(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Set", ())?;
        Ok(__cordl_ret)
    }
    pub fn New__cordl_bool_EventResetMode0(
        initialState: bool,
        mode: crate::System::Threading::EventResetMode,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialState, mode))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        initialState: bool,
        mode: crate::System::Threading::EventResetMode,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialState, mode, name))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Threading+EventWaitHandle")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::EventWaitHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
