#[cfg(feature = "Mono+Xml+SmallXmlParserException")]
#[repr(C)]
#[derive(Debug)]
pub struct SmallXmlParserException {
    __cordl_parent: crate::System::SystemException,
    pub line: i32,
    pub column: i32,
}
#[cfg(feature = "Mono+Xml+SmallXmlParserException")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::Xml::SmallXmlParserException {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Xml";
    const CLASS_NAME: &'static str = "SmallXmlParserException";
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
#[cfg(feature = "Mono+Xml+SmallXmlParserException")]
impl std::ops::Deref for crate::Mono::Xml::SmallXmlParserException {
    type Target = crate::System::SystemException;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Xml+SmallXmlParserException")]
impl std::ops::DerefMut for crate::Mono::Xml::SmallXmlParserException {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Xml+SmallXmlParserException")]
impl crate::Mono::Xml::SmallXmlParserException {
    pub fn New(
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        line: i32,
        column: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (msg, line, column))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        line: i32,
        column: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (msg, line, column))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Xml+SmallXmlParserException")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Xml::SmallXmlParserException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
