#[cfg(feature = "Vector3Serializable")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Vector3Serializable {
    pub _x: i32,
    pub _y: i32,
    pub _z: i32,
}
#[cfg(feature = "Vector3Serializable")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Vector3Serializable => ""
    ."Vector3Serializable"
);
#[cfg(feature = "Vector3Serializable")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::Vector3Serializable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Vector3Serializable")]
impl crate::GlobalNamespace::Vector3Serializable {
    pub fn Approximately(
        &mut self,
        other: crate::GlobalNamespace::Vector3Serializable,
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
    pub fn Equals_Vector3Serializable0(
        &mut self,
        other: crate::GlobalNamespace::Vector3Serializable,
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
    pub fn _ctor_Vector3_0(
        &mut self,
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        a: crate::GlobalNamespace::Vector3Serializable,
        b: crate::GlobalNamespace::Vector3Serializable,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::Vector3Serializable> {
        let __cordl_ret: crate::GlobalNamespace::Vector3Serializable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Vector3Serializable0(
        v: crate::GlobalNamespace::Vector3Serializable,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Vector3_1(
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::Vector3Serializable> {
        let __cordl_ret: crate::GlobalNamespace::Vector3Serializable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction(
        a: crate::GlobalNamespace::Vector3Serializable,
        b: crate::GlobalNamespace::Vector3Serializable,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::Vector3Serializable> {
        let __cordl_ret: crate::GlobalNamespace::Vector3Serializable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (a, b))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Vector3Serializable")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::Vector3Serializable {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        todo!()
    }
}
#[cfg(feature = "Vector3Serializable")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::Vector3Serializable {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        todo!()
    }
}
#[cfg(feature = "Vector3Serializable")]
impl AsRef<crate::System::IEquatable_1<crate::GlobalNamespace::Vector3Serializable>>
for crate::GlobalNamespace::Vector3Serializable {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::GlobalNamespace::Vector3Serializable> {
        todo!()
    }
}
#[cfg(feature = "Vector3Serializable")]
impl AsMut<crate::System::IEquatable_1<crate::GlobalNamespace::Vector3Serializable>>
for crate::GlobalNamespace::Vector3Serializable {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::GlobalNamespace::Vector3Serializable> {
        todo!()
    }
}
