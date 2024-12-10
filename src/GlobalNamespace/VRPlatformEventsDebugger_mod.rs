#[cfg(feature = "VRPlatformEventsDebugger")]
#[repr(C)]
#[derive(Debug)]
pub struct VRPlatformEventsDebugger {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _vrPlatformHelper: *mut crate::GlobalNamespace::IVRPlatformHelper,
}
#[cfg(feature = "VRPlatformEventsDebugger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::VRPlatformEventsDebugger => ""
    ."VRPlatformEventsDebugger"
);
#[cfg(feature = "VRPlatformEventsDebugger")]
impl std::ops::Deref for crate::GlobalNamespace::VRPlatformEventsDebugger {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRPlatformEventsDebugger")]
impl std::ops::DerefMut for crate::GlobalNamespace::VRPlatformEventsDebugger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRPlatformEventsDebugger")]
impl crate::GlobalNamespace::VRPlatformEventsDebugger {
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
    pub fn HandleHMDMounted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleHMDMounted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleHMDUnmounted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleHMDUnmounted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleInputFocusWasCaptured(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleInputFocusWasCaptured", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleInputFocusWasReleased(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleInputFocusWasReleased", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleVRFocusWasCaptured(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleVRFocusWasCaptured", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleVRFocusWasReleased(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleVRFocusWasReleased", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_loggerPrefix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_loggerPrefix", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VRPlatformEventsDebugger")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::VRPlatformEventsDebugger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "VRPlatformEventsDebugger")]
impl AsRef<crate::GlobalNamespace::IVerboseLogger>
for crate::GlobalNamespace::VRPlatformEventsDebugger {
    fn as_ref(&self) -> &crate::GlobalNamespace::IVerboseLogger {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "VRPlatformEventsDebugger")]
impl AsMut<crate::GlobalNamespace::IVerboseLogger>
for crate::GlobalNamespace::VRPlatformEventsDebugger {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IVerboseLogger {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "VRPlatformEventsDebugger")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::VRPlatformEventsDebugger {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "VRPlatformEventsDebugger")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::VRPlatformEventsDebugger {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "VRPlatformEventsDebugger")]
impl AsRef<crate::Zenject::IInitializable>
for crate::GlobalNamespace::VRPlatformEventsDebugger {
    fn as_ref(&self) -> &crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "VRPlatformEventsDebugger")]
impl AsMut<crate::Zenject::IInitializable>
for crate::GlobalNamespace::VRPlatformEventsDebugger {
    fn as_mut(&mut self) -> &mut crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
