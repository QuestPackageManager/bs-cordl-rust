#[cfg(feature = "cordl_class_BGLib+DotnetExtension+Disposables+DisposableExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct DisposableExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_BGLib+DotnetExtension+Disposables+DisposableExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::DotnetExtension::Disposables::DisposableExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.DotnetExtension.Disposables";
    const CLASS_NAME: &'static str = "DisposableExtensions";
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
#[cfg(feature = "BGLib+DotnetExtension+Disposables+DisposableExtensions")]
impl std::ops::Deref
for crate::BGLib::DotnetExtension::Disposables::DisposableExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+DotnetExtension+Disposables+DisposableExtensions")]
impl std::ops::DerefMut
for crate::BGLib::DotnetExtension::Disposables::DisposableExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+DotnetExtension+Disposables+DisposableExtensions")]
impl crate::BGLib::DotnetExtension::Disposables::DisposableExtensions {
    pub fn AddTo_CompositeDisposable0(
        disposable: quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
        compositeDisposable: quest_hook::libil2cpp::Gc<
            crate::BGLib::DotnetExtension::Disposables::CompositeDisposable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
                            quest_hook::libil2cpp::Gc<
                                crate::BGLib::DotnetExtension::Disposables::CompositeDisposable,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("AddTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "AddTo",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (disposable, compositeDisposable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddTo_ConcurrentCompositeDisposable1(
        disposable: quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
        compositeDisposable: quest_hook::libil2cpp::Gc<
            crate::BGLib::DotnetExtension::Disposables::ConcurrentCompositeDisposable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
                            quest_hook::libil2cpp::Gc<
                                crate::BGLib::DotnetExtension::Disposables::ConcurrentCompositeDisposable,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("AddTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "AddTo",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (disposable, compositeDisposable))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BGLib+DotnetExtension+Disposables+DisposableExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::DotnetExtension::Disposables::DisposableExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
