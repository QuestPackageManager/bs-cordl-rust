#[cfg(feature = "InstantMenuButtonTrigger")]
#[repr(C)]
#[derive(Debug)]
pub struct InstantMenuButtonTrigger {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _vrPlatformHelper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IVRPlatformHelper,
    >,
    pub menuButtonTriggeredEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
}
#[cfg(feature = "InstantMenuButtonTrigger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::InstantMenuButtonTrigger => ""
    ."InstantMenuButtonTrigger"
);
#[cfg(feature = "InstantMenuButtonTrigger")]
impl std::ops::Deref for crate::GlobalNamespace::InstantMenuButtonTrigger {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "InstantMenuButtonTrigger")]
impl std::ops::DerefMut for crate::GlobalNamespace::InstantMenuButtonTrigger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "InstantMenuButtonTrigger")]
impl crate::GlobalNamespace::InstantMenuButtonTrigger {
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
#[cfg(feature = "InstantMenuButtonTrigger")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::InstantMenuButtonTrigger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "InstantMenuButtonTrigger")]
impl AsRef<crate::GlobalNamespace::IMenuButtonTrigger>
for crate::GlobalNamespace::InstantMenuButtonTrigger {
    fn as_ref(&self) -> &crate::GlobalNamespace::IMenuButtonTrigger {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "InstantMenuButtonTrigger")]
impl AsMut<crate::GlobalNamespace::IMenuButtonTrigger>
for crate::GlobalNamespace::InstantMenuButtonTrigger {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IMenuButtonTrigger {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "InstantMenuButtonTrigger")]
impl AsRef<crate::Zenject::ITickable>
for crate::GlobalNamespace::InstantMenuButtonTrigger {
    fn as_ref(&self) -> &crate::Zenject::ITickable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "InstantMenuButtonTrigger")]
impl AsMut<crate::Zenject::ITickable>
for crate::GlobalNamespace::InstantMenuButtonTrigger {
    fn as_mut(&mut self) -> &mut crate::Zenject::ITickable {
        unsafe { std::mem::transmute(self) }
    }
}
