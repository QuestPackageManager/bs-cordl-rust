#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+UnicodeLineBreakingRules")]
#[repr(C)]
#[derive(Debug)]
pub struct UnicodeLineBreakingRules {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_UnicodeLineBreakingRules: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextAsset,
    >,
    pub m_LeadingCharacters: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    pub m_FollowingCharacters: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    pub m_UseModernHangulLineBreakingRules: bool,
    pub m_LeadingCharactersLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<u32>,
    >,
    pub m_FollowingCharactersLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<u32>,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+UnicodeLineBreakingRules")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::Text::UnicodeLineBreakingRules {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "UnicodeLineBreakingRules";
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
#[cfg(feature = "UnityEngine+TextCore+Text+UnicodeLineBreakingRules")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::UnicodeLineBreakingRules {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+UnicodeLineBreakingRules")]
impl std::ops::DerefMut
for crate::UnityEngine::TextCore::Text::UnicodeLineBreakingRules {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+UnicodeLineBreakingRules")]
impl crate::UnityEngine::TextCore::Text::UnicodeLineBreakingRules {
    pub fn GetCharacters(
        file: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::HashSet_1<u32>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::HashSet_1<u32>,
                        >,
                        1usize,
                    >("GetCharacters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCharacters", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<u32>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (file))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadLineBreakingRules_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("LoadLineBreakingRules")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadLineBreakingRules", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadLineBreakingRules_TextAsset_TextAsset1(
        &mut self,
        leadingRules: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
        followingRules: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("LoadLineBreakingRules")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadLineBreakingRules", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (leadingRules, followingRules))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_followingCharacters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
                        0usize,
                    >("get_followingCharacters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_followingCharacters", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_followingCharactersLookup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::HashSet_1<u32>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::HashSet_1<u32>,
                        >,
                        0usize,
                    >("get_followingCharactersLookup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_followingCharactersLookup", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<u32>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_leadingCharacters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
                        0usize,
                    >("get_leadingCharacters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_leadingCharacters", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_leadingCharactersLookup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::HashSet_1<u32>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::HashSet_1<u32>,
                        >,
                        0usize,
                    >("get_leadingCharactersLookup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_leadingCharactersLookup", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<u32>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_useModernHangulLineBreakingRules(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        bool,
                        0usize,
                    >("get_useModernHangulLineBreakingRules")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_useModernHangulLineBreakingRules", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+UnicodeLineBreakingRules")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::UnicodeLineBreakingRules {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
