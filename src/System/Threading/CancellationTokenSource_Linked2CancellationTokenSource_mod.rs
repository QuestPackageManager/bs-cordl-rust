#[cfg(
    feature = "System+Threading+CancellationTokenSource+Linked2CancellationTokenSource"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CancellationTokenSource_Linked2CancellationTokenSource {
    __cordl_parent: crate::System::Threading::CancellationTokenSource,
    pub _reg1: crate::System::Threading::CancellationTokenRegistration,
    pub _reg2: crate::System::Threading::CancellationTokenRegistration,
}
#[cfg(
    feature = "System+Threading+CancellationTokenSource+Linked2CancellationTokenSource"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::CancellationTokenSource_Linked2CancellationTokenSource {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "CancellationTokenSource/Linked2CancellationTokenSource";
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
    feature = "System+Threading+CancellationTokenSource+Linked2CancellationTokenSource"
)]
impl std::ops::Deref
for crate::GlobalNamespace::CancellationTokenSource_Linked2CancellationTokenSource {
    type Target = crate::System::Threading::CancellationTokenSource;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Threading+CancellationTokenSource+Linked2CancellationTokenSource"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::CancellationTokenSource_Linked2CancellationTokenSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Threading+CancellationTokenSource+Linked2CancellationTokenSource"
)]
impl crate::GlobalNamespace::CancellationTokenSource_Linked2CancellationTokenSource {
    pub fn Dispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dispose", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (disposing))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        token1: crate::System::Threading::CancellationToken,
        token2: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (token1, token2))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        token1: crate::System::Threading::CancellationToken,
        token2: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::System::Threading::CancellationToken,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (token1, token2))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "System+Threading+CancellationTokenSource+Linked2CancellationTokenSource"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::CancellationTokenSource_Linked2CancellationTokenSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
