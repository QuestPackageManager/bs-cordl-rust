#[cfg(feature = "cordl_class_Zenject+GetterProvider_2")]
#[repr(C)]
#[derive(Debug)]
pub struct GetterProvider_2<TObj: quest_hook::libil2cpp::Type, TResult: quest_hook::libil2cpp::Type>
{
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub _identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _method: quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TResult>>,
    pub _matchAll: bool,
    pub _sourceType: crate::Zenject::InjectSources,
    __cordl_phantom_TObj: std::marker::PhantomData<TObj>,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "cordl_class_Zenject+GetterProvider_2")]
unsafe impl<TObj: quest_hook::libil2cpp::Type, TResult: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Type for crate::Zenject::GetterProvider_2<TObj, TResult>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "GetterProvider`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("Zenject", "GetterProvider`2")
                .unwrap()
                .make_generic::<(TObj, TResult)>()
                .unwrap()
                .unwrap()
        })
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Zenject+GetterProvider_2")]
impl<TObj: quest_hook::libil2cpp::Type, TResult: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::Zenject::GetterProvider_2<TObj, TResult>
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+GetterProvider_2")]
impl<TObj: quest_hook::libil2cpp::Type, TResult: quest_hook::libil2cpp::Type> std::ops::DerefMut
    for crate::Zenject::GetterProvider_2<TObj, TResult>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+GetterProvider_2")]
impl<TObj: quest_hook::libil2cpp::Type, TResult: quest_hook::libil2cpp::Type>
    crate::Zenject::GetterProvider_2<TObj, TResult>
{
    pub fn GetAllInstancesWithInjectSplit(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
        injectAction: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Action>,
        >,
        buffer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObj: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::Zenject::TypeValuePair,
                            >,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::System::Action>,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "GetAllInstancesWithInjectSplit"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAllInstancesWithInjectSplit",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (context, args, injectAction, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetInstanceType(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>>
    where
        TObj: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>),
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        1usize,
                    >("GetInstanceType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetInstanceType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> =
            unsafe { cordl_method_info.invoke_unchecked(self, (context))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSubContext(
        &mut self,
        parent: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>>
    where
        TObj: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>),
                        quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                        1usize,
                    >("GetSubContext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSubContext", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext> =
            unsafe { cordl_method_info.invoke_unchecked(self, (parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TResult>>,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        sourceType: crate::Zenject::InjectSources,
        matchAll: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TObj: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (identifier, method, container, sourceType, matchAll),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TResult>>,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        sourceType: crate::Zenject::InjectSources,
        matchAll: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObj: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TResult>>,
                        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                        crate::Zenject::InjectSources,
                        bool,
                    ), quest_hook::libil2cpp::Void, 5usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (identifier, method, container, sourceType, matchAll))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCached(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TObj: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsCached")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_IsCached",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_TypeVariesBasedOnMemberType(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TObj: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_TypeVariesBasedOnMemberType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_TypeVariesBasedOnMemberType",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Zenject+GetterProvider_2")]
impl<TObj: quest_hook::libil2cpp::Type, TResult: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::ObjectType for crate::Zenject::GetterProvider_2<TObj, TResult>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+GetterProvider_2")]
impl<TObj: quest_hook::libil2cpp::Type, TResult: quest_hook::libil2cpp::Type>
    AsRef<crate::Zenject::IProvider> for crate::Zenject::GetterProvider_2<TObj, TResult>
{
    fn as_ref(&self) -> &crate::Zenject::IProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+GetterProvider_2")]
impl<TObj: quest_hook::libil2cpp::Type, TResult: quest_hook::libil2cpp::Type>
    AsMut<crate::Zenject::IProvider> for crate::Zenject::GetterProvider_2<TObj, TResult>
{
    fn as_mut(&mut self) -> &mut crate::Zenject::IProvider {
        unsafe { std::mem::transmute(self) }
    }
}
