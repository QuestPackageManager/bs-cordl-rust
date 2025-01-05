#[cfg(feature = "Newtonsoft+Json+Utilities+StringReferenceExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct StringReferenceExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringReferenceExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Utilities::StringReferenceExtensions =>
    "Newtonsoft.Json.Utilities"."StringReferenceExtensions"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+StringReferenceExtensions")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::StringReferenceExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndsWith", (s, text))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf(
        s: crate::Newtonsoft::Json::Utilities::StringReference,
        c: char,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (s, c, startIndex, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartsWith(
        s: crate::Newtonsoft::Json::Utilities::StringReference,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StartsWith", (s, text))?;
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
