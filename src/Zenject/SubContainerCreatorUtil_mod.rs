#[cfg(feature = "Zenject+SubContainerCreatorUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerCreatorUtil {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+SubContainerCreatorUtil")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::SubContainerCreatorUtil {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "SubContainerCreatorUtil";
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
#[cfg(feature = "Zenject+SubContainerCreatorUtil")]
impl std::ops::Deref for crate::Zenject::SubContainerCreatorUtil {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorUtil")]
impl std::ops::DerefMut for crate::Zenject::SubContainerCreatorUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorUtil")]
impl crate::Zenject::SubContainerCreatorUtil {
    pub fn ApplyBindSettings(
        subContainerBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::SubContainerCreatorBindInfo,
        >,
        subContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Zenject::SubContainerCreatorBindInfo,
                    >,
                    quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ApplyBindSettings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ApplyBindSettings", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (subContainerBindInfo, subContainer))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::SubContainerCreatorUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
