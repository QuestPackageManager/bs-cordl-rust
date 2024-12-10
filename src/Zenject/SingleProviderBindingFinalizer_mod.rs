#[cfg(feature = "Zenject+SingleProviderBindingFinalizer")]
#[repr(C)]
#[derive(Debug)]
pub struct SingleProviderBindingFinalizer {
    __cordl_parent: crate::Zenject::ProviderBindingFinalizer,
    pub _providerFactory: *mut crate::System::Func_3<
        *mut crate::Zenject::DiContainer,
        *mut crate::System::Type,
        *mut crate::Zenject::IProvider,
    >,
}
#[cfg(feature = "Zenject+SingleProviderBindingFinalizer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SingleProviderBindingFinalizer =>
    "Zenject"."SingleProviderBindingFinalizer"
);
#[cfg(feature = "Zenject+SingleProviderBindingFinalizer")]
impl std::ops::Deref for crate::Zenject::SingleProviderBindingFinalizer {
    type Target = crate::Zenject::ProviderBindingFinalizer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SingleProviderBindingFinalizer")]
impl std::ops::DerefMut for crate::Zenject::SingleProviderBindingFinalizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SingleProviderBindingFinalizer")]
impl crate::Zenject::SingleProviderBindingFinalizer {
    pub fn New(
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        providerFactory: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                *mut crate::Zenject::DiContainer,
                *mut crate::System::Type,
                *mut crate::Zenject::IProvider,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo, providerFactory))?;
        Ok(__cordl_object.into())
    }
    pub fn OnFinalizeBinding(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFinalizeBinding", (container))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        providerFactory: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                *mut crate::Zenject::DiContainer,
                *mut crate::System::Type,
                *mut crate::Zenject::IProvider,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindInfo, providerFactory))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+SingleProviderBindingFinalizer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::SingleProviderBindingFinalizer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
