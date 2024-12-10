#[cfg(feature = "UnityEngine+UIElements+BackgroundRepeat")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BackgroundRepeat {
    pub x: crate::UnityEngine::UIElements::Repeat,
    pub y: crate::UnityEngine::UIElements::Repeat,
}
#[cfg(feature = "UnityEngine+UIElements+BackgroundRepeat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::BackgroundRepeat =>
    "UnityEngine.UIElements"."BackgroundRepeat"
);
#[cfg(feature = "UnityEngine+UIElements+BackgroundRepeat")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::BackgroundRepeat {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BackgroundRepeat")]
impl crate::UnityEngine::UIElements::BackgroundRepeat {
    pub fn Equals_BackgroundRepeat1(
        &mut self,
        other: crate::UnityEngine::UIElements::BackgroundRepeat,
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
    pub fn _ctor(
        &mut self,
        repeatX: crate::UnityEngine::UIElements::Repeat,
        repeatY: crate::UnityEngine::UIElements::Repeat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (repeatX, repeatY),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+BackgroundRepeat")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::BackgroundRepeat>>
for crate::UnityEngine::UIElements::BackgroundRepeat {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::BackgroundRepeat> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+BackgroundRepeat")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::BackgroundRepeat>>
for crate::UnityEngine::UIElements::BackgroundRepeat {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::BackgroundRepeat,
    > {
        todo!()
    }
}
