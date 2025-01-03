#[cfg(feature = "Vector4Serializable")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Vector4Serializable {
    pub _x: i32,
    pub _y: i32,
    pub _z: i32,
    pub _w: i32,
}
#[cfg(feature = "Vector4Serializable")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Vector4Serializable => ""
    ."Vector4Serializable"
);
#[cfg(feature = "Vector4Serializable")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::Vector4Serializable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Vector4Serializable")]
impl crate::GlobalNamespace::Vector4Serializable {
    pub fn Approximately(
        &mut self,
        other: crate::GlobalNamespace::Vector4Serializable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Approximately",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
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
    pub fn Equals_Vector4Serializable0(
        &mut self,
        other: crate::GlobalNamespace::Vector4Serializable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
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
    pub fn GetSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetSize",
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
    pub fn _ctor_NetDataReader1(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (reader),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Vector4_0(
        &mut self,
        v: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        a: crate::GlobalNamespace::Vector4Serializable,
        b: crate::GlobalNamespace::Vector4Serializable,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::Vector4Serializable> {
        let __cordl_ret: crate::GlobalNamespace::Vector4Serializable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Vector4Serializable0(
        v: crate::GlobalNamespace::Vector4Serializable,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Vector4_1(
        v: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::Vector4Serializable> {
        let __cordl_ret: crate::GlobalNamespace::Vector4Serializable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction(
        a: crate::GlobalNamespace::Vector4Serializable,
        b: crate::GlobalNamespace::Vector4Serializable,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::Vector4Serializable> {
        let __cordl_ret: crate::GlobalNamespace::Vector4Serializable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (a, b))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Vector4Serializable")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::Vector4Serializable {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        todo!()
    }
}
#[cfg(feature = "Vector4Serializable")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::Vector4Serializable {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        todo!()
    }
}
#[cfg(feature = "Vector4Serializable")]
impl AsRef<crate::System::IEquatable_1<crate::GlobalNamespace::Vector4Serializable>>
for crate::GlobalNamespace::Vector4Serializable {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::GlobalNamespace::Vector4Serializable> {
        todo!()
    }
}
#[cfg(feature = "Vector4Serializable")]
impl AsMut<crate::System::IEquatable_1<crate::GlobalNamespace::Vector4Serializable>>
for crate::GlobalNamespace::Vector4Serializable {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::GlobalNamespace::Vector4Serializable> {
        todo!()
    }
}
