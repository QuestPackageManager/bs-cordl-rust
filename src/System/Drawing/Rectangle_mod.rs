#[cfg(feature = "System+Drawing+Rectangle")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
#[cfg(feature = "System+Drawing+Rectangle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Drawing::Rectangle => "System.Drawing"
    ."Rectangle"
);
#[cfg(feature = "System+Drawing+Rectangle")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Drawing::Rectangle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Drawing+Rectangle")]
impl crate::System::Drawing::Rectangle {
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
    pub fn Equals_Rectangle1(
        &mut self,
        other: crate::System::Drawing::Rectangle,
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
    pub fn get_X(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_X",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Y(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Y",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: crate::System::Drawing::Rectangle,
        right: crate::System::Drawing::Rectangle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Drawing+Rectangle")]
impl AsRef<crate::System::IEquatable_1<crate::System::Drawing::Rectangle>>
for crate::System::Drawing::Rectangle {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::System::Drawing::Rectangle> {
        todo!()
    }
}
#[cfg(feature = "System+Drawing+Rectangle")]
impl AsMut<crate::System::IEquatable_1<crate::System::Drawing::Rectangle>>
for crate::System::Drawing::Rectangle {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::System::Drawing::Rectangle> {
        todo!()
    }
}
