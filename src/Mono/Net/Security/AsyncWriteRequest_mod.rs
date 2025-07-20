#[cfg(feature = "Mono+Net+Security+AsyncWriteRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncWriteRequest {
    __cordl_parent: crate::Mono::Net::Security::AsyncReadOrWriteRequest,
}
#[cfg(feature = "Mono+Net+Security+AsyncWriteRequest")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Net::Security::AsyncWriteRequest {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Net.Security";
    const CLASS_NAME: &'static str = "AsyncWriteRequest";
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
#[cfg(feature = "Mono+Net+Security+AsyncWriteRequest")]
impl std::ops::Deref for crate::Mono::Net::Security::AsyncWriteRequest {
    type Target = crate::Mono::Net::Security::AsyncReadOrWriteRequest;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+AsyncWriteRequest")]
impl std::ops::DerefMut for crate::Mono::Net::Security::AsyncWriteRequest {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+AsyncWriteRequest")]
impl crate::Mono::Net::Security::AsyncWriteRequest {
    pub fn New(
        parent: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileAuthenticatedStream,
        >,
        sync: bool,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parent, sync, buffer, offset, _cordl_size))?;
        Ok(__cordl_object.into())
    }
    pub fn Run(
        &mut self,
        status: crate::Mono::Net::Security::AsyncOperationStatus,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Net::Security::AsyncOperationStatus,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::Mono::Net::Security::AsyncOperationStatus),
                        crate::Mono::Net::Security::AsyncOperationStatus,
                        1usize,
                    >("Run")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Run", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Mono::Net::Security::AsyncOperationStatus = unsafe {
            method.invoke_unchecked(self, (status))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        parent: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileAuthenticatedStream,
        >,
        sync: bool,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Mono::Net::Security::MobileAuthenticatedStream,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (parent, sync, buffer, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Net+Security+AsyncWriteRequest")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Net::Security::AsyncWriteRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
