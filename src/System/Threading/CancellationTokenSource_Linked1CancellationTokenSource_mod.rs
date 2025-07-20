#[cfg(
    feature = "System+Threading+CancellationTokenSource+Linked1CancellationTokenSource"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CancellationTokenSource_Linked1CancellationTokenSource {
    __cordl_parent: crate::System::Threading::CancellationTokenSource,
    pub _reg1: crate::System::Threading::CancellationTokenRegistration,
}
#[cfg(
    feature = "System+Threading+CancellationTokenSource+Linked1CancellationTokenSource"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::CancellationTokenSource_Linked1CancellationTokenSource {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "CancellationTokenSource/Linked1CancellationTokenSource";
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
    feature = "System+Threading+CancellationTokenSource+Linked1CancellationTokenSource"
)]
impl std::ops::Deref
for crate::GlobalNamespace::CancellationTokenSource_Linked1CancellationTokenSource {
    type Target = crate::System::Threading::CancellationTokenSource;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Threading+CancellationTokenSource+Linked1CancellationTokenSource"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::CancellationTokenSource_Linked1CancellationTokenSource {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Threading+CancellationTokenSource+Linked1CancellationTokenSource"
)]
impl crate::GlobalNamespace::CancellationTokenSource_Linked1CancellationTokenSource {
    pub fn Dispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Dispose", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (disposing))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        token1: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (token1))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        token1: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::System::Threading::CancellationToken),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (token1))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "System+Threading+CancellationTokenSource+Linked1CancellationTokenSource"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::CancellationTokenSource_Linked1CancellationTokenSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
