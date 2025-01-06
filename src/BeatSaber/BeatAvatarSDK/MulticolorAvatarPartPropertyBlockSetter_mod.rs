#[cfg(feature = "BeatSaber+BeatAvatarSDK+MulticolorAvatarPartPropertyBlockSetter")]
#[repr(C)]
#[derive(Debug)]
pub struct MulticolorAvatarPartPropertyBlockSetter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _colorDataList: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::BeatSaber::BeatAvatarSDK::MulticolorAvatarPartPropertyBlockSetter_ColorData,
            >,
        >,
    >,
    pub _renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
    pub _editInPlayMode: bool,
    pub _colors: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    >,
    pub _rimLightColors: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    >,
    pub _boostColors: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
    >,
    pub _highlighted: bool,
    pub _uvSegment: i32,
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+MulticolorAvatarPartPropertyBlockSetter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::BeatAvatarSDK::MulticolorAvatarPartPropertyBlockSetter =>
    "BeatSaber.BeatAvatarSDK"."MulticolorAvatarPartPropertyBlockSetter"
);
#[cfg(feature = "BeatSaber+BeatAvatarSDK+MulticolorAvatarPartPropertyBlockSetter")]
impl std::ops::Deref
for crate::BeatSaber::BeatAvatarSDK::MulticolorAvatarPartPropertyBlockSetter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+MulticolorAvatarPartPropertyBlockSetter")]
impl std::ops::DerefMut
for crate::BeatSaber::BeatAvatarSDK::MulticolorAvatarPartPropertyBlockSetter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+MulticolorAvatarPartPropertyBlockSetter")]
impl crate::BeatSaber::BeatAvatarSDK::MulticolorAvatarPartPropertyBlockSetter {
    #[cfg(
        feature = "BeatSaber+BeatAvatarSDK+MulticolorAvatarPartPropertyBlockSetter+ColorData"
    )]
    pub type ColorData = crate::BeatSaber::BeatAvatarSDK::MulticolorAvatarPartPropertyBlockSetter_ColorData;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnValidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnValidate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColors(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColors", (colors))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetHighlight(
        &mut self,
        highlighted: bool,
        uvSegment: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHighlight", (highlighted, uvSegment))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateRenderer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateRenderer", ())?;
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
#[cfg(feature = "BeatSaber+BeatAvatarSDK+MulticolorAvatarPartPropertyBlockSetter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarSDK::MulticolorAvatarPartPropertyBlockSetter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatSaber+BeatAvatarSDK+MulticolorAvatarPartPropertyBlockSetter+ColorData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct MulticolorAvatarPartPropertyBlockSetter_ColorData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _defaultColor: crate::UnityEngine::Color,
    pub _darkerColorMultiplier: f32,
    pub _whiteBoost: f32,
}
#[cfg(
    feature = "BeatSaber+BeatAvatarSDK+MulticolorAvatarPartPropertyBlockSetter+ColorData"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::BeatAvatarSDK::MulticolorAvatarPartPropertyBlockSetter_ColorData =>
    "BeatSaber.BeatAvatarSDK"."MulticolorAvatarPartPropertyBlockSetter/ColorData"
);
#[cfg(
    feature = "BeatSaber+BeatAvatarSDK+MulticolorAvatarPartPropertyBlockSetter+ColorData"
)]
impl std::ops::Deref
for crate::BeatSaber::BeatAvatarSDK::MulticolorAvatarPartPropertyBlockSetter_ColorData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+BeatAvatarSDK+MulticolorAvatarPartPropertyBlockSetter+ColorData"
)]
impl std::ops::DerefMut
for crate::BeatSaber::BeatAvatarSDK::MulticolorAvatarPartPropertyBlockSetter_ColorData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+BeatAvatarSDK+MulticolorAvatarPartPropertyBlockSetter+ColorData"
)]
impl crate::BeatSaber::BeatAvatarSDK::MulticolorAvatarPartPropertyBlockSetter_ColorData {
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
    pub fn get_darkerColorMultiplier(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_darkerColorMultiplier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_defaultColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_whiteBoost(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_whiteBoost", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "BeatSaber+BeatAvatarSDK+MulticolorAvatarPartPropertyBlockSetter+ColorData"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarSDK::MulticolorAvatarPartPropertyBlockSetter_ColorData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
