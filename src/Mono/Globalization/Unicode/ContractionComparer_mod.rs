#[cfg(feature = "Mono+Globalization+Unicode+ContractionComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct ContractionComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Globalization+Unicode+ContractionComparer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Globalization::Unicode::ContractionComparer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Globalization.Unicode";
    const CLASS_NAME: &'static str = "ContractionComparer";
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
#[cfg(feature = "Mono+Globalization+Unicode+ContractionComparer")]
impl std::ops::Deref for crate::Mono::Globalization::Unicode::ContractionComparer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+ContractionComparer")]
impl std::ops::DerefMut for crate::Mono::Globalization::Unicode::ContractionComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+ContractionComparer")]
impl crate::Mono::Globalization::Unicode::ContractionComparer {
    pub fn Compare(
        &mut self,
        c1: quest_hook::libil2cpp::Gc<crate::Mono::Globalization::Unicode::Contraction>,
        c2: quest_hook::libil2cpp::Gc<crate::Mono::Globalization::Unicode::Contraction>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Mono::Globalization::Unicode::Contraction,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Mono::Globalization::Unicode::Contraction,
                    >,
                ),
                i32,
                2usize,
            >("Compare")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Compare", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (c1, c2)) };
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
}
#[cfg(feature = "Mono+Globalization+Unicode+ContractionComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Globalization::Unicode::ContractionComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+ContractionComparer")]
impl AsRef<
    crate::System::Collections::Generic::IComparer_1<
        quest_hook::libil2cpp::Gc<crate::Mono::Globalization::Unicode::Contraction>,
    >,
> for crate::Mono::Globalization::Unicode::ContractionComparer {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IComparer_1<
        quest_hook::libil2cpp::Gc<crate::Mono::Globalization::Unicode::Contraction>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+ContractionComparer")]
impl AsMut<
    crate::System::Collections::Generic::IComparer_1<
        quest_hook::libil2cpp::Gc<crate::Mono::Globalization::Unicode::Contraction>,
    >,
> for crate::Mono::Globalization::Unicode::ContractionComparer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IComparer_1<
        quest_hook::libil2cpp::Gc<crate::Mono::Globalization::Unicode::Contraction>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
