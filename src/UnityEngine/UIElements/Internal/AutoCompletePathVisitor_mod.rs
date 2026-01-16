#[cfg(feature = "cordl_class_UnityEngine+UIElements+Internal+AutoCompletePathVisitor")]
#[repr(C)]
#[derive(Debug)]
pub struct AutoCompletePathVisitor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_VisitContext: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitContext,
    >,
    pub _maxDepth_k__BackingField: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Internal+AutoCompletePathVisitor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Internal";
    const CLASS_NAME: &'static str = "AutoCompletePathVisitor";
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
#[cfg(feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor")]
impl crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor {
    #[cfg(
        feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor+InspectedTypeScope_1"
    )]
    pub type InspectedTypeScope_1<TContainer: quest_hook::libil2cpp::Type> = crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_InspectedTypeScope_1<
        TContainer,
    >;
    #[cfg(
        feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor+VisitContext"
    )]
    pub type VisitContext = crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitContext;
    #[cfg(
        feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor+VisitedPropertyScope"
    )]
    pub type VisitedPropertyScope = crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitedPropertyScope;
    pub fn HasReachedEnd(
        &mut self,
        containerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        bool,
                        1usize,
                    >("HasReachedEnd")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HasReachedEnd", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (containerType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Reset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_IListPropertyVisitor_Visit<TContainer, TList, TElement>(
        &mut self,
        property: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::Property_2<TContainer, TList>,
        >,
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
        list: quest_hook::libil2cpp::ByRefMut<TList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Unity::Properties::Property_2<TContainer, TList>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<TContainer>,
                            quest_hook::libil2cpp::ByRefMut<TList>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Unity.Properties.IListPropertyVisitor.Visit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Properties.IListPropertyVisitor.Visit", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (property, container, list))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_IPropertyBagVisitor_Visit<TContainer>(
        &mut self,
        properties: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag_1<TContainer>,
        >,
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Unity::Properties::IPropertyBag_1<TContainer>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<TContainer>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Unity.Properties.IPropertyBagVisitor.Visit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Properties.IPropertyBagVisitor.Visit", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (properties, container))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_IPropertyVisitor_Visit<TContainer, TValue>(
        &mut self,
        property: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::Property_2<TContainer, TValue>,
        >,
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Unity::Properties::Property_2<TContainer, TValue>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<TContainer>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Unity.Properties.IPropertyVisitor.Visit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Properties.IPropertyVisitor.Visit", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (property, container))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_ITypeVisitor_Visit<TContainer>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("Unity.Properties.ITypeVisitor.Visit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Properties.ITypeVisitor.Visit", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VisitPropertyType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("VisitPropertyType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitPropertyType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (_cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
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
    pub fn get_maxDepth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_maxDepth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_maxDepth", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Internal+AutoCompletePathVisitor")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor")]
impl AsRef<crate::Unity::Properties::IListPropertyVisitor>
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor {
    fn as_ref(&self) -> &crate::Unity::Properties::IListPropertyVisitor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor")]
impl AsMut<crate::Unity::Properties::IListPropertyVisitor>
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IListPropertyVisitor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor")]
impl AsRef<crate::Unity::Properties::IPropertyBagVisitor>
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor {
    fn as_ref(&self) -> &crate::Unity::Properties::IPropertyBagVisitor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor")]
impl AsMut<crate::Unity::Properties::IPropertyBagVisitor>
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IPropertyBagVisitor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor")]
impl AsRef<crate::Unity::Properties::IPropertyVisitor>
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor {
    fn as_ref(&self) -> &crate::Unity::Properties::IPropertyVisitor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor")]
impl AsMut<crate::Unity::Properties::IPropertyVisitor>
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IPropertyVisitor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor")]
impl AsRef<crate::Unity::Properties::ITypeVisitor>
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor {
    fn as_ref(&self) -> &crate::Unity::Properties::ITypeVisitor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor")]
impl AsMut<crate::Unity::Properties::ITypeVisitor>
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::ITypeVisitor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Internal+AutoCompletePathVisitor+InspectedTypeScope_1"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AutoCompletePathVisitor_InspectedTypeScope_1<
    TContainer: quest_hook::libil2cpp::Type,
> {
    pub m_VisitContext: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitContext,
    >,
    __cordl_phantom_TContainer: std::marker::PhantomData<TContainer>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Internal+AutoCompletePathVisitor+InspectedTypeScope_1"
)]
unsafe impl<TContainer: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_InspectedTypeScope_1<
    TContainer,
> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Internal";
    const CLASS_NAME: &'static str = "AutoCompletePathVisitor/InspectedTypeScope`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.UIElements.Internal",
                        "AutoCompletePathVisitor/InspectedTypeScope`1",
                    )
                    .unwrap()
                    .make_generic::<(TContainer)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Internal+AutoCompletePathVisitor+InspectedTypeScope_1"
)]
unsafe impl<TContainer: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_InspectedTypeScope_1<
    TContainer,
> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Internal+AutoCompletePathVisitor+InspectedTypeScope_1"
)]
unsafe impl<TContainer: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_InspectedTypeScope_1<
    TContainer,
> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Internal+AutoCompletePathVisitor+InspectedTypeScope_1"
)]
unsafe impl<TContainer: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_InspectedTypeScope_1<
    TContainer,
> {
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
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Internal+AutoCompletePathVisitor+InspectedTypeScope_1"
)]
unsafe impl<TContainer: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_InspectedTypeScope_1<
    TContainer,
> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Internal+AutoCompletePathVisitor+InspectedTypeScope_1"
)]
unsafe impl<TContainer: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_InspectedTypeScope_1<
    TContainer,
> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor+InspectedTypeScope_1"
)]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_InspectedTypeScope_1<
    TContainer,
> {
    pub fn Dispose(
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
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitContext,
        >,
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
                            crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitContext,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (context))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor+InspectedTypeScope_1"
)]
impl<TContainer: quest_hook::libil2cpp::Type> AsRef<crate::System::IDisposable>
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_InspectedTypeScope_1<
    TContainer,
> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor+InspectedTypeScope_1"
)]
impl<TContainer: quest_hook::libil2cpp::Type> AsMut<crate::System::IDisposable>
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_InspectedTypeScope_1<
    TContainer,
> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Internal+AutoCompletePathVisitor+VisitContext"
)]
#[repr(C)]
#[derive(Debug)]
pub struct AutoCompletePathVisitor_VisitContext {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _propertyPathInfos_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::PropertyPathInfo,
        >,
    >,
    pub _types_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
    >,
    pub _current_k__BackingField: crate::Unity::Properties::PropertyPath,
    pub _currentDepth_k__BackingField: i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Internal+AutoCompletePathVisitor+VisitContext"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitContext {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Internal";
    const CLASS_NAME: &'static str = "AutoCompletePathVisitor/VisitContext";
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
#[cfg(feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor+VisitContext")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitContext {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor+VisitContext")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitContext {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor+VisitContext")]
impl crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitContext {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
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
    pub fn get_current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Properties::PropertyPath> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Properties::PropertyPath,
                        0usize,
                    >("get_current")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_current", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Properties::PropertyPath = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_currentDepth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_currentDepth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_currentDepth", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_propertyPathInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::PropertyPathInfo,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::UIElements::PropertyPathInfo,
                            >,
                        >,
                        0usize,
                    >("get_propertyPathInfos")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_propertyPathInfos", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::PropertyPathInfo,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_types(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::HashSet_1<
                                quest_hook::libil2cpp::Gc<crate::System::Type>,
                            >,
                        >,
                        0usize,
                    >("get_types")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_types", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_current(
        &mut self,
        value: crate::Unity::Properties::PropertyPath,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Properties::PropertyPath),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_current")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_current", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_currentDepth(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_currentDepth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_currentDepth", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_propertyPathInfos(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::PropertyPathInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::UIElements::PropertyPathInfo,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_propertyPathInfos")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_propertyPathInfos", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Internal+AutoCompletePathVisitor+VisitContext"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Internal+AutoCompletePathVisitor+VisitedPropertyScope"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AutoCompletePathVisitor_VisitedPropertyScope {
    pub m_VisitContext: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitContext,
    >,
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Internal+AutoCompletePathVisitor+VisitedPropertyScope"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitedPropertyScope {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Internal";
    const CLASS_NAME: &'static str = "AutoCompletePathVisitor/VisitedPropertyScope";
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
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Internal+AutoCompletePathVisitor+VisitedPropertyScope"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitedPropertyScope {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Internal+AutoCompletePathVisitor+VisitedPropertyScope"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitedPropertyScope {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Internal+AutoCompletePathVisitor+VisitedPropertyScope"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitedPropertyScope {
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
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Internal+AutoCompletePathVisitor+VisitedPropertyScope"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitedPropertyScope {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+UIElements+Internal+AutoCompletePathVisitor+VisitedPropertyScope"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitedPropertyScope {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor+VisitedPropertyScope"
)]
impl crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitedPropertyScope {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IProperty0(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitContext,
        >,
        property: quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitContext,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Unity::Properties::IProperty,
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
            cordl_method_info.invoke_unchecked(self, (context, property))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_Type1(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitContext,
        >,
        index: i32,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitContext,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (context, index, _cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor+VisitedPropertyScope"
)]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitedPropertyScope {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+AutoCompletePathVisitor+VisitedPropertyScope"
)]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::UIElements::Internal::AutoCompletePathVisitor_VisitedPropertyScope {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
