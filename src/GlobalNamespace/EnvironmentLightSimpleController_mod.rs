#[cfg(feature = "EnvironmentLightSimpleController")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentLightSimpleController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _color: crate::UnityEngine::Color,
    pub _colorId: i32,
    pub _lightManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LightWithIdManager,
    >,
}
#[cfg(feature = "EnvironmentLightSimpleController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::EnvironmentLightSimpleController => ""
    ."EnvironmentLightSimpleController"
);
#[cfg(feature = "EnvironmentLightSimpleController")]
impl std::ops::Deref for crate::GlobalNamespace::EnvironmentLightSimpleController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentLightSimpleController")]
impl std::ops::DerefMut for crate::GlobalNamespace::EnvironmentLightSimpleController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentLightSimpleController")]
impl crate::GlobalNamespace::EnvironmentLightSimpleController {
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
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
#[cfg(feature = "EnvironmentLightSimpleController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EnvironmentLightSimpleController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
