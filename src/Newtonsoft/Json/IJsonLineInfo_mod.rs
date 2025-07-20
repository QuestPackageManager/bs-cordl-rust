#[cfg(feature = "Newtonsoft+Json+IJsonLineInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct IJsonLineInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+IJsonLineInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::Newtonsoft::Json::IJsonLineInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json";
    const CLASS_NAME: &'static str = "IJsonLineInfo";
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
#[cfg(feature = "Newtonsoft+Json+IJsonLineInfo")]
impl std::ops::Deref for crate::Newtonsoft::Json::IJsonLineInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+IJsonLineInfo")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::IJsonLineInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+IJsonLineInfo")]
impl crate::Newtonsoft::Json::IJsonLineInfo {
    pub fn HasLineInfo(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::IJsonLineInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("HasLineInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::IJsonLineInfo as
                    quest_hook::libil2cpp::Type > ::class(), "HasLineInfo", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_LineNumber(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::IJsonLineInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_LineNumber")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::IJsonLineInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_LineNumber", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_LinePosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::IJsonLineInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_LinePosition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::IJsonLineInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_LinePosition", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+IJsonLineInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::IJsonLineInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
