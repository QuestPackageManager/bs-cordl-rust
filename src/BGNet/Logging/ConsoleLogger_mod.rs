#[cfg(feature = "BGNet+Logging+ConsoleLogger")]
#[repr(C)]
#[derive(Debug)]
pub struct ConsoleLogger {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGNet+Logging+ConsoleLogger")]
unsafe impl quest_hook::libil2cpp::Type for crate::BGNet::Logging::ConsoleLogger {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGNet.Logging";
    const CLASS_NAME: &'static str = "ConsoleLogger";
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
#[cfg(feature = "BGNet+Logging+ConsoleLogger")]
impl std::ops::Deref for crate::BGNet::Logging::ConsoleLogger {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Logging+ConsoleLogger")]
impl std::ops::DerefMut for crate::BGNet::Logging::ConsoleLogger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Logging+ConsoleLogger")]
impl crate::BGNet::Logging::ConsoleLogger {
    pub fn LogError(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("LogError")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LogError", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (message))
        };
        Ok(__cordl_ret.into())
    }
    pub fn LogException(
        &mut self,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Exception>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("LogException")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LogException", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (exception, message))
        };
        Ok(__cordl_ret.into())
    }
    pub fn LogInfo(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("LogInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LogInfo", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (message))
        };
        Ok(__cordl_ret.into())
    }
    pub fn LogWarning(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("LogWarning")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LogWarning", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (message))
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
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGNet+Logging+ConsoleLogger")]
impl quest_hook::libil2cpp::ObjectType for crate::BGNet::Logging::ConsoleLogger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGNet+Logging+ConsoleLogger")]
impl AsRef<crate::BGNet::Logging::Debug_ILogger>
for crate::BGNet::Logging::ConsoleLogger {
    fn as_ref(&self) -> &crate::BGNet::Logging::Debug_ILogger {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BGNet+Logging+ConsoleLogger")]
impl AsMut<crate::BGNet::Logging::Debug_ILogger>
for crate::BGNet::Logging::ConsoleLogger {
    fn as_mut(&mut self) -> &mut crate::BGNet::Logging::Debug_ILogger {
        unsafe { std::mem::transmute(self) }
    }
}
