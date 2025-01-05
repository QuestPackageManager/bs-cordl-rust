#[cfg(feature = "SyncStateId")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SyncStateId {
    pub _id: u8,
}
#[cfg(feature = "SyncStateId")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SyncStateId => ""."SyncStateId"
);
#[cfg(feature = "SyncStateId")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::GlobalNamespace::SyncStateId {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "SyncStateId")]
impl crate::GlobalNamespace::SyncStateId {
    pub const kMaxValue: u8 = 128u8;
    pub fn Before(
        &mut self,
        other: crate::GlobalNamespace::SyncStateId,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Before",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateFromSerializedData(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SyncStateId> {
        let __cordl_ret: crate::GlobalNamespace::SyncStateId = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CreateFromSerializedData",
            (reader),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Deserialize(
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SyncStateId> {
        let __cordl_ret: crate::GlobalNamespace::SyncStateId = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeWithFlag(
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
        flag: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SyncStateId> {
        let __cordl_ret: crate::GlobalNamespace::SyncStateId = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeWithFlag", (reader, flag))?;
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
    pub fn Equals_SyncStateId0(
        &mut self,
        other: crate::GlobalNamespace::SyncStateId,
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
    pub fn Increment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SyncStateId> {
        let __cordl_ret: crate::GlobalNamespace::SyncStateId = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Increment",
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
    pub fn SerializeWithFlag(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        flag: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SerializeWithFlag",
            (writer, flag),
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
        id: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (id),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        a: crate::GlobalNamespace::SyncStateId,
        b: crate::GlobalNamespace::SyncStateId,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        a: crate::GlobalNamespace::SyncStateId,
        b: crate::GlobalNamespace::SyncStateId,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (a, b))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SyncStateId")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SyncStateId>>
for crate::GlobalNamespace::SyncStateId {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SyncStateId> {
        todo!()
    }
}
#[cfg(feature = "SyncStateId")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SyncStateId>>
for crate::GlobalNamespace::SyncStateId {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SyncStateId> {
        todo!()
    }
}
#[cfg(feature = "SyncStateId")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SyncStateId>>
for crate::GlobalNamespace::SyncStateId {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SyncStateId> {
        todo!()
    }
}
#[cfg(feature = "SyncStateId")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SyncStateId>>
for crate::GlobalNamespace::SyncStateId {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SyncStateId> {
        todo!()
    }
}
