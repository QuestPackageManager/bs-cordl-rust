#[cfg(feature = "UnityEngine+UIElements+UIR+GradientRemap")]
#[repr(C)]
#[derive(Debug)]
pub struct GradientRemap {
    __cordl_parent: crate::UnityEngine::UIElements::UIR::LinkedPoolItem_1<
        *mut crate::UnityEngine::UIElements::UIR::GradientRemap,
    >,
    pub origIndex: i32,
    pub destIndex: i32,
    pub location: crate::UnityEngine::RectInt,
    pub next: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::GradientRemap,
    >,
    pub atlas: crate::UnityEngine::UIElements::TextureId,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+GradientRemap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::GradientRemap =>
    "UnityEngine.UIElements.UIR"."GradientRemap"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+GradientRemap")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::GradientRemap {
    type Target = crate::UnityEngine::UIElements::UIR::LinkedPoolItem_1<
        *mut crate::UnityEngine::UIElements::UIR::GradientRemap,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+GradientRemap")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::GradientRemap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+GradientRemap")]
impl crate::UnityEngine::UIElements::UIR::GradientRemap {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
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
#[cfg(feature = "UnityEngine+UIElements+UIR+GradientRemap")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::GradientRemap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
