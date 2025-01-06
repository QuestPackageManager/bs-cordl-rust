#[cfg(feature = "MultiplayerAvatarsData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MultiplayerAvatarsData {
    pub multiplayerAvatarsData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::MultiplayerAvatarData,
        >,
    >,
    pub supportedAvatarTypeIdHashesBloomFilter: crate::GlobalNamespace::BitMask128,
}
#[cfg(feature = "MultiplayerAvatarsData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerAvatarsData => ""
    ."MultiplayerAvatarsData"
);
#[cfg(feature = "MultiplayerAvatarsData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::MultiplayerAvatarsData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "MultiplayerAvatarsData")]
impl crate::GlobalNamespace::MultiplayerAvatarsData {
    pub fn CreateFromSerializedData(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::MultiplayerAvatarsData> {
        let __cordl_ret: crate::GlobalNamespace::MultiplayerAvatarsData = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CreateFromSerializedData",
            (reader),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Deserialize(
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::MultiplayerAvatarsData> {
        let __cordl_ret: crate::GlobalNamespace::MultiplayerAvatarsData = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeAvatarsData(
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::MultiplayerAvatarData,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::MultiplayerAvatarData,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeAvatarsData", (reader))?;
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
    pub fn Equals_MultiplayerAvatarsData0(
        &mut self,
        other: crate::GlobalNamespace::MultiplayerAvatarsData,
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
    pub fn SerializeAvatarsData(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SerializeAvatarsData",
            (writer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SupportsOptionalAvatarDataType(
        &mut self,
        typeHash: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SupportsOptionalAvatarDataType",
            (typeHash),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_BitMask128_1(
        &mut self,
        multiplayerAvatarsData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::MultiplayerAvatarData,
            >,
        >,
        supportedAvatarTypeIdHashesBloomFilter: crate::GlobalNamespace::BitMask128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (multiplayerAvatarsData, supportedAvatarTypeIdHashesBloomFilter),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IEnumerable_1_0(
        &mut self,
        multiplayerAvatarsData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::MultiplayerAvatarData,
            >,
        >,
        supportedAvatarTypeIdHashes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<u32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (multiplayerAvatarsData, supportedAvatarTypeIdHashes),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerAvatarsData")]
impl AsRef<
    crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        crate::GlobalNamespace::MultiplayerAvatarsData,
    >,
> for crate::GlobalNamespace::MultiplayerAvatarsData {
    fn as_ref(
        &self,
    ) -> &crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        crate::GlobalNamespace::MultiplayerAvatarsData,
    > {
        todo!()
    }
}
#[cfg(feature = "MultiplayerAvatarsData")]
impl AsMut<
    crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        crate::GlobalNamespace::MultiplayerAvatarsData,
    >,
> for crate::GlobalNamespace::MultiplayerAvatarsData {
    fn as_mut(
        &mut self,
    ) -> &mut crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        crate::GlobalNamespace::MultiplayerAvatarsData,
    > {
        todo!()
    }
}
#[cfg(feature = "MultiplayerAvatarsData")]
impl AsRef<crate::System::IEquatable_1<crate::GlobalNamespace::MultiplayerAvatarsData>>
for crate::GlobalNamespace::MultiplayerAvatarsData {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::GlobalNamespace::MultiplayerAvatarsData> {
        todo!()
    }
}
#[cfg(feature = "MultiplayerAvatarsData")]
impl AsMut<crate::System::IEquatable_1<crate::GlobalNamespace::MultiplayerAvatarsData>>
for crate::GlobalNamespace::MultiplayerAvatarsData {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::GlobalNamespace::MultiplayerAvatarsData,
    > {
        todo!()
    }
}
