#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser")]
#[repr(C)]
#[derive(Debug)]
pub struct RichTextTagParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::TextCore::RichTextTagParser {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore";
    const CLASS_NAME: &'static str = "RichTextTagParser";
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
#[cfg(feature = "UnityEngine+TextCore+RichTextTagParser")]
impl std::ops::Deref for crate::UnityEngine::TextCore::RichTextTagParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+RichTextTagParser")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::RichTextTagParser {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+RichTextTagParser")]
impl crate::UnityEngine::TextCore::RichTextTagParser {
    #[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+ParseError")]
    pub type ParseError = crate::UnityEngine::TextCore::RichTextTagParser_ParseError;
    #[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+Segment")]
    pub type Segment = crate::UnityEngine::TextCore::RichTextTagParser_Segment;
    #[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+Tag")]
    pub type Tag = crate::UnityEngine::TextCore::RichTextTagParser_Tag;
    #[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+TagType")]
    pub type TagType = crate::UnityEngine::TextCore::RichTextTagParser_TagType;
    #[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+TagTypeInfo")]
    pub type TagTypeInfo = crate::UnityEngine::TextCore::RichTextTagParser_TagTypeInfo;
    #[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+TagUnitType")]
    pub type TagUnitType = crate::UnityEngine::TextCore::RichTextTagParser_TagUnitType;
    #[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+TagValue")]
    pub type TagValue = crate::UnityEngine::TextCore::RichTextTagParser_TagValue;
    #[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+TagValueType")]
    pub type TagValueType = crate::UnityEngine::TextCore::RichTextTagParser_TagValueType;
    pub fn AddLink(
        _cordl_type: crate::UnityEngine::TextCore::RichTextTagParser_TagType,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        links: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::System::ValueTuple_3<
                    i32,
                    crate::UnityEngine::TextCore::RichTextTagParser_TagType,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::TextCore::RichTextTagParser_TagType,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::System::ValueTuple_3<
                                    i32,
                                    crate::UnityEngine::TextCore::RichTextTagParser_TagType,
                                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                                >,
                            >,
                        >,
                    ), i32, 3usize>("AddLink")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddLink",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type, value, links))? };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStateToSegment(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tags: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::TextCore::RichTextTagParser_Tag,
            >,
        >,
        segments: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::RichTextTagParser_Segment,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::TextCore::RichTextTagParser_Tag,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::TextCore::RichTextTagParser_Segment,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "ApplyStateToSegment"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ApplyStateToSegment",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (input, tags, segments))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateTextGenerationSettingsArray(
        tgs: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::NativeTextGenerationSettings,
        >,
        links: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::System::ValueTuple_3<
                    i32,
                    crate::UnityEngine::TextCore::RichTextTagParser_TagType,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
        >,
        hyperlinkColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::TextCore::NativeTextGenerationSettings,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::System::ValueTuple_3<
                                    i32,
                                    crate::UnityEngine::TextCore::RichTextTagParser_TagType,
                                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                                >,
                            >,
                        >,
                        crate::UnityEngine::Color,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "CreateTextGenerationSettingsArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateTextGenerationSettingsArray",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (tgs, links, hyperlinkColor))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateTextSpan(
        segment: crate::UnityEngine::TextCore::RichTextTagParser_Segment,
        tgs: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::NativeTextGenerationSettings,
        >,
        links: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::System::ValueTuple_3<
                    i32,
                    crate::UnityEngine::TextCore::RichTextTagParser_TagType,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
        >,
        hyperlinkColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextCore::TextSpan> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::TextCore::RichTextTagParser_Segment,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::TextCore::NativeTextGenerationSettings,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::System::ValueTuple_3<
                                    i32,
                                    crate::UnityEngine::TextCore::RichTextTagParser_TagType,
                                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                                >,
                            >,
                        >,
                        crate::UnityEngine::Color,
                    ), crate::UnityEngine::TextCore::TextSpan, 4usize>(
                        "CreateTextSpan"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateTextSpan",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::TextSpan = unsafe {
            cordl_method_info.invoke_unchecked((), (segment, tgs, links, hyperlinkColor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindTags(
        inputStr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        errors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::RichTextTagParser_ParseError,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::TextCore::RichTextTagParser_Tag,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::TextCore::RichTextTagParser_ParseError,
                                >,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::UnityEngine::TextCore::RichTextTagParser_Tag,
                        >,
                    >, 2usize>("FindTags")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FindTags",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::TextCore::RichTextTagParser_Tag,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (inputStr, errors))? };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateSegments(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tags: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::TextCore::RichTextTagParser_Tag,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::RichTextTagParser_Segment,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::TextCore::RichTextTagParser_Tag,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::TextCore::RichTextTagParser_Segment,
                        >,
                    >, 2usize>("GenerateSegments")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GenerateSegments",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::RichTextTagParser_Segment,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (input, tags))? };
        Ok(__cordl_ret.into())
    }
    pub fn PickResultingTags(
        allTags: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::TextCore::RichTextTagParser_Tag,
            >,
        >,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        atPosition: i32,
        applicableTags: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::TextCore::RichTextTagParser_Tag,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::TextCore::RichTextTagParser_Tag,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::TextCore::RichTextTagParser_Tag,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::TextCore::RichTextTagParser_Tag,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::UnityEngine::TextCore::RichTextTagParser_Tag,
                        >,
                    >, 4usize>("PickResultingTags")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PickResultingTags",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::TextCore::RichTextTagParser_Tag,
            >,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (allTags, input, atPosition, applicableTags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SpanToEnum(
        tagCandidate: crate::System::ReadOnlySpan_1<char>,
        tagType: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::RichTextTagParser_TagType,
        >,
        error: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        attribute: quest_hook::libil2cpp::ByRefMut<crate::System::ReadOnlySpan_1<char>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<char>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::TextCore::RichTextTagParser_TagType,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<crate::System::ReadOnlySpan_1<char>>,
                    ), bool, 4usize>("SpanToEnum")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SpanToEnum",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (tagCandidate, tagType, error, attribute))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn tagMatch(
        tagCandidate: crate::System::ReadOnlySpan_1<char>,
        tagName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<char>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), bool, 2usize>("tagMatch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "tagMatch",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (tagCandidate, tagName))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::TextCore::RichTextTagParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+ParseError")]
#[repr(C)]
#[derive(Debug)]
pub struct RichTextTagParser_ParseError {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub position: i32,
    pub message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+ParseError")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::TextCore::RichTextTagParser_ParseError
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore";
    const CLASS_NAME: &'static str = "RichTextTagParser/ParseError";
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
#[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+ParseError")]
impl std::ops::Deref for crate::UnityEngine::TextCore::RichTextTagParser_ParseError {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+ParseError")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::RichTextTagParser_ParseError {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+ParseError")]
impl crate::UnityEngine::TextCore::RichTextTagParser_ParseError {
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_RichTextTagParser_ParseError1(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::RichTextTagParser_ParseError,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::RichTextTagParser_ParseError,
                    >), bool, 1usize>("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetHashCode",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        position: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, position))?;
        Ok(__cordl_object.into())
    }
    pub fn PrintMembers(
        &mut self,
        builder: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>),
                        bool,
                        1usize,
                    >("PrintMembers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrintMembers", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (builder))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        position: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
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
            unsafe { cordl_method_info.invoke_unchecked(self, (message, position))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_EqualityContract(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<crate::System::Type>, 0usize>(
                        "get_EqualityContract",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_EqualityContract",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+ParseError")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::TextCore::RichTextTagParser_ParseError
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+ParseError")]
impl
    AsRef<
        crate::System::IEquatable_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::RichTextTagParser_ParseError>,
        >,
    > for crate::UnityEngine::TextCore::RichTextTagParser_ParseError
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::RichTextTagParser_ParseError>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+ParseError")]
impl
    AsMut<
        crate::System::IEquatable_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::RichTextTagParser_ParseError>,
        >,
    > for crate::UnityEngine::TextCore::RichTextTagParser_ParseError
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::RichTextTagParser_ParseError>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+Segment")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct RichTextTagParser_Segment {
    pub tags: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::TextCore::RichTextTagParser_Tag,
        >,
    >,
    pub start: i32,
    pub end: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+Segment")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::TextCore::RichTextTagParser_Segment
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore";
    const CLASS_NAME: &'static str = "RichTextTagParser/Segment";
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+Segment")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::TextCore::RichTextTagParser_Segment
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+Segment")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::TextCore::RichTextTagParser_Segment
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+Segment")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::TextCore::RichTextTagParser_Segment
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+Segment")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::TextCore::RichTextTagParser_Segment
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+Segment")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::TextCore::RichTextTagParser_Segment
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+Segment")]
impl crate::UnityEngine::TextCore::RichTextTagParser_Segment {}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+Tag")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct RichTextTagParser_Tag {
    pub tagType: crate::UnityEngine::TextCore::RichTextTagParser_TagType,
    pub isClosing: bool,
    pub start: i32,
    pub end: i32,
    pub value: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::RichTextTagParser_TagValue>,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+Tag")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::TextCore::RichTextTagParser_Tag {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore";
    const CLASS_NAME: &'static str = "RichTextTagParser/Tag";
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+Tag")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::TextCore::RichTextTagParser_Tag
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+Tag")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::TextCore::RichTextTagParser_Tag
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+Tag")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::TextCore::RichTextTagParser_Tag
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+Tag")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::TextCore::RichTextTagParser_Tag {
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+Tag")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::TextCore::RichTextTagParser_Tag
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+Tag")]
impl crate::UnityEngine::TextCore::RichTextTagParser_Tag {}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum RichTextTagParser_TagType {
    #[default]
    Align = 1i32,
    AllCaps = 2i32,
    Alpha = 3i32,
    Bold = 4i32,
    Br = 5i32,
    CSpace = 7i32,
    Color = 6i32,
    Font = 8i32,
    FontWeight = 9i32,
    Hyperlink = 0i32,
    Indent = 11i32,
    Italic = 10i32,
    LineHeight = 12i32,
    LineIndent = 13i32,
    Link = 14i32,
    Lowercase = 15i32,
    Mark = 16i32,
    Mspace = 17i32,
    NoBr = 18i32,
    NoParse = 19i32,
    Size = 21i32,
    SmallCaps = 22i32,
    Space = 23i32,
    Sprite = 24i32,
    Strikethrough = 20i32,
    Style = 25i32,
    Subscript = 26i32,
    Superscript = 27i32,
    Underline = 28i32,
    Unknown = 30i32,
    Uppercase = 29i32,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagType")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::TextCore::RichTextTagParser_TagType
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore";
    const CLASS_NAME: &'static str = "RichTextTagParser/TagType";
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagType")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::TextCore::RichTextTagParser_TagType
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagType")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::TextCore::RichTextTagParser_TagType
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagType")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::TextCore::RichTextTagParser_TagType
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagType")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::TextCore::RichTextTagParser_TagType
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagTypeInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct RichTextTagParser_TagTypeInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub TagType: crate::UnityEngine::TextCore::RichTextTagParser_TagType,
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub valueType: crate::UnityEngine::TextCore::RichTextTagParser_TagValueType,
    pub unitType: crate::UnityEngine::TextCore::RichTextTagParser_TagUnitType,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagTypeInfo")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::TextCore::RichTextTagParser_TagTypeInfo
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore";
    const CLASS_NAME: &'static str = "RichTextTagParser/TagTypeInfo";
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
#[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+TagTypeInfo")]
impl std::ops::Deref for crate::UnityEngine::TextCore::RichTextTagParser_TagTypeInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+TagTypeInfo")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::RichTextTagParser_TagTypeInfo {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+TagTypeInfo")]
impl crate::UnityEngine::TextCore::RichTextTagParser_TagTypeInfo {
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_RichTextTagParser_TagTypeInfo1(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::RichTextTagParser_TagTypeInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::RichTextTagParser_TagTypeInfo,
                    >), bool, 1usize>("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetHashCode",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        tagType: crate::UnityEngine::TextCore::RichTextTagParser_TagType,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        valueType: crate::UnityEngine::TextCore::RichTextTagParser_TagValueType,
        unitType: crate::UnityEngine::TextCore::RichTextTagParser_TagUnitType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tagType, name, valueType, unitType))?;
        Ok(__cordl_object.into())
    }
    pub fn PrintMembers(
        &mut self,
        builder: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>),
                        bool,
                        1usize,
                    >("PrintMembers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrintMembers", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (builder))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        tagType: crate::UnityEngine::TextCore::RichTextTagParser_TagType,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        valueType: crate::UnityEngine::TextCore::RichTextTagParser_TagValueType,
        unitType: crate::UnityEngine::TextCore::RichTextTagParser_TagUnitType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::TextCore::RichTextTagParser_TagType,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::TextCore::RichTextTagParser_TagValueType,
                        crate::UnityEngine::TextCore::RichTextTagParser_TagUnitType,
                    ), quest_hook::libil2cpp::Void, 4usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (tagType, name, valueType, unitType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_EqualityContract(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<crate::System::Type>, 0usize>(
                        "get_EqualityContract",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_EqualityContract",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagTypeInfo")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::TextCore::RichTextTagParser_TagTypeInfo
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+TagTypeInfo")]
impl
    AsRef<
        crate::System::IEquatable_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::RichTextTagParser_TagTypeInfo>,
        >,
    > for crate::UnityEngine::TextCore::RichTextTagParser_TagTypeInfo
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::RichTextTagParser_TagTypeInfo>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+TagTypeInfo")]
impl
    AsMut<
        crate::System::IEquatable_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::RichTextTagParser_TagTypeInfo>,
        >,
    > for crate::UnityEngine::TextCore::RichTextTagParser_TagTypeInfo
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::RichTextTagParser_TagTypeInfo>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagUnitType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum RichTextTagParser_TagUnitType {
    #[default]
    FontUnits = 1i32,
    Percentage = 2i32,
    Pixels = 0i32,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagUnitType")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::TextCore::RichTextTagParser_TagUnitType
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore";
    const CLASS_NAME: &'static str = "RichTextTagParser/TagUnitType";
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagUnitType")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::TextCore::RichTextTagParser_TagUnitType
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagUnitType")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::TextCore::RichTextTagParser_TagUnitType
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagUnitType")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::TextCore::RichTextTagParser_TagUnitType
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagUnitType")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::TextCore::RichTextTagParser_TagUnitType
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagValue")]
#[repr(C)]
#[derive(Debug)]
pub struct RichTextTagParser_TagValue {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cordl_type: crate::UnityEngine::TextCore::RichTextTagParser_TagValueType,
    pub m_stringValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_numericalValue: f32,
    pub m_colorValue: crate::UnityEngine::Color,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagValue")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::TextCore::RichTextTagParser_TagValue
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore";
    const CLASS_NAME: &'static str = "RichTextTagParser/TagValue";
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
#[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+TagValue")]
impl std::ops::Deref for crate::UnityEngine::TextCore::RichTextTagParser_TagValue {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+TagValue")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::RichTextTagParser_TagValue {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+TagValue")]
impl crate::UnityEngine::TextCore::RichTextTagParser_TagValue {
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_RichTextTagParser_TagValue1(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::RichTextTagParser_TagValue>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::RichTextTagParser_TagValue,
                    >), bool, 1usize>("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetHashCode",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New_Color0(
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (value))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (value))?;
        Ok(__cordl_object.into())
    }
    pub fn PrintMembers(
        &mut self,
        builder: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>),
                        bool,
                        1usize,
                    >("PrintMembers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrintMembers", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (builder))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Color0(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Color),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ColorValue(&mut self) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::Color, 0usize>("get_ColorValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_ColorValue",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_EqualityContract(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<crate::System::Type>, 0usize>(
                        "get_EqualityContract",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_EqualityContract",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_NumericalValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_NumericalValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_NumericalValue",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_StringValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_StringValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_StringValue", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagValue")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::TextCore::RichTextTagParser_TagValue
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+TagValue")]
impl
    AsRef<
        crate::System::IEquatable_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::RichTextTagParser_TagValue>,
        >,
    > for crate::UnityEngine::TextCore::RichTextTagParser_TagValue
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::RichTextTagParser_TagValue>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+TextCore+RichTextTagParser+TagValue")]
impl
    AsMut<
        crate::System::IEquatable_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::RichTextTagParser_TagValue>,
        >,
    > for crate::UnityEngine::TextCore::RichTextTagParser_TagValue
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::RichTextTagParser_TagValue>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagValueType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum RichTextTagParser_TagValueType {
    #[default]
    ColorValue = 4i32,
    None = 0i32,
    NumericalValue = 1i32,
    StringValue = 2i32,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagValueType")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::TextCore::RichTextTagParser_TagValueType
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore";
    const CLASS_NAME: &'static str = "RichTextTagParser/TagValueType";
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagValueType")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::TextCore::RichTextTagParser_TagValueType
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagValueType")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::TextCore::RichTextTagParser_TagValueType
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagValueType")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::TextCore::RichTextTagParser_TagValueType
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+RichTextTagParser+TagValueType")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::TextCore::RichTextTagParser_TagValueType
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
