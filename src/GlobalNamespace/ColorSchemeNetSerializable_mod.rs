#[cfg(feature = "ColorSchemeNetSerializable")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ColorSchemeNetSerializable {
    pub saberAColor: crate::GlobalNamespace::ColorNoAlphaSerializable,
    pub saberBColor: crate::GlobalNamespace::ColorNoAlphaSerializable,
    pub obstaclesColor: crate::GlobalNamespace::ColorNoAlphaSerializable,
    pub environmentColor0: crate::GlobalNamespace::ColorNoAlphaSerializable,
    pub environmentColor1: crate::GlobalNamespace::ColorNoAlphaSerializable,
    pub environmentColor0Boost: crate::GlobalNamespace::ColorNoAlphaSerializable,
    pub environmentColor1Boost: crate::GlobalNamespace::ColorNoAlphaSerializable,
}
#[cfg(feature = "ColorSchemeNetSerializable")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ColorSchemeNetSerializable =>
    ""."ColorSchemeNetSerializable"
);
#[cfg(feature = "ColorSchemeNetSerializable")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::ColorSchemeNetSerializable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ColorSchemeNetSerializable")]
impl crate::GlobalNamespace::ColorSchemeNetSerializable {
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Deserialize",
            (reader),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Serialize",
            (writer),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ColorSchemeNetSerializable")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ColorSchemeNetSerializable {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        todo!()
    }
}
#[cfg(feature = "ColorSchemeNetSerializable")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ColorSchemeNetSerializable {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        todo!()
    }
}
