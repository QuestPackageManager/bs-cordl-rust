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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::CancellationTokenSource_Linked2CancellationTokenSource =>
    "System.Threading"."CancellationTokenSource/Linked2CancellationTokenSource"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        token1: crate::System::Threading::CancellationToken,
        token2: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (token1, token2))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        token1: crate::System::Threading::CancellationToken,
        token2: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (token1, token2))?;
        Ok(__cordl_object)
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
