#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StyleSelectorHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleSelectorHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StyleSelectorHelper")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.StyleSheets";
    const CLASS_NAME: &'static str = "StyleSelectorHelper";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSelectorHelper")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSelectorHelper")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSelectorHelper")]
impl crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper {
    #[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSelectorHelper+SelectorWorkItem")]
    pub type SelectorWorkItem =
        crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper_SelectorWorkItem;
    pub fn FastLookup(
        table: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IDictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleComplexSelector>,
            >,
        >,
        matchedSelectors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord,
            >,
        >,
        context: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleMatchingContext>,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        record: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IDictionary_2<
                                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::UIElements::StyleComplexSelector,
                                >,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::StyleMatchingContext,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord,
                        >,
                    ), quest_hook::libil2cpp::Void, 5usize>("FastLookup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FastLookup",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (table, matchedSelectors, context, input, record))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindMatches(
        context: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleMatchingContext>,
        matchedSelectors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord,
            >,
        >,
        parentSheetIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::StyleMatchingContext,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord,
                            >,
                        >,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("FindMatches")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FindMatches",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (context, matchedSelectors, parentSheetIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MatchRightToLeft(
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        complexSelector: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleComplexSelector,
        >,
        processResult: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                crate::UnityEngine::UIElements::StyleSheets::MatchResultInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::StyleComplexSelector,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_2<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::UIElements::VisualElement,
                                >,
                                crate::UnityEngine::UIElements::StyleSheets::MatchResultInfo,
                            >,
                        >,
                    ), bool, 3usize>("MatchRightToLeft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MatchRightToLeft",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (element, complexSelector, processResult))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MatchesSelector(
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        selector: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSelector>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleSheets::MatchResultInfo>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSelector>,
                    ), crate::UnityEngine::UIElements::StyleSheets::MatchResultInfo, 2usize>(
                        "MatchesSelector",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MatchesSelector",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleSheets::MatchResultInfo =
            unsafe { cordl_method_info.invoke_unchecked((), (element, selector))? };
        Ok(__cordl_ret.into())
    }
    pub fn TestSelectorLinkedList(
        currentComplexSelector: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleComplexSelector,
        >,
        matchedSelectors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord,
            >,
        >,
        context: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleMatchingContext>,
        record: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::StyleComplexSelector,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::StyleMatchingContext,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::StyleSheets::SelectorMatchRecord,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "TestSelectorLinkedList"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TestSelectorLinkedList",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (currentComplexSelector, matchedSelectors, context, record),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StyleSelectorHelper")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StyleSelectorHelper+SelectorWorkItem"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct StyleSelectorHelper_SelectorWorkItem {
    pub _cordl_type: crate::UnityEngine::UIElements::StyleSheet_OrderedSelectorType,
    pub input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StyleSelectorHelper+SelectorWorkItem"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper_SelectorWorkItem
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.StyleSheets";
    const CLASS_NAME: &'static str = "StyleSelectorHelper/SelectorWorkItem";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StyleSelectorHelper+SelectorWorkItem"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper_SelectorWorkItem
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StyleSelectorHelper+SelectorWorkItem"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper_SelectorWorkItem
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StyleSelectorHelper+SelectorWorkItem"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper_SelectorWorkItem
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StyleSelectorHelper+SelectorWorkItem"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper_SelectorWorkItem
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StyleSelectorHelper+SelectorWorkItem"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper_SelectorWorkItem
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSelectorHelper+SelectorWorkItem")]
impl crate::UnityEngine::UIElements::StyleSheets::StyleSelectorHelper_SelectorWorkItem {
    pub fn _ctor(
        &mut self,
        _cordl_type: crate::UnityEngine::UIElements::StyleSheet_OrderedSelectorType,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::StyleSheet_OrderedSelectorType,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (_cordl_type, input))? };
        Ok(__cordl_ret.into())
    }
}
