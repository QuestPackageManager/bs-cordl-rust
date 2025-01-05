#[cfg(feature = "ColorNoAlphaSerializable")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ColorNoAlphaSerializable {
    pub _color: crate::UnityEngine::Color,
}
#[cfg(feature = "ColorNoAlphaSerializable")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ColorNoAlphaSerializable => ""
    ."ColorNoAlphaSerializable"
);
#[cfg(feature = "ColorNoAlphaSerializable")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::ColorNoAlphaSerializable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ColorNoAlphaSerializable")]
impl crate::GlobalNamespace::ColorNoAlphaSerializable {
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
    pub fn Equals_ColorNoAlphaSerializable0(
        &mut self,
        other: crate::GlobalNamespace::ColorNoAlphaSerializable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
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
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (color),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Color1(
        c: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::ColorNoAlphaSerializable,
    > {
        let __cordl_ret: crate::GlobalNamespace::ColorNoAlphaSerializable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_ColorNoAlphaSerializable0(
        c: crate::GlobalNamespace::ColorNoAlphaSerializable,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (c))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ColorNoAlphaSerializable")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ColorNoAlphaSerializable {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        todo!()
    }
}
#[cfg(feature = "ColorNoAlphaSerializable")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::ColorNoAlphaSerializable {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        todo!()
    }
}
#[cfg(feature = "ColorNoAlphaSerializable")]
impl AsRef<crate::System::IEquatable_1<crate::GlobalNamespace::ColorNoAlphaSerializable>>
for crate::GlobalNamespace::ColorNoAlphaSerializable {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::GlobalNamespace::ColorNoAlphaSerializable> {
        todo!()
    }
}
#[cfg(feature = "ColorNoAlphaSerializable")]
impl AsMut<crate::System::IEquatable_1<crate::GlobalNamespace::ColorNoAlphaSerializable>>
for crate::GlobalNamespace::ColorNoAlphaSerializable {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::GlobalNamespace::ColorNoAlphaSerializable,
    > {
        todo!()
    }
}
