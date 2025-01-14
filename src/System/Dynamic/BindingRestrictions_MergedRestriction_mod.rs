#[cfg(feature = "System+Dynamic+BindingRestrictions+MergedRestriction")]
#[repr(C)]
#[derive(Debug)]
pub struct BindingRestrictions_MergedRestriction {
    __cordl_parent: crate::System::Dynamic::BindingRestrictions,
    pub Left: quest_hook::libil2cpp::Gc<crate::System::Dynamic::BindingRestrictions>,
    pub Right: quest_hook::libil2cpp::Gc<crate::System::Dynamic::BindingRestrictions>,
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+MergedRestriction")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BindingRestrictions_MergedRestriction {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Dynamic";
    const CLASS_NAME: &'static str = "BindingRestrictions/MergedRestriction";
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
#[cfg(feature = "System+Dynamic+BindingRestrictions+MergedRestriction")]
impl std::ops::Deref for crate::GlobalNamespace::BindingRestrictions_MergedRestriction {
    type Target = crate::System::Dynamic::BindingRestrictions;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+MergedRestriction")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BindingRestrictions_MergedRestriction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+MergedRestriction")]
impl crate::GlobalNamespace::BindingRestrictions_MergedRestriction {
    pub fn GetExpression(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
                0usize,
            >("GetExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetExpression", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        left: quest_hook::libil2cpp::Gc<crate::System::Dynamic::BindingRestrictions>,
        right: quest_hook::libil2cpp::Gc<crate::System::Dynamic::BindingRestrictions>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (left, right))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        left: quest_hook::libil2cpp::Gc<crate::System::Dynamic::BindingRestrictions>,
        right: quest_hook::libil2cpp::Gc<crate::System::Dynamic::BindingRestrictions>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Dynamic::BindingRestrictions,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Dynamic::BindingRestrictions,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (left, right))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+MergedRestriction")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BindingRestrictions_MergedRestriction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
