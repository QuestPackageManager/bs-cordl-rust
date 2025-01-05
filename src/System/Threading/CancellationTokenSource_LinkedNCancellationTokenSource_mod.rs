#[cfg(
    feature = "System+Threading+CancellationTokenSource+LinkedNCancellationTokenSource"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CancellationTokenSource_LinkedNCancellationTokenSource {
    __cordl_parent: crate::System::Threading::CancellationTokenSource,
}
#[cfg(
    feature = "System+Threading+CancellationTokenSource+LinkedNCancellationTokenSource"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::CancellationTokenSource_LinkedNCancellationTokenSource =>
    "System.Threading"."CancellationTokenSource/LinkedNCancellationTokenSource"
);
#[cfg(
    feature = "System+Threading+CancellationTokenSource+LinkedNCancellationTokenSource"
)]
impl std::ops::Deref
for crate::GlobalNamespace::CancellationTokenSource_LinkedNCancellationTokenSource {
    type Target = crate::System::Threading::CancellationTokenSource;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Threading+CancellationTokenSource+LinkedNCancellationTokenSource"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::CancellationTokenSource_LinkedNCancellationTokenSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Threading+CancellationTokenSource+LinkedNCancellationTokenSource"
)]
impl crate::GlobalNamespace::CancellationTokenSource_LinkedNCancellationTokenSource {}
#[cfg(
    feature = "System+Threading+CancellationTokenSource+LinkedNCancellationTokenSource"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::CancellationTokenSource_LinkedNCancellationTokenSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
