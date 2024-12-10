#[cfg(feature = "DeactivateOnInputFocusCapture")]
#[repr(C)]
#[derive(Debug)]
pub struct DeactivateOnInputFocusCapture {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _vrPlatformHelper: *mut crate::GlobalNamespace::IVRPlatformHelper,
}
#[cfg(feature = "DeactivateOnInputFocusCapture")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DeactivateOnInputFocusCapture
    => ""."DeactivateOnInputFocusCapture"
);
#[cfg(feature = "DeactivateOnInputFocusCapture")]
impl std::ops::Deref for crate::GlobalNamespace::DeactivateOnInputFocusCapture {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DeactivateOnInputFocusCapture")]
impl std::ops::DerefMut for crate::GlobalNamespace::DeactivateOnInputFocusCapture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DeactivateOnInputFocusCapture")]
impl crate::GlobalNamespace::DeactivateOnInputFocusCapture {
    pub fn ActivateGameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ActivateGameObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DeactivateGameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeactivateGameObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleInputFocusCaptured(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleInputFocusCaptured", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleInputFocusReleased(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleInputFocusReleased", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        vrPlatformHelper: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IVRPlatformHelper,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (vrPlatformHelper))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "DeactivateOnInputFocusCapture")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DeactivateOnInputFocusCapture {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DeactivateOnInputFocusCapture")]
impl AsRef<crate::GlobalNamespace::IVerboseLogger>
for crate::GlobalNamespace::DeactivateOnInputFocusCapture {
    fn as_ref(&self) -> &crate::GlobalNamespace::IVerboseLogger {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "DeactivateOnInputFocusCapture")]
impl AsMut<crate::GlobalNamespace::IVerboseLogger>
for crate::GlobalNamespace::DeactivateOnInputFocusCapture {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IVerboseLogger {
        unsafe { std::mem::transmute(self) }
    }
}
