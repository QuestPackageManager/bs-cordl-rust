#[cfg(feature = "System+Drawing+SizeF")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SizeF {
    pub width: f32,
    pub height: f32,
}
#[cfg(feature = "System+Drawing+SizeF")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Drawing::SizeF => "System.Drawing"
    ."SizeF"
);
#[cfg(feature = "System+Drawing+SizeF")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Drawing::SizeF {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Drawing+SizeF")]
impl crate::System::Drawing::SizeF {
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
    pub fn Equals_SizeF1(
        &mut self,
        other: crate::System::Drawing::SizeF,
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
    pub fn get_Height(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Height",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Width(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Width",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        sz1: crate::System::Drawing::SizeF,
        sz2: crate::System::Drawing::SizeF,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (sz1, sz2))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Drawing+SizeF")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Drawing::SizeF>>
for crate::System::Drawing::SizeF {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::Drawing::SizeF> {
        todo!()
    }
}
#[cfg(feature = "System+Drawing+SizeF")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Drawing::SizeF>>
for crate::System::Drawing::SizeF {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Drawing::SizeF> {
        todo!()
    }
}
