#[cfg(feature = "System+Threading+ThreadPoolGlobals")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreadPoolGlobals {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+ThreadPoolGlobals")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Threading::ThreadPoolGlobals {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "ThreadPoolGlobals";
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
#[cfg(feature = "System+Threading+ThreadPoolGlobals")]
impl std::ops::Deref for crate::System::Threading::ThreadPoolGlobals {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ThreadPoolGlobals")]
impl std::ops::DerefMut for crate::System::Threading::ThreadPoolGlobals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ThreadPoolGlobals")]
impl crate::System::Threading::ThreadPoolGlobals {}
#[cfg(feature = "System+Threading+ThreadPoolGlobals")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::ThreadPoolGlobals {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
