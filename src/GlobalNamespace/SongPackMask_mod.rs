#[cfg(feature = "SongPackMask")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SongPackMask {
    pub _bloomFilter: crate::GlobalNamespace::BitMask256,
}
#[cfg(feature = "SongPackMask")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SongPackMask => ""
    ."SongPackMask"
);
#[cfg(feature = "SongPackMask")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::SongPackMask {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "SongPackMask")]
impl crate::GlobalNamespace::SongPackMask {
    pub const kHashBits: i32 = 15i32;
    pub const kHashCount: i32 = 2i32;
    pub const kToStringPrefix: &'static str = "[SongPackMask ";
    pub const kToStringSuffix: &'static str = "]";
    pub fn Contains(
        &mut self,
        other: crate::GlobalNamespace::SongPackMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Contains",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CreateFromSerializedData(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SongPackMask> {
        let __cordl_ret: crate::GlobalNamespace::SongPackMask = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CreateFromSerializedData",
            (reader),
        )?;
        Ok(__cordl_ret)
    }
    pub fn DifferenceFrom(
        &mut self,
        other: crate::GlobalNamespace::SongPackMask,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DifferenceFrom",
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
    pub fn Equals_SongPackMask0(
        &mut self,
        other: crate::GlobalNamespace::SongPackMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
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
    pub fn _ctor_BitMask256_1(
        &mut self,
        bloomFilter: crate::GlobalNamespace::BitMask256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (bloomFilter),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IEnumerable_1_2(
        &mut self,
        packs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (packs),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String0(
        &mut self,
        packId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (packId),
        )?;
        Ok(__cordl_ret)
    }
}
