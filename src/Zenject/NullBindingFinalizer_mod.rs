#[cfg(feature = "Zenject+NullBindingFinalizer")]
#[repr(C)]
#[derive(Debug)]
pub struct NullBindingFinalizer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+NullBindingFinalizer")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::NullBindingFinalizer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "NullBindingFinalizer";
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
#[cfg(feature = "Zenject+NullBindingFinalizer")]
impl std::ops::Deref for crate::Zenject::NullBindingFinalizer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+NullBindingFinalizer")]
impl std::ops::DerefMut for crate::Zenject::NullBindingFinalizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+NullBindingFinalizer")]
impl crate::Zenject::NullBindingFinalizer {
    pub fn FinalizeBinding(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("FinalizeBinding")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FinalizeBinding", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (container))
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
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_BindingInheritanceMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Zenject::BindingInheritanceMethods> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::Zenject::BindingInheritanceMethods,
                0usize,
            >("get_BindingInheritanceMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_BindingInheritanceMethod", 0usize
                )
            });
        let __cordl_ret: crate::Zenject::BindingInheritanceMethods = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+NullBindingFinalizer")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::NullBindingFinalizer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+NullBindingFinalizer")]
impl AsRef<crate::Zenject::IBindingFinalizer> for crate::Zenject::NullBindingFinalizer {
    fn as_ref(&self) -> &crate::Zenject::IBindingFinalizer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+NullBindingFinalizer")]
impl AsMut<crate::Zenject::IBindingFinalizer> for crate::Zenject::NullBindingFinalizer {
    fn as_mut(&mut self) -> &mut crate::Zenject::IBindingFinalizer {
        unsafe { std::mem::transmute(self) }
    }
}
