#[cfg(feature = "System+UnhandledExceptionEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct UnhandledExceptionEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub _exception: *mut crate::System::Object,
    pub _isTerminating: bool,
}
#[cfg(feature = "System+UnhandledExceptionEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::UnhandledExceptionEventArgs => "System"
    ."UnhandledExceptionEventArgs"
);
#[cfg(feature = "System+UnhandledExceptionEventArgs")]
impl std::ops::Deref for crate::System::UnhandledExceptionEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+UnhandledExceptionEventArgs")]
impl std::ops::DerefMut for crate::System::UnhandledExceptionEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+UnhandledExceptionEventArgs")]
impl crate::System::UnhandledExceptionEventArgs {
    pub fn New(
        exception: *mut crate::System::Object,
        isTerminating: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (exception, isTerminating))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        exception: *mut crate::System::Object,
        isTerminating: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (exception, isTerminating))?;
        Ok(__cordl_ret)
    }
    pub fn get_ExceptionObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_ExceptionObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsTerminating(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsTerminating", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+UnhandledExceptionEventArgs")]
impl quest_hook::libil2cpp::ObjectType for crate::System::UnhandledExceptionEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
