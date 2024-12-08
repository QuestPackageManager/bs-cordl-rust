#[cfg(feature = "FlyingScoreSpawner")]
#[repr(C)]
#[derive(Debug)]
pub struct FlyingScoreSpawner {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _flyingScoreEffectPool: *mut crate::GlobalNamespace::FlyingScoreEffect_Pool,
    pub _initData: *mut crate::GlobalNamespace::FlyingScoreSpawner_InitData,
}
#[cfg(feature = "FlyingScoreSpawner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FlyingScoreSpawner => ""."FlyingScoreSpawner"
);
#[cfg(feature = "FlyingScoreSpawner")]
impl std::ops::Deref for FlyingScoreSpawner {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingScoreSpawner")]
impl std::ops::DerefMut for FlyingScoreSpawner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingScoreSpawner")]
impl FlyingScoreSpawner {
    #[cfg(feature = "FlyingScoreSpawner+_SpawnFlyingScoreNextFrameCoroutine_d__5")]
    pub type _SpawnFlyingScoreNextFrameCoroutine_d__5 = crate::GlobalNamespace::FlyingScoreSpawner__SpawnFlyingScoreNextFrameCoroutine_d__5;
    #[cfg(feature = "FlyingScoreSpawner+SpawnPosition")]
    pub type SpawnPosition = crate::GlobalNamespace::FlyingScoreSpawner_SpawnPosition;
    #[cfg(feature = "FlyingScoreSpawner+InitData")]
    pub type InitData = crate::GlobalNamespace::FlyingScoreSpawner_InitData;
    pub fn HandleFlyingObjectEffectDidFinish(
        &mut self,
        flyingObjectEffect: *mut FlyingObjectEffect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleFlyingObjectEffectDidFinish", (flyingObjectEffect))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SpawnFlyingScore(
        &mut self,
        cutScoreBuffer: *mut IReadonlyCutScoreBuffer,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SpawnFlyingScore", (cutScoreBuffer, color))?;
        Ok(__cordl_ret)
    }
    pub fn SpawnFlyingScoreNextFrame(
        &mut self,
        cutScoreBuffer: *mut IReadonlyCutScoreBuffer,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SpawnFlyingScoreNextFrame", (cutScoreBuffer, color))?;
        Ok(__cordl_ret)
    }
    pub fn SpawnFlyingScoreNextFrameCoroutine(
        &mut self,
        cutScoreBuffer: *mut IReadonlyCutScoreBuffer,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("SpawnFlyingScoreNextFrameCoroutine", (cutScoreBuffer, color))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "FlyingScoreSpawner")]
impl quest_hook::libil2cpp::ObjectType for FlyingScoreSpawner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "FlyingScoreSpawner+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct FlyingScoreSpawner_InitData {
    __cordl_parent: crate::System::Object,
    pub spawnPosition: crate::GlobalNamespace::FlyingScoreSpawner_SpawnPosition,
}
#[cfg(feature = "FlyingScoreSpawner+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FlyingScoreSpawner_InitData =>
    ""."FlyingScoreSpawner/InitData"
);
#[cfg(feature = "FlyingScoreSpawner+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::FlyingScoreSpawner_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingScoreSpawner+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::FlyingScoreSpawner_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingScoreSpawner+InitData")]
impl crate::GlobalNamespace::FlyingScoreSpawner_InitData {
    pub fn New(
        spawnPosition: crate::GlobalNamespace::FlyingScoreSpawner_SpawnPosition,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (spawnPosition))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        spawnPosition: crate::GlobalNamespace::FlyingScoreSpawner_SpawnPosition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (spawnPosition))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "FlyingScoreSpawner+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FlyingScoreSpawner_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "FlyingScoreSpawner+SpawnPosition")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlyingScoreSpawner_SpawnPosition {
    AboveGround = 1i32,
    Underground = 0i32,
}
#[cfg(feature = "FlyingScoreSpawner+SpawnPosition")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::FlyingScoreSpawner_SpawnPosition => ""
    ."FlyingScoreSpawner/SpawnPosition"
);