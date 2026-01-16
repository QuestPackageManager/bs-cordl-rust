#[cfg(feature = "cordl_class_System+Text+RegularExpressions+MatchSparse")]
#[repr(C)]
#[derive(Debug)]
pub struct MatchSparse {
    __cordl_parent: crate::System::Text::RegularExpressions::Match,
    pub _caps: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
}
#[cfg(feature = "cordl_class_System+Text+RegularExpressions+MatchSparse")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Text::RegularExpressions::MatchSparse {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Text.RegularExpressions";
    const CLASS_NAME: &'static str = "MatchSparse";
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
#[cfg(feature = "System+Text+RegularExpressions+MatchSparse")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::MatchSparse {
    type Target = crate::System::Text::RegularExpressions::Match;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+MatchSparse")]
impl std::ops::DerefMut for crate::System::Text::RegularExpressions::MatchSparse {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+MatchSparse")]
impl crate::System::Text::RegularExpressions::MatchSparse {
    pub fn New(
        regex: quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Regex>,
        caps: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        capcount: i32,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        begpos: i32,
        len: i32,
        startpos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (regex, caps, capcount, text, begpos, len, startpos),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        regex: quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Regex>,
        caps: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        capcount: i32,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        begpos: i32,
        len: i32,
        startpos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Regex>,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 7usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (regex, caps, capcount, text, begpos, len, startpos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Groups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::GroupCollection>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Text::RegularExpressions::GroupCollection,
                    >, 0usize>("get_Groups")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_Groups",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::GroupCollection,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Text+RegularExpressions+MatchSparse")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Text::RegularExpressions::MatchSparse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
