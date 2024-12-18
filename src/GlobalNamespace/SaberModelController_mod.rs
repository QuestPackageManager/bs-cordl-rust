#[cfg(feature = "SaberModelController")]
#[repr(C)]
#[derive(Debug)]
pub struct SaberModelController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _saberTrail: *mut crate::GlobalNamespace::SaberTrail,
    pub _setSaberGlowColors: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::SetSaberGlowColor,
    >,
    pub _setSaberFakeGlowColors: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::SetSaberFakeGlowColor,
    >,
    pub _saberLight: *mut crate::GlobalNamespace::TubeBloomPrePassLight,
    pub _initData: *mut crate::GlobalNamespace::SaberModelController_InitData,
    pub _colorManager: *mut crate::GlobalNamespace::ColorManager,
}
#[cfg(feature = "SaberModelController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SaberModelController => ""
    ."SaberModelController"
);
#[cfg(feature = "SaberModelController")]
impl std::ops::Deref for crate::GlobalNamespace::SaberModelController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SaberModelController")]
impl std::ops::DerefMut for crate::GlobalNamespace::SaberModelController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SaberModelController")]
impl crate::GlobalNamespace::SaberModelController {
    #[cfg(feature = "SaberModelController+InitData")]
    pub type InitData = crate::GlobalNamespace::SaberModelController_InitData;
    pub fn Init(
        &mut self,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        saber: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Saber>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (parent, saber))?;
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
}
#[cfg(feature = "SaberModelController")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SaberModelController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SaberModelController+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct SaberModelController_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub trailTintColor: crate::UnityEngine::Color,
}
#[cfg(feature = "SaberModelController+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SaberModelController_InitData
    => ""."SaberModelController/InitData"
);
#[cfg(feature = "SaberModelController+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::SaberModelController_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SaberModelController+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::SaberModelController_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SaberModelController+InitData")]
impl crate::GlobalNamespace::SaberModelController_InitData {
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
#[cfg(feature = "SaberModelController+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SaberModelController_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
