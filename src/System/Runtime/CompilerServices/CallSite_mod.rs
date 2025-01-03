#[cfg(feature = "System+Runtime+CompilerServices+CallSite")]
#[repr(C)]
#[derive(Debug)]
pub struct CallSite {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _binder: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::CallSiteBinder,
    >,
    pub _match: bool,
}
#[cfg(feature = "System+Runtime+CompilerServices+CallSite")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::CompilerServices::CallSite =>
    "System.Runtime.CompilerServices"."CallSite"
);
#[cfg(feature = "System+Runtime+CompilerServices+CallSite")]
impl std::ops::Deref for crate::System::Runtime::CompilerServices::CallSite {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+CallSite")]
impl std::ops::DerefMut for crate::System::Runtime::CompilerServices::CallSite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+CallSite")]
impl crate::System::Runtime::CompilerServices::CallSite {
    pub fn New(
        binder: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSiteBinder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (binder))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSiteBinder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (binder))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Binder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSiteBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSiteBinder,
        > = __cordl_object.invoke("get_Binder", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+CallSite")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::CallSite {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
