#[cfg(feature = "System+Net+HeaderInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct HeaderInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub IsRequestRestricted: bool,
    pub IsResponseRestricted: bool,
    pub Parser: quest_hook::libil2cpp::Gc<crate::System::Net::HeaderParser>,
    pub HeaderName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub AllowMultiValues: bool,
}
#[cfg(feature = "System+Net+HeaderInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::HeaderInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "HeaderInfo";
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
#[cfg(feature = "System+Net+HeaderInfo")]
impl std::ops::Deref for crate::System::Net::HeaderInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HeaderInfo")]
impl std::ops::DerefMut for crate::System::Net::HeaderInfo {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HeaderInfo")]
impl crate::System::Net::HeaderInfo {
    pub fn New(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        requestRestricted: bool,
        responseRestricted: bool,
        multi: bool,
        p: quest_hook::libil2cpp::Gc<crate::System::Net::HeaderParser>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (name, requestRestricted, responseRestricted, multi, p),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        requestRestricted: bool,
        responseRestricted: bool,
        multi: bool,
        p: quest_hook::libil2cpp::Gc<crate::System::Net::HeaderParser>,
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
                            bool,
                            bool,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::System::Net::HeaderParser>,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (name, requestRestricted, responseRestricted, multi, p),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+HeaderInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::HeaderInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
