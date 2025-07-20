#[cfg(feature = "TMPro+TMP_GlyphPairAdjustmentRecord")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_GlyphPairAdjustmentRecord {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_FirstAdjustmentRecord: crate::TMPro::TMP_GlyphAdjustmentRecord,
    pub m_SecondAdjustmentRecord: crate::TMPro::TMP_GlyphAdjustmentRecord,
    pub m_FeatureLookupFlags: crate::TMPro::FontFeatureLookupFlags,
}
#[cfg(feature = "TMPro+TMP_GlyphPairAdjustmentRecord")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_GlyphPairAdjustmentRecord {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_GlyphPairAdjustmentRecord";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "TMPro+TMP_GlyphPairAdjustmentRecord")]
impl std::ops::Deref for crate::TMPro::TMP_GlyphPairAdjustmentRecord {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_GlyphPairAdjustmentRecord")]
impl std::ops::DerefMut for crate::TMPro::TMP_GlyphPairAdjustmentRecord {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_GlyphPairAdjustmentRecord")]
impl crate::TMPro::TMP_GlyphPairAdjustmentRecord {
    pub fn New_GlyphPairAdjustmentRecord1(
        glyphPairAdjustmentRecord: crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (glyphPairAdjustmentRecord))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TMP_GlyphAdjustmentRecord_TMP_GlyphAdjustmentRecord0(
        firstAdjustmentRecord: crate::TMPro::TMP_GlyphAdjustmentRecord,
        secondAdjustmentRecord: crate::TMPro::TMP_GlyphAdjustmentRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (firstAdjustmentRecord, secondAdjustmentRecord))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_GlyphPairAdjustmentRecord1(
        &mut self,
        glyphPairAdjustmentRecord: crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (glyphPairAdjustmentRecord))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TMP_GlyphAdjustmentRecord_TMP_GlyphAdjustmentRecord0(
        &mut self,
        firstAdjustmentRecord: crate::TMPro::TMP_GlyphAdjustmentRecord,
        secondAdjustmentRecord: crate::TMPro::TMP_GlyphAdjustmentRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::TMPro::TMP_GlyphAdjustmentRecord,
                            crate::TMPro::TMP_GlyphAdjustmentRecord,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(self, (firstAdjustmentRecord, secondAdjustmentRecord))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_featureLookupFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::FontFeatureLookupFlags> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::TMPro::FontFeatureLookupFlags,
                        0usize,
                    >("get_featureLookupFlags")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_featureLookupFlags", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::TMPro::FontFeatureLookupFlags = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_firstAdjustmentRecord(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::TMP_GlyphAdjustmentRecord> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::TMPro::TMP_GlyphAdjustmentRecord,
                        0usize,
                    >("get_firstAdjustmentRecord")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_firstAdjustmentRecord", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::TMPro::TMP_GlyphAdjustmentRecord = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_secondAdjustmentRecord(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::TMP_GlyphAdjustmentRecord> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::TMPro::TMP_GlyphAdjustmentRecord,
                        0usize,
                    >("get_secondAdjustmentRecord")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_secondAdjustmentRecord", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::TMPro::TMP_GlyphAdjustmentRecord = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_featureLookupFlags(
        &mut self,
        value: crate::TMPro::FontFeatureLookupFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::TMPro::FontFeatureLookupFlags),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_featureLookupFlags")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "set_featureLookupFlags", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_firstAdjustmentRecord(
        &mut self,
        value: crate::TMPro::TMP_GlyphAdjustmentRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::TMPro::TMP_GlyphAdjustmentRecord),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_firstAdjustmentRecord")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "set_firstAdjustmentRecord", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_secondAdjustmentRecord(
        &mut self,
        value: crate::TMPro::TMP_GlyphAdjustmentRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::TMPro::TMP_GlyphAdjustmentRecord),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_secondAdjustmentRecord")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "set_secondAdjustmentRecord", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMP_GlyphPairAdjustmentRecord")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_GlyphPairAdjustmentRecord {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
