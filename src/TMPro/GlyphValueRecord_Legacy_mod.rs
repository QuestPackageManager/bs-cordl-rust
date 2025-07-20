#[cfg(feature = "TMPro+GlyphValueRecord_Legacy")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GlyphValueRecord_Legacy {
    pub xPlacement: f32,
    pub yPlacement: f32,
    pub xAdvance: f32,
    pub yAdvance: f32,
}
#[cfg(feature = "TMPro+GlyphValueRecord_Legacy")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::GlyphValueRecord_Legacy {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "GlyphValueRecord_Legacy";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "TMPro+GlyphValueRecord_Legacy")]
unsafe impl quest_hook::libil2cpp::Argument for crate::TMPro::GlyphValueRecord_Legacy {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "TMPro+GlyphValueRecord_Legacy")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::TMPro::GlyphValueRecord_Legacy {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "TMPro+GlyphValueRecord_Legacy")]
unsafe impl quest_hook::libil2cpp::Returned for crate::TMPro::GlyphValueRecord_Legacy {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "TMPro+GlyphValueRecord_Legacy")]
unsafe impl quest_hook::libil2cpp::Return for crate::TMPro::GlyphValueRecord_Legacy {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "TMPro+GlyphValueRecord_Legacy")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::TMPro::GlyphValueRecord_Legacy {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+GlyphValueRecord_Legacy")]
impl crate::TMPro::GlyphValueRecord_Legacy {
    pub fn _ctor(
        &mut self,
        valueRecord: crate::UnityEngine::TextCore::LowLevel::GlyphValueRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::GlyphValueRecord_Legacy as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::TextCore::LowLevel::GlyphValueRecord),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::GlyphValueRecord_Legacy as quest_hook::libil2cpp::Type
                    > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (valueRecord))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        a: crate::TMPro::GlyphValueRecord_Legacy,
        b: crate::TMPro::GlyphValueRecord_Legacy,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::GlyphValueRecord_Legacy> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::GlyphValueRecord_Legacy as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::TMPro::GlyphValueRecord_Legacy,
                    crate::TMPro::GlyphValueRecord_Legacy,
                ),
                crate::TMPro::GlyphValueRecord_Legacy,
                2usize,
            >("op_Addition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::GlyphValueRecord_Legacy as quest_hook::libil2cpp::Type
                    > ::class(), "op_Addition", 2usize
                )
            });
        let __cordl_ret: crate::TMPro::GlyphValueRecord_Legacy = unsafe {
            method.invoke_unchecked((), (a, b))?
        };
        Ok(__cordl_ret.into())
    }
}
