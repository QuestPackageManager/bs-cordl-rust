#[cfg(feature = "InternetConnectionChecker")]
#[repr(C)]
#[derive(Debug)]
pub struct InternetConnectionChecker {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "InternetConnectionChecker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::InternetConnectionChecker => ""
    ."InternetConnectionChecker"
);
#[cfg(feature = "InternetConnectionChecker")]
impl std::ops::Deref for crate::GlobalNamespace::InternetConnectionChecker {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "InternetConnectionChecker")]
impl std::ops::DerefMut for crate::GlobalNamespace::InternetConnectionChecker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "InternetConnectionChecker")]
impl crate::GlobalNamespace::InternetConnectionChecker {
    #[cfg(feature = "InternetConnectionChecker+_IsConnectedToInternetAsync_d__0")]
    pub type _IsConnectedToInternetAsync_d__0 = crate::GlobalNamespace::InternetConnectionChecker__IsConnectedToInternetAsync_d__0;
    pub fn IsConnectedToInternetAsync(
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsConnectedToInternetAsync", (cancellationToken))?;
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
#[cfg(feature = "InternetConnectionChecker")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::InternetConnectionChecker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
