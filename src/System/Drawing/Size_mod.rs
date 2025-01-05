#[cfg(feature = "System+Drawing+Size")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}
#[cfg(feature = "System+Drawing+Size")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Drawing::Size => "System.Drawing"."Size"
);
#[cfg(feature = "System+Drawing+Size")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Drawing::Size {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Drawing+Size")]
impl crate::System::Drawing::Size {
    pub fn Equals_Gc0(
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
    pub fn Equals_Size1(
        &mut self,
        other: crate::System::Drawing::Size,
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
    pub fn get_Height(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Height",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Width(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Width",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        sz1: crate::System::Drawing::Size,
        sz2: crate::System::Drawing::Size,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (sz1, sz2))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Drawing+Size")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Drawing::Size>>
for crate::System::Drawing::Size {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::Drawing::Size> {
        todo!()
    }
}
#[cfg(feature = "System+Drawing+Size")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Drawing::Size>>
for crate::System::Drawing::Size {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Drawing::Size> {
        todo!()
    }
}
