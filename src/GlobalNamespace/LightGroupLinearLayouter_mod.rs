#[cfg(feature = "LightGroupLinearLayouter")]
#[repr(C)]
#[derive(Debug)]
pub struct LightGroupLinearLayouter {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LightGroupSubsystem,
    >,
    pub _movementStep: crate::UnityEngine::Vector3,
    pub _defaultRotation: crate::UnityEngine::Vector3,
    pub _startFromCenter: bool,
}
#[cfg(feature = "LightGroupLinearLayouter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightGroupLinearLayouter => ""
    ."LightGroupLinearLayouter"
);
#[cfg(feature = "LightGroupLinearLayouter")]
impl std::ops::Deref for crate::GlobalNamespace::LightGroupLinearLayouter {
    type Target = quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightGroupSubsystem>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightGroupLinearLayouter")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightGroupLinearLayouter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightGroupLinearLayouter")]
impl crate::GlobalNamespace::LightGroupLinearLayouter {
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
}
#[cfg(feature = "LightGroupLinearLayouter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightGroupLinearLayouter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LightGroupLinearLayouter")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IEditTimeValidated>>
for crate::GlobalNamespace::LightGroupLinearLayouter {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IEditTimeValidated> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LightGroupLinearLayouter")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IEditTimeValidated>>
for crate::GlobalNamespace::LightGroupLinearLayouter {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IEditTimeValidated> {
        unsafe { std::mem::transmute(self) }
    }
}
