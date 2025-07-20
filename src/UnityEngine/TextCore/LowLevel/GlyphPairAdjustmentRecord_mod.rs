#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphPairAdjustmentRecord")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GlyphPairAdjustmentRecord {
    pub m_FirstAdjustmentRecord: crate::UnityEngine::TextCore::LowLevel::GlyphAdjustmentRecord,
    pub m_SecondAdjustmentRecord: crate::UnityEngine::TextCore::LowLevel::GlyphAdjustmentRecord,
    pub m_FeatureLookupFlags: crate::UnityEngine::TextCore::LowLevel::FontFeatureLookupFlags,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphPairAdjustmentRecord")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.LowLevel";
    const CLASS_NAME: &'static str = "GlyphPairAdjustmentRecord";
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
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphPairAdjustmentRecord")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphPairAdjustmentRecord")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord {
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
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphPairAdjustmentRecord")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord {
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
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphPairAdjustmentRecord")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord {
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
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphPairAdjustmentRecord")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphPairAdjustmentRecord")]
impl crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord {
    pub fn Equals_GlyphPairAdjustmentRecord1(
        &mut self,
        other: crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Equals", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Equals", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetHashCode", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_featureLookupFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::FontFeatureLookupFlags,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::TextCore::LowLevel::FontFeatureLookupFlags,
                        0usize,
                    >("get_featureLookupFlags")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_featureLookupFlags", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::FontFeatureLookupFlags = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_firstAdjustmentRecord(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::GlyphAdjustmentRecord,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::TextCore::LowLevel::GlyphAdjustmentRecord,
                        0usize,
                    >("get_firstAdjustmentRecord")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_firstAdjustmentRecord", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::GlyphAdjustmentRecord = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_secondAdjustmentRecord(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::GlyphAdjustmentRecord,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::TextCore::LowLevel::GlyphAdjustmentRecord,
                        0usize,
                    >("get_secondAdjustmentRecord")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_secondAdjustmentRecord", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::GlyphAdjustmentRecord = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphPairAdjustmentRecord")]
impl AsRef<
    crate::System::IEquatable_1<
        crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
    >,
> for crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphPairAdjustmentRecord")]
impl AsMut<
    crate::System::IEquatable_1<
        crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
    >,
> for crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
    > {
        todo!()
    }
}
