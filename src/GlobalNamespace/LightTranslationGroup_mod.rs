#[cfg(feature = "LightTranslationGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct LightTranslationGroup {
    __cordl_parent: crate::GlobalNamespace::LightTransformGroup_3<
        *mut crate::GlobalNamespace::LightGroupTranslationXTransform,
        *mut crate::GlobalNamespace::LightGroupTranslationYTransform,
        *mut crate::GlobalNamespace::LightGroupTranslationZTransform,
    >,
    pub _xTranslationLimits: crate::UnityEngine::Vector2,
    pub _yTranslationLimits: crate::UnityEngine::Vector2,
    pub _zTranslationLimits: crate::UnityEngine::Vector2,
    pub _xDistributionLimits: crate::UnityEngine::Vector2,
    pub _yDistributionLimits: crate::UnityEngine::Vector2,
    pub _zDistributionLimits: crate::UnityEngine::Vector2,
}
#[cfg(feature = "LightTranslationGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightTranslationGroup => ""
    ."LightTranslationGroup"
);
#[cfg(feature = "LightTranslationGroup")]
impl std::ops::Deref for crate::GlobalNamespace::LightTranslationGroup {
    type Target = crate::GlobalNamespace::LightTransformGroup_3<
        *mut crate::GlobalNamespace::LightGroupTranslationXTransform,
        *mut crate::GlobalNamespace::LightGroupTranslationYTransform,
        *mut crate::GlobalNamespace::LightGroupTranslationZTransform,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightTranslationGroup")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightTranslationGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightTranslationGroup")]
impl crate::GlobalNamespace::LightTranslationGroup {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn __Validate(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("__Validate", ())?;
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
    pub fn get_xDistributionLimits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_xDistributionLimits", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_xTranslationLimits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_xTranslationLimits", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_yDistributionLimits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_yDistributionLimits", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_yTranslationLimits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_yTranslationLimits", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_zDistributionLimits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_zDistributionLimits", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_zTranslationLimits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_zTranslationLimits", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LightTranslationGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightTranslationGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
