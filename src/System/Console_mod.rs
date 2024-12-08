#[cfg(feature = "System+Console")]
#[repr(C)]
#[derive(Debug)]
pub struct Console {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Console")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Console => "System"."Console"
);
#[cfg(feature = "System+Console")]
impl std::ops::Deref for crate::System::Console {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Console")]
impl std::ops::DerefMut for crate::System::Console {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Console")]
impl crate::System::Console {
    #[cfg(feature = "System+Console+WindowsConsole")]
    pub type WindowsConsole = crate::System::Console_WindowsConsole;
}
#[cfg(feature = "System+Console")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Console {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Console+WindowsConsole+WindowsCancelHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct WindowsConsole_WindowsCancelHandler {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Console+WindowsConsole+WindowsCancelHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::WindowsConsole_WindowsCancelHandler =>
    "System"."Console/WindowsConsole/WindowsCancelHandler"
);
#[cfg(feature = "System+Console+WindowsConsole+WindowsCancelHandler")]
impl std::ops::Deref for crate::System::WindowsConsole_WindowsCancelHandler {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Console+WindowsConsole+WindowsCancelHandler")]
impl std::ops::DerefMut for crate::System::WindowsConsole_WindowsCancelHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Console+WindowsConsole+WindowsCancelHandler")]
impl crate::System::WindowsConsole_WindowsCancelHandler {
    pub fn Invoke(&mut self, keyCode: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (keyCode))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Console+WindowsConsole+WindowsCancelHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::WindowsConsole_WindowsCancelHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Console+WindowsConsole")]
#[repr(C)]
#[derive(Debug)]
pub struct Console_WindowsConsole {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Console+WindowsConsole")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Console_WindowsConsole => "System"
    ."Console/WindowsConsole"
);
#[cfg(feature = "System+Console+WindowsConsole")]
impl std::ops::Deref for crate::System::Console_WindowsConsole {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Console+WindowsConsole")]
impl std::ops::DerefMut for crate::System::Console_WindowsConsole {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Console+WindowsConsole")]
impl crate::System::Console_WindowsConsole {
    #[cfg(feature = "System+Console+WindowsConsole+WindowsCancelHandler")]
    pub type WindowsCancelHandler = crate::System::WindowsConsole_WindowsCancelHandler;
}
#[cfg(feature = "System+Console+WindowsConsole")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Console_WindowsConsole {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
