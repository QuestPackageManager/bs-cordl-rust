#[cfg(feature = "Color32Serializable")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Color32Serializable {
    pub _color: crate::UnityEngine::Color32,
}
#[cfg(feature = "Color32Serializable")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Color32Serializable => ""
    ."Color32Serializable"
);
#[cfg(feature = "Color32Serializable")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::Color32Serializable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Color32Serializable")]
impl crate::GlobalNamespace::Color32Serializable {
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
    pub fn Equals_Color32Serializable0(
        &mut self,
        other: crate::GlobalNamespace::Color32Serializable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Gc1(
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
        color: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (color),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Color32Serializable0(
        c: crate::GlobalNamespace::Color32Serializable,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_ret: crate::UnityEngine::Color32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Color32_1(
        c: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::Color32Serializable> {
        let __cordl_ret: crate::GlobalNamespace::Color32Serializable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (c))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Color32Serializable")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Color32Serializable>>
for crate::GlobalNamespace::Color32Serializable {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Color32Serializable> {
        todo!()
    }
}
#[cfg(feature = "Color32Serializable")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Color32Serializable>>
for crate::GlobalNamespace::Color32Serializable {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Color32Serializable> {
        todo!()
    }
}
#[cfg(feature = "Color32Serializable")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::INetSerializable>>
for crate::GlobalNamespace::Color32Serializable {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::INetSerializable> {
        todo!()
    }
}
#[cfg(feature = "Color32Serializable")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::INetSerializable>>
for crate::GlobalNamespace::Color32Serializable {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::INetSerializable> {
        todo!()
    }
}
