#[cfg(feature = "UnityEngine+AddressableAssets+Utility+SerializationUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct SerializationUtilities {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+SerializationUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::Utility::SerializationUtilities =>
    "UnityEngine.AddressableAssets.Utility"."SerializationUtilities"
);
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+SerializationUtilities")]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::Utility::SerializationUtilities {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+SerializationUtilities")]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::Utility::SerializationUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+SerializationUtilities")]
impl crate::UnityEngine::AddressableAssets::Utility::SerializationUtilities {
    #[cfg(
        feature = "UnityEngine+AddressableAssets+Utility+SerializationUtilities+ObjectType"
    )]
    pub type ObjectType = crate::UnityEngine::AddressableAssets::Utility::SerializationUtilities_ObjectType;
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+SerializationUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::Utility::SerializationUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+Utility+SerializationUtilities+ObjectType"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerializationUtilities_ObjectType {
    AsciiString = 0i32,
    Hash128 = 5i32,
    Int32 = 4i32,
    JsonObject = 7i32,
    Type = 6i32,
    UInt16 = 2i32,
    UInt32 = 3i32,
    UnicodeString = 1i32,
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+Utility+SerializationUtilities+ObjectType"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::Utility::SerializationUtilities_ObjectType =>
    "UnityEngine.AddressableAssets.Utility"."SerializationUtilities/ObjectType"
);
