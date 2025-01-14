#[cfg(feature = "Newtonsoft+Json+Utilities+StringReferenceExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct StringReferenceExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringReferenceExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Utilities::StringReferenceExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "StringReferenceExtensions";
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
#[cfg(feature = "Newtonsoft+Json+Utilities+StringReferenceExtensions")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::StringReferenceExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringReferenceExtensions")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Utilities::StringReferenceExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringReferenceExtensions")]
impl crate::Newtonsoft::Json::Utilities::StringReferenceExtensions {
    pub fn EndsWith(
        s: crate::Newtonsoft::Json::Utilities::StringReference,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Newtonsoft::Json::Utilities::StringReference,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                bool,
                2usize,
            >("EndsWith")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EndsWith", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (s, text)) };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf(
        s: crate::Newtonsoft::Json::Utilities::StringReference,
        c: char,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Newtonsoft::Json::Utilities::StringReference, char, i32, i32),
                i32,
                4usize,
            >("IndexOf")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IndexOf", 4usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (s, c, startIndex, length))
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartsWith(
        s: crate::Newtonsoft::Json::Utilities::StringReference,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Newtonsoft::Json::Utilities::StringReference,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                bool,
                2usize,
            >("StartsWith")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "StartsWith", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (s, text)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringReferenceExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::StringReferenceExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
