#[cfg(feature = "cordl_class_Unity+Properties+PropertyBag_1")]
#[repr(C)]
#[derive(Debug)]
pub struct PropertyBag_1<TContainer: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _InstantiationKind_k__BackingField: crate::Unity::Properties::InstantiationKind,
    __cordl_phantom_TContainer: std::marker::PhantomData<TContainer>,
}
#[cfg(feature = "cordl_class_Unity+Properties+PropertyBag_1")]
unsafe impl<TContainer: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "PropertyBag`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Unity.Properties",
                        "PropertyBag`1",
                    )
                    .unwrap()
                    .make_generic::<(TContainer)>()
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
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<TContainer: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<TContainer: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::PropertyBag_1<TContainer> {
    pub fn Accept(
        &mut self,
        visitor: quest_hook::libil2cpp::Gc<crate::Unity::Properties::ITypeVisitor>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Unity::Properties::ITypeVisitor,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Accept")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Accept",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (visitor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate(&mut self) -> quest_hook::libil2cpp::Result<TContainer>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), TContainer, 0usize>("Instantiate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Instantiate", 0usize
                        )
                    })
            });
        let __cordl_ret: TContainer = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Unity_Properties_IConstructor_TContainer__Instantiate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<TContainer>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        TContainer,
                        0usize,
                    >("Unity.Properties.IConstructor<TContainer>.Instantiate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Properties.IConstructor<TContainer>.Instantiate",
                            0usize
                        )
                    })
            });
        let __cordl_ret: TContainer = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_IConstructor_get_InstantiationKind(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Properties::InstantiationKind>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Properties::InstantiationKind,
                        0usize,
                    >("Unity.Properties.IConstructor.get_InstantiationKind")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Properties.IConstructor.get_InstantiationKind", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Properties::InstantiationKind = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_Internal_IPropertyBagRegister_Register(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("Unity.Properties.Internal.IPropertyBagRegister.Register")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Properties.Internal.IPropertyBagRegister.Register",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_InstantiationKind(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Properties::InstantiationKind>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Properties::InstantiationKind,
                        0usize,
                    >("get_InstantiationKind")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_InstantiationKind", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Properties::InstantiationKind = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+PropertyBag_1")]
impl<TContainer: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IConstructor>
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_ref(&self) -> &crate::Unity::Properties::IConstructor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IConstructor>
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IConstructor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IConstructor_1<TContainer>>
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_ref(&self) -> &crate::Unity::Properties::IConstructor_1<TContainer> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IConstructor_1<TContainer>>
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IConstructor_1<TContainer> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IPropertyBag>
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_ref(&self) -> &crate::Unity::Properties::IPropertyBag {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IPropertyBag>
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IPropertyBag {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IPropertyBag_1<TContainer>>
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_ref(&self) -> &crate::Unity::Properties::IPropertyBag_1<TContainer> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IPropertyBag_1<TContainer>>
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IPropertyBag_1<TContainer> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::Internal::IPropertyBagRegister>
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_ref(&self) -> &crate::Unity::Properties::Internal::IPropertyBagRegister {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::Internal::IPropertyBagRegister>
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::Internal::IPropertyBagRegister {
        unsafe { std::mem::transmute(self) }
    }
}
