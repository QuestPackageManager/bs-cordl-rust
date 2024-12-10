#[cfg(feature = "System+Console")]
#[repr(C)]
#[derive(Debug)]
pub struct Console {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Console")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Console => "System"."Console"
);
#[cfg(feature = "System+Console")]
impl std::ops::Deref for crate::System::Console {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
#[cfg(feature = "System+Console+WindowsConsole")]
#[repr(C)]
#[derive(Debug)]
pub struct Console_WindowsConsole {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Console+WindowsConsole")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Console_WindowsConsole => "System"
    ."Console/WindowsConsole"
);
#[cfg(feature = "System+Console+WindowsConsole")]
impl std::ops::Deref for crate::System::Console_WindowsConsole {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub type WindowsCancelHandler = crate::System::WindowsConsole_Console_WindowsCancelHandler;
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
#[cfg(feature = "System+Console+WindowsConsole+WindowsCancelHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct WindowsConsole_Console_WindowsCancelHandler {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Console+WindowsConsole+WindowsCancelHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::WindowsConsole_Console_WindowsCancelHandler => "System"
    ."Console/WindowsConsole/WindowsCancelHandler"
);
#[cfg(feature = "System+Console+WindowsConsole+WindowsCancelHandler")]
impl std::ops::Deref for crate::System::WindowsConsole_Console_WindowsCancelHandler {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Console+WindowsConsole+WindowsCancelHandler")]
impl std::ops::DerefMut for crate::System::WindowsConsole_Console_WindowsCancelHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Console+WindowsConsole+WindowsCancelHandler")]
impl crate::System::WindowsConsole_Console_WindowsCancelHandler {
    pub fn Invoke(&mut self, keyCode: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (keyCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Console+WindowsConsole+WindowsCancelHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::WindowsConsole_Console_WindowsCancelHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
