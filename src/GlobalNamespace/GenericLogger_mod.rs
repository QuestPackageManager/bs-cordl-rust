#[cfg(feature = "GenericLogger")]
#[repr(C)]
#[derive(Debug)]
pub struct GenericLogger {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "GenericLogger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GenericLogger => ""
    ."GenericLogger"
);
#[cfg(feature = "GenericLogger")]
impl std::ops::Deref for crate::GlobalNamespace::GenericLogger {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GenericLogger")]
impl std::ops::DerefMut for crate::GlobalNamespace::GenericLogger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GenericLogger")]
impl crate::GlobalNamespace::GenericLogger {
    pub const kVerboseLogDefineSymbol: &'static str = "VERBOSE_LOGGING";
    #[cfg(feature = "GenericLogger+ScopedStopwatch")]
    pub type ScopedStopwatch = crate::GlobalNamespace::GenericLogger_ScopedStopwatch;
}
#[cfg(feature = "GenericLogger")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::GenericLogger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GenericLogger+ScopedStopwatch")]
#[repr(C)]
#[derive(Debug)]
pub struct GenericLogger_ScopedStopwatch {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _processName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _stopwatch: *mut crate::System::Diagnostics::Stopwatch,
}
#[cfg(feature = "GenericLogger+ScopedStopwatch")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GenericLogger_ScopedStopwatch
    => ""."GenericLogger/ScopedStopwatch"
);
#[cfg(feature = "GenericLogger+ScopedStopwatch")]
impl std::ops::Deref for crate::GlobalNamespace::GenericLogger_ScopedStopwatch {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GenericLogger+ScopedStopwatch")]
impl std::ops::DerefMut for crate::GlobalNamespace::GenericLogger_ScopedStopwatch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GenericLogger+ScopedStopwatch")]
impl crate::GlobalNamespace::GenericLogger_ScopedStopwatch {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        processName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (processName))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        processName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (processName))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GenericLogger+ScopedStopwatch")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GenericLogger_ScopedStopwatch {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GenericLogger+ScopedStopwatch")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::GenericLogger_ScopedStopwatch {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GenericLogger+ScopedStopwatch")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::GenericLogger_ScopedStopwatch {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
