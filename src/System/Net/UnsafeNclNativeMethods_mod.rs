#[cfg(
    feature = "cordl_class_System+Net+UnsafeNclNativeMethods+HttpApi+HTTP_REQUEST_HEADER_ID"
)]
#[repr(C)]
#[derive(Debug)]
pub struct HttpApi_UnsafeNclNativeMethods_HTTP_REQUEST_HEADER_ID {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_System+Net+UnsafeNclNativeMethods+HttpApi+HTTP_REQUEST_HEADER_ID"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::HttpApi_UnsafeNclNativeMethods_HTTP_REQUEST_HEADER_ID {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "UnsafeNclNativeMethods/HttpApi/HTTP_REQUEST_HEADER_ID";
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
#[cfg(
    feature = "cordl_class_System+Net+UnsafeNclNativeMethods+HttpApi+HTTP_REQUEST_HEADER_ID"
)]
impl std::ops::Deref
for crate::System::Net::HttpApi_UnsafeNclNativeMethods_HTTP_REQUEST_HEADER_ID {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "cordl_class_System+Net+UnsafeNclNativeMethods+HttpApi+HTTP_REQUEST_HEADER_ID"
)]
impl std::ops::DerefMut
for crate::System::Net::HttpApi_UnsafeNclNativeMethods_HTTP_REQUEST_HEADER_ID {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+UnsafeNclNativeMethods+HttpApi+HTTP_REQUEST_HEADER_ID")]
impl crate::System::Net::HttpApi_UnsafeNclNativeMethods_HTTP_REQUEST_HEADER_ID {
    pub fn ToString(
        position: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked((), (position))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_System+Net+UnsafeNclNativeMethods+HttpApi+HTTP_REQUEST_HEADER_ID"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::HttpApi_UnsafeNclNativeMethods_HTTP_REQUEST_HEADER_ID {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_System+Net+UnsafeNclNativeMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct UnsafeNclNativeMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_System+Net+UnsafeNclNativeMethods")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::UnsafeNclNativeMethods {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "UnsafeNclNativeMethods";
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
#[cfg(feature = "cordl_class_System+Net+UnsafeNclNativeMethods")]
impl std::ops::Deref for crate::System::Net::UnsafeNclNativeMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_System+Net+UnsafeNclNativeMethods")]
impl std::ops::DerefMut for crate::System::Net::UnsafeNclNativeMethods {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+UnsafeNclNativeMethods")]
impl crate::System::Net::UnsafeNclNativeMethods {
    #[cfg(feature = "System+Net+UnsafeNclNativeMethods+HttpApi")]
    pub type HttpApi = crate::System::Net::UnsafeNclNativeMethods_HttpApi;
    #[cfg(feature = "System+Net+UnsafeNclNativeMethods+SecureStringHelper")]
    pub type SecureStringHelper = crate::System::Net::UnsafeNclNativeMethods_SecureStringHelper;
}
#[cfg(feature = "cordl_class_System+Net+UnsafeNclNativeMethods")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::UnsafeNclNativeMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_System+Net+UnsafeNclNativeMethods+HttpApi")]
#[repr(C)]
#[derive(Debug)]
pub struct UnsafeNclNativeMethods_HttpApi {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_System+Net+UnsafeNclNativeMethods+HttpApi")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::UnsafeNclNativeMethods_HttpApi {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "UnsafeNclNativeMethods/HttpApi";
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
#[cfg(feature = "cordl_class_System+Net+UnsafeNclNativeMethods+HttpApi")]
impl std::ops::Deref for crate::System::Net::UnsafeNclNativeMethods_HttpApi {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_System+Net+UnsafeNclNativeMethods+HttpApi")]
impl std::ops::DerefMut for crate::System::Net::UnsafeNclNativeMethods_HttpApi {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+UnsafeNclNativeMethods+HttpApi")]
impl crate::System::Net::UnsafeNclNativeMethods_HttpApi {
    #[cfg(feature = "System+Net+UnsafeNclNativeMethods+HttpApi+HTTP_REQUEST_HEADER_ID")]
    pub type HTTP_REQUEST_HEADER_ID = crate::System::Net::HttpApi_UnsafeNclNativeMethods_HTTP_REQUEST_HEADER_ID;
}
#[cfg(feature = "cordl_class_System+Net+UnsafeNclNativeMethods+HttpApi")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::UnsafeNclNativeMethods_HttpApi {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_System+Net+UnsafeNclNativeMethods+SecureStringHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct UnsafeNclNativeMethods_SecureStringHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_System+Net+UnsafeNclNativeMethods+SecureStringHelper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::UnsafeNclNativeMethods_SecureStringHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "UnsafeNclNativeMethods/SecureStringHelper";
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
#[cfg(feature = "cordl_class_System+Net+UnsafeNclNativeMethods+SecureStringHelper")]
impl std::ops::Deref for crate::System::Net::UnsafeNclNativeMethods_SecureStringHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_System+Net+UnsafeNclNativeMethods+SecureStringHelper")]
impl std::ops::DerefMut
for crate::System::Net::UnsafeNclNativeMethods_SecureStringHelper {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+UnsafeNclNativeMethods+SecureStringHelper")]
impl crate::System::Net::UnsafeNclNativeMethods_SecureStringHelper {
    pub fn CreateSecureString(
        plainString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::SecureString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<crate::System::Security::SecureString>,
                        1usize,
                    >("CreateSecureString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateSecureString", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::SecureString,
        > = unsafe { cordl_method_info.invoke_unchecked((), (plainString))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateString(
        secureString: quest_hook::libil2cpp::Gc<crate::System::Security::SecureString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Security::SecureString,
                        >),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("CreateString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateString", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked((), (secureString))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Net+UnsafeNclNativeMethods+SecureStringHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::UnsafeNclNativeMethods_SecureStringHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
