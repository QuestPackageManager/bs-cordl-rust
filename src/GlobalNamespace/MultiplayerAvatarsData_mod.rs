#[cfg(feature = "MultiplayerAvatarsData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MultiplayerAvatarsData {
    pub multiplayerAvatarsData: *mut crate::System::Collections::Generic::List_1<
        MultiplayerAvatarData,
    >,
    pub supportedAvatarTypeIdHashesBloomFilter: BitMask128,
}
#[cfg(feature = "MultiplayerAvatarsData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for MultiplayerAvatarsData => ""."MultiplayerAvatarsData"
);
#[cfg(feature = "MultiplayerAvatarsData")]
unsafe impl quest_hook::libil2cpp::ThisArgument for MultiplayerAvatarsData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "MultiplayerAvatarsData")]
impl MultiplayerAvatarsData {
    pub fn CreateFromSerializedData(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<MultiplayerAvatarsData> {
        let __cordl_ret: MultiplayerAvatarsData = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CreateFromSerializedData",
            (reader),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_MultiplayerAvatarsData0(
        &mut self,
        other: MultiplayerAvatarsData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
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
    pub fn SerializeAvatarsData(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SerializeAvatarsData",
            (writer),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_BitMask128_1(
        &mut self,
        multiplayerAvatarsData: *mut crate::System::Collections::Generic::List_1<
            MultiplayerAvatarData,
        >,
        supportedAvatarTypeIdHashesBloomFilter: BitMask128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (multiplayerAvatarsData, supportedAvatarTypeIdHashesBloomFilter),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IEnumerable_1_0(
        &mut self,
        multiplayerAvatarsData: *mut crate::System::Collections::Generic::List_1<
            MultiplayerAvatarData,
        >,
        supportedAvatarTypeIdHashes: *mut crate::System::Collections::Generic::IEnumerable_1<
            u32,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (multiplayerAvatarsData, supportedAvatarTypeIdHashes),
        )?;
        Ok(__cordl_ret)
    }
}
