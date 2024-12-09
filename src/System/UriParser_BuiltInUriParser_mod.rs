#[cfg(feature = "System+UriParser+BuiltInUriParser")]
#[repr(C)]
#[derive(Debug)]
pub struct UriParser_BuiltInUriParser {
    __cordl_parent: crate::System::UriParser,
}
#[cfg(feature = "System+UriParser+BuiltInUriParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::UriParser_BuiltInUriParser =>
    "System"."UriParser/BuiltInUriParser"
);
#[cfg(feature = "System+UriParser+BuiltInUriParser")]
impl std::ops::Deref for crate::GlobalNamespace::UriParser_BuiltInUriParser {
    type Target = crate::System::UriParser;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+UriParser+BuiltInUriParser")]
impl std::ops::DerefMut for crate::GlobalNamespace::UriParser_BuiltInUriParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+UriParser+BuiltInUriParser")]
impl crate::GlobalNamespace::UriParser_BuiltInUriParser {
    pub fn New(
        lwrCaseScheme: *mut quest_hook::libil2cpp::Il2CppString,
        defaultPort: i32,
        syntaxFlags: crate::System::UriSyntaxFlags,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lwrCaseScheme, defaultPort, syntaxFlags))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        lwrCaseScheme: *mut quest_hook::libil2cpp::Il2CppString,
        defaultPort: i32,
        syntaxFlags: crate::System::UriSyntaxFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lwrCaseScheme, defaultPort, syntaxFlags))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+UriParser+BuiltInUriParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::UriParser_BuiltInUriParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
