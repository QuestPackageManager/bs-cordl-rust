#[cfg(feature = "SaberModelContainer")]
#[repr(C)]
#[derive(Debug)]
pub struct SaberModelContainer {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _saber: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Saber>,
    pub _saberModelControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SaberModelController,
    >,
    pub _container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SaberModelContainer_InitData,
    >,
}
#[cfg(feature = "SaberModelContainer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SaberModelContainer => ""
    ."SaberModelContainer"
);
#[cfg(feature = "SaberModelContainer")]
impl std::ops::Deref for crate::GlobalNamespace::SaberModelContainer {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SaberModelContainer")]
impl std::ops::DerefMut for crate::GlobalNamespace::SaberModelContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SaberModelContainer")]
impl crate::GlobalNamespace::SaberModelContainer {
    #[cfg(feature = "SaberModelContainer+InitData")]
    pub type InitData = crate::GlobalNamespace::SaberModelContainer_InitData;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "SaberModelContainer")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SaberModelContainer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SaberModelContainer+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct SaberModelContainer_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub trailTintColor: crate::UnityEngine::Color,
}
#[cfg(feature = "SaberModelContainer+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SaberModelContainer_InitData =>
    ""."SaberModelContainer/InitData"
);
#[cfg(feature = "SaberModelContainer+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::SaberModelContainer_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SaberModelContainer+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::SaberModelContainer_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SaberModelContainer+InitData")]
impl crate::GlobalNamespace::SaberModelContainer_InitData {
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Color1(
        trailTintColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (trailTintColor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Color1(
        &mut self,
        trailTintColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (trailTintColor))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SaberModelContainer+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SaberModelContainer_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
