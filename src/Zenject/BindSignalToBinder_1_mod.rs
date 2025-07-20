#[cfg(feature = "Zenject+BindSignalToBinder_1")]
#[repr(C)]
#[derive(Debug)]
pub struct BindSignalToBinder_1<TSignal: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub _bindStatement: quest_hook::libil2cpp::Gc<crate::Zenject::BindStatement>,
    pub _signalBindInfo: quest_hook::libil2cpp::Gc<
        crate::Zenject::SignalBindingBindInfo,
    >,
    __cordl_phantom_TSignal: std::marker::PhantomData<TSignal>,
}
#[cfg(feature = "Zenject+BindSignalToBinder_1")]
unsafe impl<TSignal: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::Zenject::BindSignalToBinder_1<TSignal> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "BindSignalToBinder`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Zenject",
                        "BindSignalToBinder`1",
                    )
                    .unwrap()
                    .make_generic::<(TSignal)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "Zenject+BindSignalToBinder_1")]
impl<TSignal: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::BindSignalToBinder_1<TSignal> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+BindSignalToBinder_1")]
impl<TSignal: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::BindSignalToBinder_1<TSignal> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+BindSignalToBinder_1")]
impl<
    TSignal: quest_hook::libil2cpp::Type,
> crate::Zenject::BindSignalToBinder_1<TSignal> {
    pub fn New(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        signalBindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::SignalBindingBindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container, signalBindInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn ToMethod_Action1(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::SignalCopyBinder>,
    >
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Action>),
                        quest_hook::libil2cpp::Gc<crate::Zenject::SignalCopyBinder>,
                        1usize,
                    >("ToMethod")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToMethod", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::SignalCopyBinder> = unsafe {
            cordl_method_info.invoke_unchecked(self, (callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToMethod_Action_1_0(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<TSignal>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::SignalCopyBinder>,
    >
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Action_1<TSignal>>),
                        quest_hook::libil2cpp::Gc<crate::Zenject::SignalCopyBinder>,
                        1usize,
                    >("ToMethod")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToMethod", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::SignalCopyBinder> = unsafe {
            cordl_method_info.invoke_unchecked(self, (callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToMethod_Action_2_2<TObject>(
        &mut self,
        handler: quest_hook::libil2cpp::Gc<crate::System::Action_2<TObject, TSignal>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::BindSignalFromBinder_2<TObject, TSignal>,
        >,
    >
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_2<TObject, TSignal>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::BindSignalFromBinder_2<TObject, TSignal>,
                        >,
                        1usize,
                    >("ToMethod")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToMethod", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::BindSignalFromBinder_2<TObject, TSignal>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (handler))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToMethod_Func_2_3<TObject>(
        &mut self,
        handlerGetter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                TObject,
                quest_hook::libil2cpp::Gc<crate::System::Action>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::BindSignalFromBinder_2<TObject, TSignal>,
        >,
    >
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Func_2<
                                TObject,
                                quest_hook::libil2cpp::Gc<crate::System::Action>,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::BindSignalFromBinder_2<TObject, TSignal>,
                        >,
                        1usize,
                    >("ToMethod")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToMethod", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::BindSignalFromBinder_2<TObject, TSignal>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (handlerGetter))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToMethod_Func_2_4<TObject>(
        &mut self,
        handlerGetter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                TObject,
                quest_hook::libil2cpp::Gc<crate::System::Action_1<TSignal>>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::BindSignalFromBinder_2<TObject, TSignal>,
        >,
    >
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Func_2<
                                TObject,
                                quest_hook::libil2cpp::Gc<crate::System::Action_1<TSignal>>,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::BindSignalFromBinder_2<TObject, TSignal>,
                        >,
                        1usize,
                    >("ToMethod")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToMethod", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::BindSignalFromBinder_2<TObject, TSignal>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (handlerGetter))? };
        Ok(__cordl_ret.into())
    }
    pub fn __zenCreate(
        P_0: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        1usize,
                    >("__zenCreate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "__zenCreate", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked((), (P_0))? };
        Ok(__cordl_ret.into())
    }
    pub fn __zenCreateInjectTypeInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    >
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
                        0usize,
                    >("__zenCreateInjectTypeInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "__zenCreateInjectTypeInfo", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        signalBindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::SignalBindingBindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            quest_hook::libil2cpp::Gc<
                                crate::Zenject::SignalBindingBindInfo,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (container, signalBindInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_SignalBindInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::SignalBindingBindInfo>,
    >
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::Zenject::SignalBindingBindInfo>,
                        0usize,
                    >("get_SignalBindInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_SignalBindInfo", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::SignalBindingBindInfo,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+BindSignalToBinder_1")]
impl<TSignal: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::BindSignalToBinder_1<TSignal> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
