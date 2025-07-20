#[cfg(feature = "System+WindowsConsoleDriver")]
#[repr(C)]
#[derive(Debug)]
pub struct WindowsConsoleDriver {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub inputHandle: crate::System::IntPtr,
    pub outputHandle: crate::System::IntPtr,
    pub defaultAttribute: i16,
}
#[cfg(feature = "System+WindowsConsoleDriver")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::WindowsConsoleDriver {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "WindowsConsoleDriver";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+WindowsConsoleDriver")]
impl std::ops::Deref for crate::System::WindowsConsoleDriver {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+WindowsConsoleDriver")]
impl std::ops::DerefMut for crate::System::WindowsConsoleDriver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+WindowsConsoleDriver")]
impl crate::System::WindowsConsoleDriver {
    pub fn GetConsoleScreenBufferInfo(
        handle: crate::System::IntPtr,
        info: quest_hook::libil2cpp::ByRefMut<crate::System::ConsoleScreenBufferInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::WindowsConsoleDriver as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::System::ConsoleScreenBufferInfo,
                    >,
                ),
                bool,
                2usize,
            >("GetConsoleScreenBufferInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::WindowsConsoleDriver as quest_hook::libil2cpp::Type >
                    ::class(), "GetConsoleScreenBufferInfo", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (handle, info))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetStdHandle(
        handle: crate::System::Handles,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::WindowsConsoleDriver as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Handles),
                crate::System::IntPtr,
                1usize,
            >("GetStdHandle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::WindowsConsoleDriver as quest_hook::libil2cpp::Type >
                    ::class(), "GetStdHandle", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (handle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsModifierKey(virtualKeyCode: i16) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::WindowsConsoleDriver as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i16), bool, 1usize>("IsModifierKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::WindowsConsoleDriver as quest_hook::libil2cpp::Type >
                    ::class(), "IsModifierKey", 1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (virtualKeyCode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReadConsoleInput(
        handle: crate::System::IntPtr,
        record: quest_hook::libil2cpp::ByRefMut<crate::System::InputRecord>,
        length: i32,
        nread: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::WindowsConsoleDriver as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::ByRefMut<crate::System::InputRecord>,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                4usize,
            >("ReadConsoleInput")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::WindowsConsoleDriver as quest_hook::libil2cpp::Type >
                    ::class(), "ReadConsoleInput", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (handle, record, length, nread))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadKey(
        &mut self,
        intercept: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::ConsoleKeyInfo> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::WindowsConsoleDriver as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), crate::System::ConsoleKeyInfo, 1usize>("ReadKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::WindowsConsoleDriver as quest_hook::libil2cpp::Type >
                    ::class(), "ReadKey", 1usize
                )
            });
        let __cordl_ret: crate::System::ConsoleKeyInfo = unsafe {
            method.invoke_unchecked(self, (intercept))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::WindowsConsoleDriver as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::WindowsConsoleDriver as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+WindowsConsoleDriver")]
impl quest_hook::libil2cpp::ObjectType for crate::System::WindowsConsoleDriver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+WindowsConsoleDriver")]
impl AsRef<crate::System::IConsoleDriver> for crate::System::WindowsConsoleDriver {
    fn as_ref(&self) -> &crate::System::IConsoleDriver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+WindowsConsoleDriver")]
impl AsMut<crate::System::IConsoleDriver> for crate::System::WindowsConsoleDriver {
    fn as_mut(&mut self) -> &mut crate::System::IConsoleDriver {
        unsafe { std::mem::transmute(self) }
    }
}
