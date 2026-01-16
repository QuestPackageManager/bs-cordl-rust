#[cfg(feature = "cordl_class_UnityEngine+Networking+UnityWebRequestAsyncOperation")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityWebRequestAsyncOperation {
    __cordl_parent: crate::UnityEngine::AsyncOperation,
    pub _webRequest_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Networking::UnityWebRequest,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Networking+UnityWebRequestAsyncOperation")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Networking::UnityWebRequestAsyncOperation {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Networking";
    const CLASS_NAME: &'static str = "UnityWebRequestAsyncOperation";
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
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestAsyncOperation")]
impl std::ops::Deref for crate::UnityEngine::Networking::UnityWebRequestAsyncOperation {
    type Target = crate::UnityEngine::AsyncOperation;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestAsyncOperation")]
impl std::ops::DerefMut
for crate::UnityEngine::Networking::UnityWebRequestAsyncOperation {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestAsyncOperation")]
impl crate::UnityEngine::Networking::UnityWebRequestAsyncOperation {
    #[cfg(
        feature = "UnityEngine+Networking+UnityWebRequestAsyncOperation+BindingsMarshaller"
    )]
    pub type BindingsMarshaller = crate::UnityEngine::Networking::UnityWebRequestAsyncOperation_BindingsMarshaller;
    pub fn New(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ptr))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (ptr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_webRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Networking::UnityWebRequest>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Networking::UnityWebRequest,
                        >,
                        0usize,
                    >("get_webRequest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_webRequest", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequest,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_webRequest(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Networking::UnityWebRequest>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Networking::UnityWebRequest,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_webRequest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_webRequest", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Networking+UnityWebRequestAsyncOperation")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::UnityWebRequestAsyncOperation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Networking+UnityWebRequestAsyncOperation+BindingsMarshaller"
)]
#[repr(C)]
#[derive(Debug)]
pub struct UnityWebRequestAsyncOperation_BindingsMarshaller {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Networking+UnityWebRequestAsyncOperation+BindingsMarshaller"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Networking::UnityWebRequestAsyncOperation_BindingsMarshaller {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Networking";
    const CLASS_NAME: &'static str = "UnityWebRequestAsyncOperation/BindingsMarshaller";
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
    feature = "UnityEngine+Networking+UnityWebRequestAsyncOperation+BindingsMarshaller"
)]
impl std::ops::Deref
for crate::UnityEngine::Networking::UnityWebRequestAsyncOperation_BindingsMarshaller {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Networking+UnityWebRequestAsyncOperation+BindingsMarshaller"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Networking::UnityWebRequestAsyncOperation_BindingsMarshaller {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Networking+UnityWebRequestAsyncOperation+BindingsMarshaller"
)]
impl crate::UnityEngine::Networking::UnityWebRequestAsyncOperation_BindingsMarshaller {
    pub fn ConvertToManaged(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequestAsyncOperation,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Networking::UnityWebRequestAsyncOperation,
                        >,
                        1usize,
                    >("ConvertToManaged")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertToManaged", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequestAsyncOperation,
        > = unsafe { cordl_method_info.invoke_unchecked((), (ptr))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Networking+UnityWebRequestAsyncOperation+BindingsMarshaller"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::UnityWebRequestAsyncOperation_BindingsMarshaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
