#[cfg(feature = "PlayerStateHash")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PlayerStateHash {
    pub _bloomFilter: BitMask128,
}
#[cfg(feature = "PlayerStateHash")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for PlayerStateHash => ""."PlayerStateHash"
);
#[cfg(feature = "PlayerStateHash")]
unsafe impl quest_hook::libil2cpp::ThisArgument for PlayerStateHash {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PlayerStateHash")]
impl PlayerStateHash {
    pub const kToStringPrefix: &'static str = "[PlayerStateMask ";
    pub const kToStringSuffix: &'static str = "]";
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
    pub fn CreateFromSerializedData(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<PlayerStateHash> {
        let __cordl_ret: PlayerStateHash = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CreateFromSerializedData",
            (reader),
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
    pub fn _ctor_BitMask128_0(
        &mut self,
        bloomFilter: BitMask128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (bloomFilter),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        state: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (state),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_HashSet_1_2(
        &mut self,
        stateHashSet: *mut crate::System::Collections::Generic::HashSet_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (stateHashSet),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToShortString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToShortString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_PlayerStateHash0(
        &mut self,
        other: PlayerStateHash,
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
    pub fn AddState(
        &mut self,
        state: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<PlayerStateHash> {
        let __cordl_ret: PlayerStateHash = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddState",
            (state),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToBytes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToBytes",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Contains(
        &mut self,
        state: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Contains",
            (state),
        )?;
        Ok(__cordl_ret)
    }
}
