#[cfg(feature = "DelayedMenuButtonTrigger")]
#[repr(C)]
#[derive(Debug)]
pub struct DelayedMenuButtonTrigger {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub menuButtonTriggeredEvent: *mut crate::System::Action,
    pub _pressDuration: f32,
    pub _timer: f32,
    pub _waitingForButtonRelease: bool,
    pub _vrPlatformHelper: *mut crate::GlobalNamespace::IVRPlatformHelper,
}
#[cfg(feature = "DelayedMenuButtonTrigger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DelayedMenuButtonTrigger => ""
    ."DelayedMenuButtonTrigger"
);
#[cfg(feature = "DelayedMenuButtonTrigger")]
impl std::ops::Deref for crate::GlobalNamespace::DelayedMenuButtonTrigger {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DelayedMenuButtonTrigger")]
impl std::ops::DerefMut for crate::GlobalNamespace::DelayedMenuButtonTrigger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DelayedMenuButtonTrigger")]
impl crate::GlobalNamespace::DelayedMenuButtonTrigger {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
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
    pub fn add_menuButtonTriggeredEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_menuButtonTriggeredEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_menuButtonTriggeredEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_menuButtonTriggeredEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DelayedMenuButtonTrigger")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DelayedMenuButtonTrigger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
