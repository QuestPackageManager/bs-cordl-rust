#[cfg(feature = "ColorAlphaVariationSO")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorAlphaVariationSO {
    __cordl_parent: crate::GlobalNamespace::ColorSO,
    pub _baseColor: *mut crate::GlobalNamespace::NoAlphaColorSO,
    pub _alpha: *mut crate::GlobalNamespace::AlphaSO,
}
#[cfg(feature = "ColorAlphaVariationSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ColorAlphaVariationSO => ""
    ."ColorAlphaVariationSO"
);
#[cfg(feature = "ColorAlphaVariationSO")]
impl std::ops::Deref for crate::GlobalNamespace::ColorAlphaVariationSO {
    type Target = crate::GlobalNamespace::ColorSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorAlphaVariationSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::ColorAlphaVariationSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorAlphaVariationSO")]
impl crate::GlobalNamespace::ColorAlphaVariationSO {
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
    pub fn get_alpha(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AlphaSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AlphaSO> = __cordl_object
            .invoke("get_alpha", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_baseColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoAlphaColorSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoAlphaColorSO,
        > = __cordl_object.invoke("get_baseColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ColorAlphaVariationSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ColorAlphaVariationSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
