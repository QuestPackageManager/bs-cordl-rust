#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncSceneContext {
    __cordl_parent: crate::Zenject::SceneContext,
    pub _asyncPreloaders: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::BGLib::AppFlow::Initialization::AsyncPreloader,
            >,
        >,
    >,
    pub _asyncInstallers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::BGLib::AppFlow::Initialization::AsyncInstaller,
            >,
        >,
    >,
    pub _state: crate::BGLib::AppFlow::Initialization::AsyncSceneContext_State,
    pub _registry: quest_hook::libil2cpp::Gc<
        crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry,
    >,
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::AppFlow::Initialization::AsyncSceneContext {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.AppFlow.Initialization";
    const CLASS_NAME: &'static str = "AsyncSceneContext";
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
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext")]
impl std::ops::Deref for crate::BGLib::AppFlow::Initialization::AsyncSceneContext {
    type Target = crate::Zenject::SceneContext;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext")]
impl std::ops::DerefMut for crate::BGLib::AppFlow::Initialization::AsyncSceneContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext")]
impl crate::BGLib::AppFlow::Initialization::AsyncSceneContext {
    #[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext+State")]
    pub type State = crate::BGLib::AppFlow::Initialization::AsyncSceneContext_State;
    pub fn CreateContainerForLoading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BGLib::AppFlow::Initialization::AsyncSceneContext as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                0usize,
            >("CreateContainerForLoading")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BGLib::AppFlow::Initialization::AsyncSceneContext as
                    quest_hook::libil2cpp::Type > ::class(), "CreateContainerForLoading",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRegistry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BGLib::AppFlow::Initialization::AsyncSceneContext as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry,
                >,
                0usize,
            >("CreateRegistry")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BGLib::AppFlow::Initialization::AsyncSceneContext as
                    quest_hook::libil2cpp::Type > ::class(), "CreateRegistry", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn InstallInstallers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BGLib::AppFlow::Initialization::AsyncSceneContext as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("InstallInstallers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BGLib::AppFlow::Initialization::AsyncSceneContext as
                    quest_hook::libil2cpp::Type > ::class(), "InstallInstallers", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadInstallersAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BGLib::AppFlow::Initialization::AsyncSceneContext as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        quest_hook::libil2cpp::Gc<
                            crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry,
                        >,
                    >,
                >,
                0usize,
            >("LoadInstallersAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BGLib::AppFlow::Initialization::AsyncSceneContext as
                    quest_hook::libil2cpp::Type > ::class(), "LoadInstallersAsync",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Run(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BGLib::AppFlow::Initialization::AsyncSceneContext as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Run")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BGLib::AppFlow::Initialization::AsyncSceneContext as
                    quest_hook::libil2cpp::Type > ::class(), "Run", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RunAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BGLib::AppFlow::Initialization::AsyncSceneContext as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                0usize,
            >("RunAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BGLib::AppFlow::Initialization::AsyncSceneContext as
                    quest_hook::libil2cpp::Type > ::class(), "RunAsync", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn __n__0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BGLib::AppFlow::Initialization::AsyncSceneContext as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("<>n__0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BGLib::AppFlow::Initialization::AsyncSceneContext as
                    quest_hook::libil2cpp::Type > ::class(), "<>n__0", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BGLib::AppFlow::Initialization::AsyncSceneContext as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BGLib::AppFlow::Initialization::AsyncSceneContext as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::AppFlow::Initialization::AsyncSceneContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext+State")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AsyncSceneContext_State {
    #[default]
    Initialized = 2i32,
    Initializing = 1i32,
    NotInitialized = 0i32,
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext+State")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::AppFlow::Initialization::AsyncSceneContext_State {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BGLib.AppFlow.Initialization";
    const CLASS_NAME: &'static str = "AsyncSceneContext/State";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext+State")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BGLib::AppFlow::Initialization::AsyncSceneContext_State {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext+State")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BGLib::AppFlow::Initialization::AsyncSceneContext_State {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext+State")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BGLib::AppFlow::Initialization::AsyncSceneContext_State {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext+State")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BGLib::AppFlow::Initialization::AsyncSceneContext_State {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
