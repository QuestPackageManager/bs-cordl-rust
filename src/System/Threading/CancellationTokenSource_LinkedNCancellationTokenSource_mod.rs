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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::CancellationTokenSource_LinkedNCancellationTokenSource {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "CancellationTokenSource/LinkedNCancellationTokenSource";
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
    feature = "System+Threading+CancellationTokenSource+LinkedNCancellationTokenSource"
)]
impl std::ops::Deref
for crate::GlobalNamespace::CancellationTokenSource_LinkedNCancellationTokenSource {
    type Target = crate::System::Threading::CancellationTokenSource;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Threading+CancellationTokenSource+LinkedNCancellationTokenSource"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::CancellationTokenSource_LinkedNCancellationTokenSource {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
