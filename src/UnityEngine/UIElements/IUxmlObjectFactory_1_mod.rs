#[cfg(feature = "UnityEngine+UIElements+IUxmlObjectFactory_1")]
#[repr(C)]
#[derive(Debug)]
pub struct IUxmlObjectFactory_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+IUxmlObjectFactory_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::IUxmlObjectFactory_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "IUxmlObjectFactory`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.UIElements",
                        "IUxmlObjectFactory`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
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
#[cfg(feature = "UnityEngine+UIElements+IUxmlObjectFactory_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::IUxmlObjectFactory_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IUxmlObjectFactory_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::IUxmlObjectFactory_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IUxmlObjectFactory_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::IUxmlObjectFactory_1<T> {
    pub fn CreateObject(
        &mut self,
        bag: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IUxmlAttributes>,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::IUxmlAttributes,
                            >,
                            crate::UnityEngine::UIElements::CreationContext,
                        ),
                        T,
                        2usize,
                    >("CreateObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateObject", 2usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked(self, (bag, cc))? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IUxmlObjectFactory_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IUxmlObjectFactory_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+IUxmlObjectFactory_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::UnityEngine::UIElements::IBaseUxmlFactory>
for crate::UnityEngine::UIElements::IUxmlObjectFactory_1<T> {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IBaseUxmlFactory {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IUxmlObjectFactory_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::UnityEngine::UIElements::IBaseUxmlFactory>
for crate::UnityEngine::UIElements::IUxmlObjectFactory_1<T> {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IBaseUxmlFactory {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IUxmlObjectFactory_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::UnityEngine::UIElements::IBaseUxmlObjectFactory>
for crate::UnityEngine::UIElements::IUxmlObjectFactory_1<T> {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IBaseUxmlObjectFactory {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IUxmlObjectFactory_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::UnityEngine::UIElements::IBaseUxmlObjectFactory>
for crate::UnityEngine::UIElements::IUxmlObjectFactory_1<T> {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IBaseUxmlObjectFactory {
        unsafe { std::mem::transmute(self) }
    }
}
