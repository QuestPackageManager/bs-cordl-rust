#[cfg(feature = "cordl_class_System+Text+RegularExpressions+RegexReplacement")]
#[repr(C)]
#[derive(Debug)]
pub struct RegexReplacement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _strings: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _rules: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
    pub _Pattern_k__BackingField: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "cordl_class_System+Text+RegularExpressions+RegexReplacement")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::System::Text::RegularExpressions::RegexReplacement
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Text.RegularExpressions";
    const CLASS_NAME: &'static str = "RegexReplacement";
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
#[cfg(feature = "System+Text+RegularExpressions+RegexReplacement")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::RegexReplacement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexReplacement")]
impl std::ops::DerefMut for crate::System::Text::RegularExpressions::RegexReplacement {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexReplacement")]
impl crate::System::Text::RegularExpressions::RegexReplacement {
    pub const LastGroup: i32 = -3i32;
    pub const LeftPortion: i32 = -1i32;
    pub const RightPortion: i32 = -2i32;
    pub const Specials: i32 = 4i32;
    pub const WholeString: i32 = -4i32;
    pub fn GetOrCreate(
        replRef: quest_hook::libil2cpp::Gc<
            crate::System::WeakReference_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Text::RegularExpressions::RegexReplacement,
                >,
            >,
        >,
        replacement: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        caps: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        capsize: i32,
        capnames: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        roptions: crate::System::Text::RegularExpressions::RegexOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::RegexReplacement>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::WeakReference_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Text::RegularExpressions::RegexReplacement,
                                >,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
                        crate::System::Text::RegularExpressions::RegexOptions,
                    ), quest_hook::libil2cpp::Gc<
                        crate::System::Text::RegularExpressions::RegexReplacement,
                    >, 6usize>("GetOrCreate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetOrCreate",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexReplacement,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (replRef, replacement, caps, capsize, capnames, roptions),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        rep: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        concat: quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::RegexNode>,
        _caps: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rep, concat, _caps))?;
        Ok(__cordl_object.into())
    }
    pub fn Replace(
        &mut self,
        regex: quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Regex>,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        count: i32,
        startat: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Regex>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, 4usize>(
                        "Replace",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Replace",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, (regex, input, count, startat))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReplacementImpl(
        &mut self,
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        _cordl_match: quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Match>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                        quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Match>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ReplacementImpl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReplacementImpl",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (sb, _cordl_match))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReplacementImplRTL(
        &mut self,
        al: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        _cordl_match: quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Match>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Match>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ReplacementImplRTL")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReplacementImplRTL",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (al, _cordl_match))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        rep: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        concat: quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::RegexNode>,
        _caps: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Text::RegularExpressions::RegexNode,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rep, concat, _caps))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Pattern(
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
                    >("get_Pattern")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Pattern", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Text+RegularExpressions+RegexReplacement")]
impl quest_hook::libil2cpp::ObjectType
    for crate::System::Text::RegularExpressions::RegexReplacement
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
