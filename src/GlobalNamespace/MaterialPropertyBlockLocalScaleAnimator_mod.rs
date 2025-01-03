#[cfg(feature = "MaterialPropertyBlockLocalScaleAnimator")]
#[repr(C)]
#[derive(Debug)]
pub struct MaterialPropertyBlockLocalScaleAnimator {
    __cordl_parent: crate::GlobalNamespace::MaterialPropertyBlockAnimator,
    pub _targetTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
}
#[cfg(feature = "MaterialPropertyBlockLocalScaleAnimator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MaterialPropertyBlockLocalScaleAnimator => ""
    ."MaterialPropertyBlockLocalScaleAnimator"
);
#[cfg(feature = "MaterialPropertyBlockLocalScaleAnimator")]
impl std::ops::Deref
for crate::GlobalNamespace::MaterialPropertyBlockLocalScaleAnimator {
    type Target = crate::GlobalNamespace::MaterialPropertyBlockAnimator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialPropertyBlockLocalScaleAnimator")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MaterialPropertyBlockLocalScaleAnimator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialPropertyBlockLocalScaleAnimator")]
impl crate::GlobalNamespace::MaterialPropertyBlockLocalScaleAnimator {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetProperty", ())?;
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
#[cfg(feature = "MaterialPropertyBlockLocalScaleAnimator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MaterialPropertyBlockLocalScaleAnimator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
