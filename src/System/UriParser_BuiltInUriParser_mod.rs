#[cfg(feature = "cordl_class_System+UriParser+BuiltInUriParser")]
#[repr(C)]
#[derive(Debug)]
pub struct UriParser_BuiltInUriParser {
    __cordl_parent: crate::System::UriParser,
}
#[cfg(feature = "cordl_class_System+UriParser+BuiltInUriParser")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::UriParser_BuiltInUriParser {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "UriParser/BuiltInUriParser";
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
#[cfg(feature = "System+UriParser+BuiltInUriParser")]
impl std::ops::Deref for crate::GlobalNamespace::UriParser_BuiltInUriParser {
    type Target = crate::System::UriParser;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+UriParser+BuiltInUriParser")]
impl std::ops::DerefMut for crate::GlobalNamespace::UriParser_BuiltInUriParser {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+UriParser+BuiltInUriParser")]
impl crate::GlobalNamespace::UriParser_BuiltInUriParser {
    pub fn New(
        lwrCaseScheme: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultPort: i32,
        syntaxFlags: crate::System::UriSyntaxFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lwrCaseScheme, defaultPort, syntaxFlags))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        lwrCaseScheme: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultPort: i32,
        syntaxFlags: crate::System::UriSyntaxFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        crate::System::UriSyntaxFlags,
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
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (lwrCaseScheme, defaultPort, syntaxFlags))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+UriParser+BuiltInUriParser")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::UriParser_BuiltInUriParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
