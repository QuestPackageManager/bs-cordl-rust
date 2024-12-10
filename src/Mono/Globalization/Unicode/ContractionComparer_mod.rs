#[cfg(feature = "Mono+Globalization+Unicode+ContractionComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct ContractionComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Globalization+Unicode+ContractionComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Globalization::Unicode::ContractionComparer => "Mono.Globalization.Unicode"
    ."ContractionComparer"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (c1, c2))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
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
        *mut crate::Mono::Globalization::Unicode::Contraction,
    >,
> for crate::Mono::Globalization::Unicode::ContractionComparer {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IComparer_1<
        *mut crate::Mono::Globalization::Unicode::Contraction,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+ContractionComparer")]
impl AsMut<
    crate::System::Collections::Generic::IComparer_1<
        *mut crate::Mono::Globalization::Unicode::Contraction,
    >,
> for crate::Mono::Globalization::Unicode::ContractionComparer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IComparer_1<
        *mut crate::Mono::Globalization::Unicode::Contraction,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
