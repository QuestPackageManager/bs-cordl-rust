#[cfg(feature = "LightColorGroupEffectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct LightColorGroupEffectManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _lightGroups: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::LightGroup>,
    >,
    pub _beatmapCallbacksController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController,
    >,
    pub _container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub _colorBoostBeatmapDataCallbackWrapper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    >,
    pub _lightColorGroupEffects: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::LightColorGroupEffect,
        >,
    >,
}
#[cfg(feature = "LightColorGroupEffectManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightColorGroupEffectManager =>
    ""."LightColorGroupEffectManager"
);
#[cfg(feature = "LightColorGroupEffectManager")]
impl std::ops::Deref for crate::GlobalNamespace::LightColorGroupEffectManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightColorGroupEffectManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightColorGroupEffectManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightColorGroupEffectManager")]
impl crate::GlobalNamespace::LightColorGroupEffectManager {
    pub fn HandleColorBoostBeatmapEvent(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorBoostBeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleColorBoostBeatmapEvent", (eventData))?;
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
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
    pub fn get_lightGroups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyCollection_1<
                *mut crate::GlobalNamespace::LightGroup,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyCollection_1<
                *mut crate::GlobalNamespace::LightGroup,
            >,
        > = __cordl_object.invoke("get_lightGroups", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightColorGroupEffectManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightColorGroupEffectManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
