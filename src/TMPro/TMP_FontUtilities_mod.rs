#[cfg(feature = "TMPro+TMP_FontUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_FontUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TMPro+TMP_FontUtilities")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_FontUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_FontUtilities";
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
#[cfg(feature = "TMPro+TMP_FontUtilities")]
impl std::ops::Deref for crate::TMPro::TMP_FontUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_FontUtilities")]
impl std::ops::DerefMut for crate::TMPro::TMP_FontUtilities {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_FontUtilities")]
impl crate::TMPro::TMP_FontUtilities {
    pub fn SearchForCharacterInternal_List_1_1(
        fonts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
            >,
        >,
        unicode: u32,
        character: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
                                >,
                            >,
                            u32,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
                        3usize,
                    >("SearchForCharacterInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SearchForCharacterInternal", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset> = unsafe {
            cordl_method_info.invoke_unchecked((), (fonts, unicode, character))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SearchForCharacterInternal_TMP_FontAsset0(
        font: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
        unicode: u32,
        character: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
                            u32,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
                        3usize,
                    >("SearchForCharacterInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SearchForCharacterInternal", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset> = unsafe {
            cordl_method_info.invoke_unchecked((), (font, unicode, character))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SearchForCharacter_List_1_1(
        fonts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
            >,
        >,
        unicode: u32,
        character: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
                                >,
                            >,
                            u32,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
                        3usize,
                    >("SearchForCharacter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SearchForCharacter", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset> = unsafe {
            cordl_method_info.invoke_unchecked((), (fonts, unicode, character))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SearchForCharacter_TMP_FontAsset0(
        font: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
        unicode: u32,
        character: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
                            u32,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
                        3usize,
                    >("SearchForCharacter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SearchForCharacter", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset> = unsafe {
            cordl_method_info.invoke_unchecked((), (font, unicode, character))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMP_FontUtilities")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_FontUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
