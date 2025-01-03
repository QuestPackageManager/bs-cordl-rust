#[cfg(feature = "UnityEngine+UIElements+TransitionData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TransitionData {
    pub transitionDelay: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::TimeValue,
        >,
    >,
    pub transitionDuration: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::TimeValue,
        >,
    >,
    pub transitionProperty: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::StylePropertyName,
        >,
    >,
    pub transitionTimingFunction: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::EasingFunction,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+TransitionData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TransitionData =>
    "UnityEngine.UIElements"."TransitionData"
);
#[cfg(feature = "UnityEngine+UIElements+TransitionData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::TransitionData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TransitionData")]
impl crate::UnityEngine::UIElements::TransitionData {
    pub fn Copy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TransitionData> {
        let __cordl_ret: crate::UnityEngine::UIElements::TransitionData = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Copy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyFrom(
        &mut self,
        other: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::TransitionData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyFrom",
            (other),
        )?;
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
    pub fn Equals_TransitionData0(
        &mut self,
        other: crate::UnityEngine::UIElements::TransitionData,
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
    pub fn op_Equality(
        lhs: crate::UnityEngine::UIElements::TransitionData,
        rhs: crate::UnityEngine::UIElements::TransitionData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+TransitionData")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::TransitionData>>
for crate::UnityEngine::UIElements::TransitionData {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::TransitionData> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+TransitionData")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::TransitionData>>
for crate::UnityEngine::UIElements::TransitionData {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::TransitionData,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+TransitionData")]
impl AsRef<
    crate::UnityEngine::UIElements::IStyleDataGroup_1<
        crate::UnityEngine::UIElements::TransitionData,
    >,
> for crate::UnityEngine::UIElements::TransitionData {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::UIElements::IStyleDataGroup_1<
        crate::UnityEngine::UIElements::TransitionData,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+TransitionData")]
impl AsMut<
    crate::UnityEngine::UIElements::IStyleDataGroup_1<
        crate::UnityEngine::UIElements::TransitionData,
    >,
> for crate::UnityEngine::UIElements::TransitionData {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::IStyleDataGroup_1<
        crate::UnityEngine::UIElements::TransitionData,
    > {
        todo!()
    }
}
