#[cfg(feature = "ColorSchemeNetSerializable")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ColorSchemeNetSerializable {
    pub saberAColor: ColorNoAlphaSerializable,
    pub saberBColor: ColorNoAlphaSerializable,
    pub obstaclesColor: ColorNoAlphaSerializable,
    pub environmentColor0: ColorNoAlphaSerializable,
    pub environmentColor1: ColorNoAlphaSerializable,
    pub environmentColor0Boost: ColorNoAlphaSerializable,
    pub environmentColor1Boost: ColorNoAlphaSerializable,
}
#[cfg(feature = "ColorSchemeNetSerializable")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for ColorSchemeNetSerializable => ""
    ."ColorSchemeNetSerializable"
);
#[cfg(feature = "ColorSchemeNetSerializable")]
unsafe impl quest_hook::libil2cpp::ThisArgument for ColorSchemeNetSerializable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ColorSchemeNetSerializable")]
impl ColorSchemeNetSerializable {
    pub fn Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Deserialize",
            (reader),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Serialize",
            (writer),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        saberAColor: crate::UnityEngine::Color,
        saberBColor: crate::UnityEngine::Color,
        obstaclesColor: crate::UnityEngine::Color,
        environmentColor0: crate::UnityEngine::Color,
        environmentColor1: crate::UnityEngine::Color,
        environmentColor0Boost: crate::UnityEngine::Color,
        environmentColor1Boost: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                saberAColor,
                saberBColor,
                obstaclesColor,
                environmentColor0,
                environmentColor1,
                environmentColor0Boost,
                environmentColor1Boost,
            ),
        )?;
        Ok(__cordl_ret)
    }
}
