#[cfg(feature = "SpawnRotationChevron")]
#[repr(C)]
#[derive(Debug)]
pub struct SpawnRotationChevron {
    __cordl_parent: crate::GlobalNamespace::LightWithIdMonoBehaviour,
    pub _lights: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::TubeBloomPrePassLight,
        >,
    >,
    pub _color: crate::UnityEngine::Color,
    pub _lightAmount: f32,
}
#[cfg(feature = "SpawnRotationChevron")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SpawnRotationChevron => ""
    ."SpawnRotationChevron"
);
#[cfg(feature = "SpawnRotationChevron")]
impl std::ops::Deref for crate::GlobalNamespace::SpawnRotationChevron {
    type Target = crate::GlobalNamespace::LightWithIdMonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SpawnRotationChevron")]
impl std::ops::DerefMut for crate::GlobalNamespace::SpawnRotationChevron {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SpawnRotationChevron")]
impl crate::GlobalNamespace::SpawnRotationChevron {
    #[cfg(feature = "SpawnRotationChevron+Pool")]
    pub type Pool = crate::GlobalNamespace::SpawnRotationChevron_Pool;
    pub fn ColorWasSet(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ColorWasSet", (color))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetLightAmount(
        &mut self,
        amount: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLightAmount", (amount))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateLights(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateLights", ())?;
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
}
#[cfg(feature = "SpawnRotationChevron")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SpawnRotationChevron {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SpawnRotationChevron+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct SpawnRotationChevron_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::SpawnRotationChevron,
    >,
}
#[cfg(feature = "SpawnRotationChevron+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SpawnRotationChevron_Pool => ""
    ."SpawnRotationChevron/Pool"
);
#[cfg(feature = "SpawnRotationChevron+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::SpawnRotationChevron_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::SpawnRotationChevron,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SpawnRotationChevron+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::SpawnRotationChevron_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SpawnRotationChevron+Pool")]
impl crate::GlobalNamespace::SpawnRotationChevron_Pool {
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
#[cfg(feature = "SpawnRotationChevron+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SpawnRotationChevron_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
