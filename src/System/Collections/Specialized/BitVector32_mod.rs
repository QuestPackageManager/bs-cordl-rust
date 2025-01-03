#[cfg(feature = "System+Collections+Specialized+BitVector32")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BitVector32 {
    pub _data: u32,
}
#[cfg(feature = "System+Collections+Specialized+BitVector32")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Collections::Specialized::BitVector32 =>
    "System.Collections.Specialized"."BitVector32"
);
#[cfg(feature = "System+Collections+Specialized+BitVector32")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Collections::Specialized::BitVector32 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Collections+Specialized+BitVector32")]
impl crate::System::Collections::Specialized::BitVector32 {
    pub fn CreateMask_0() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateMask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateMask_i32_1(previous: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateMask", (previous))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (o),
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
    pub fn ToString_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_BitVector32_0(
        value: crate::System::Collections::Specialized::BitVector32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(&mut self, bit: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (bit),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Item(
        &mut self,
        bit: i32,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Item",
            (bit, value),
        )?;
        Ok(__cordl_ret.into())
    }
}
