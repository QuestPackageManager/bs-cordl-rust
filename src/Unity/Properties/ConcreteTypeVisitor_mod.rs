#[cfg(feature = "cordl_class_Unity+Properties+ConcreteTypeVisitor")]
#[repr(C)]
#[derive(Debug)]
pub struct ConcreteTypeVisitor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Properties+ConcreteTypeVisitor")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Properties::ConcreteTypeVisitor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "ConcreteTypeVisitor";
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
#[cfg(feature = "Unity+Properties+ConcreteTypeVisitor")]
impl std::ops::Deref for crate::Unity::Properties::ConcreteTypeVisitor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+ConcreteTypeVisitor")]
impl std::ops::DerefMut for crate::Unity::Properties::ConcreteTypeVisitor {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+ConcreteTypeVisitor")]
impl crate::Unity::Properties::ConcreteTypeVisitor {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Unity_Properties_IPropertyBagVisitor_Visit<TContainer>(
        &mut self,
        properties: quest_hook::libil2cpp::Gc<crate::Unity::Properties::IPropertyBag_1<TContainer>>,
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::Unity::Properties::IPropertyBag_1<TContainer>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<TContainer>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "Unity.Properties.IPropertyBagVisitor.Visit",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Unity.Properties.IPropertyBagVisitor.Visit",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (properties, container))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitContainer<TContainer>(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<TContainer>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("VisitContainer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitContainer", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (container))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+ConcreteTypeVisitor")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Properties::ConcreteTypeVisitor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+ConcreteTypeVisitor")]
impl AsRef<crate::Unity::Properties::IPropertyBagVisitor>
    for crate::Unity::Properties::ConcreteTypeVisitor
{
    fn as_ref(&self) -> &crate::Unity::Properties::IPropertyBagVisitor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+ConcreteTypeVisitor")]
impl AsMut<crate::Unity::Properties::IPropertyBagVisitor>
    for crate::Unity::Properties::ConcreteTypeVisitor
{
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IPropertyBagVisitor {
        unsafe { std::mem::transmute(self) }
    }
}
