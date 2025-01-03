#[cfg(feature = "System+Drawing+Color")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Color {
    pub name: *mut quest_hook::libil2cpp::Il2CppString,
    pub value: i64,
    pub knownColor: i16,
    pub state: i16,
}
#[cfg(feature = "System+Drawing+Color")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Drawing::Color => "System.Drawing"
    ."Color"
);
#[cfg(feature = "System+Drawing+Color")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Drawing::Color {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Drawing+Color")]
impl crate::System::Drawing::Color {
    pub fn Equals_Color1(
        &mut self,
        other: crate::System::Drawing::Color,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
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
    pub fn get_A(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_A",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_B(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_B",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_G(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_G",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsKnownColor(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsKnownColor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_R(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_R",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: crate::System::Drawing::Color,
        right: crate::System::Drawing::Color,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Drawing+Color")]
impl AsRef<crate::System::IEquatable_1<crate::System::Drawing::Color>>
for crate::System::Drawing::Color {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::System::Drawing::Color> {
        todo!()
    }
}
#[cfg(feature = "System+Drawing+Color")]
impl AsMut<crate::System::IEquatable_1<crate::System::Drawing::Color>>
for crate::System::Drawing::Color {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::System::Drawing::Color> {
        todo!()
    }
}
