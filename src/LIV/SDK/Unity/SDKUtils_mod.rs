#[cfg(feature = "LIV+SDK+Unity+SDKUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct SDKUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "LIV+SDK+Unity+SDKUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKUtils => "LIV.SDK.Unity"
    ."SDKUtils"
);
#[cfg(feature = "LIV+SDK+Unity+SDKUtils")]
impl std::ops::Deref for crate::LIV::SDK::Unity::SDKUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKUtils")]
impl std::ops::DerefMut for crate::LIV::SDK::Unity::SDKUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKUtils")]
impl crate::LIV::SDK::Unity::SDKUtils {}
#[cfg(feature = "LIV+SDK+Unity+SDKUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::LIV::SDK::Unity::SDKUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}