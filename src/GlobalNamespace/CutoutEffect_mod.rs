#[cfg(feature = "CutoutEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct CutoutEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _materialPropertyBlockController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MaterialPropertyBlockController,
    >,
    pub _useRandomCutoutOffset: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BoolSO,
    >,
    pub _cutoutOffset: crate::UnityEngine::Vector3,
    pub _randomNoiseTexOffset: crate::UnityEngine::Vector3,
    pub _cutout: f32,
}
#[cfg(feature = "CutoutEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CutoutEffect => ""
    ."CutoutEffect"
);
#[cfg(feature = "CutoutEffect")]
impl std::ops::Deref for crate::GlobalNamespace::CutoutEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CutoutEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::CutoutEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CutoutEffect")]
impl crate::GlobalNamespace::CutoutEffect {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetCutout_Vector3_1(
        &mut self,
        cutout: f32,
        cutoutOffset: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCutout", (cutout, cutoutOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCutout_f32_0(
        &mut self,
        cutout: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCutout", (cutout))?;
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
    pub fn get_useRandomCutoutOffset(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useRandomCutoutOffset", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "CutoutEffect")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CutoutEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
