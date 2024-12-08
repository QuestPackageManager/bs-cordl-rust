#[cfg(feature = "System+IConsoleDriver")]
#[repr(C)]
#[derive(Debug)]
pub struct IConsoleDriver {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+IConsoleDriver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IConsoleDriver => "System"
    ."IConsoleDriver"
);
#[cfg(feature = "System+IConsoleDriver")]
impl std::ops::Deref for crate::System::IConsoleDriver {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IConsoleDriver")]
impl std::ops::DerefMut for crate::System::IConsoleDriver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IConsoleDriver")]
impl crate::System::IConsoleDriver {
    pub fn ReadKey(
        &mut self,
        intercept: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::ConsoleKeyInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ConsoleKeyInfo = __cordl_object
            .invoke("ReadKey", (intercept))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+IConsoleDriver")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IConsoleDriver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
