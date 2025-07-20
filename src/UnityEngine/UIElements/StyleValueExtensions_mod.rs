#[cfg(feature = "UnityEngine+UIElements+StyleValueExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleValueExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::StyleValueExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "StyleValueExtensions";
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
#[cfg(feature = "UnityEngine+UIElements+StyleValueExtensions")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleValueExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::StyleValueExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueExtensions")]
impl crate::UnityEngine::UIElements::StyleValueExtensions {
    pub fn CopyFrom<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        other: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::StyleValueExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<T>,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<T>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CopyFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::StyleValueExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "CopyFrom", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (list, other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DebugString<T>(
        styleValue: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IStyleValue_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::StyleValueExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::IStyleValue_1<T>,
                >),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("DebugString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::StyleValueExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "DebugString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (styleValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToLength_StyleKeyword0(
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::StyleValueExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::UIElements::StyleKeyword),
                crate::UnityEngine::UIElements::Length,
                1usize,
            >("ToLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::StyleValueExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "ToLength", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), (keyword))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToLength_StyleLength1(
        styleLength: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::StyleValueExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::UIElements::StyleLength),
                crate::UnityEngine::UIElements::Length,
                1usize,
            >("ToLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::StyleValueExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "ToLength", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Length = unsafe {
            method.invoke_unchecked((), (styleLength))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToRotate(
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Rotate> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::StyleValueExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::UIElements::StyleKeyword),
                crate::UnityEngine::UIElements::Rotate,
                1usize,
            >("ToRotate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::StyleValueExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "ToRotate", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Rotate = unsafe {
            method.invoke_unchecked((), (keyword))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToScale(
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Scale> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::StyleValueExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::UIElements::StyleKeyword),
                crate::UnityEngine::UIElements::Scale,
                1usize,
            >("ToScale")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::StyleValueExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "ToScale", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Scale = unsafe {
            method.invoke_unchecked((), (keyword))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToTranslate(
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Translate> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::StyleValueExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::UIElements::StyleKeyword),
                crate::UnityEngine::UIElements::Translate,
                1usize,
            >("ToTranslate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::StyleValueExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "ToTranslate", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Translate = unsafe {
            method.invoke_unchecked((), (keyword))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToYogaValue(
        length: crate::UnityEngine::UIElements::Length,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Yoga::YogaValue> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::StyleValueExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::UIElements::Length),
                crate::UnityEngine::Yoga::YogaValue,
                1usize,
            >("ToYogaValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::StyleValueExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "ToYogaValue", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Yoga::YogaValue = unsafe {
            method.invoke_unchecked((), (length))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleValueExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
