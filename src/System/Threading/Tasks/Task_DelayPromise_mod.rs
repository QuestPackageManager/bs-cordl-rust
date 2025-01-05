#[cfg(feature = "System+Threading+Tasks+Task+DelayPromise")]
#[repr(C)]
#[derive(Debug)]
pub struct Task_DelayPromise {
    __cordl_parent: crate::System::Threading::Tasks::Task_1<
        crate::System::Threading::Tasks::VoidTaskResult,
    >,
    pub Token: crate::System::Threading::CancellationToken,
    pub Registration: crate::System::Threading::CancellationTokenRegistration,
    pub Timer: quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>,
}
#[cfg(feature = "System+Threading+Tasks+Task+DelayPromise")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Task_DelayPromise =>
    "System.Threading.Tasks"."Task/DelayPromise"
);
#[cfg(feature = "System+Threading+Tasks+Task+DelayPromise")]
impl std::ops::Deref for crate::GlobalNamespace::Task_DelayPromise {
    type Target = crate::System::Threading::Tasks::Task_1<
        crate::System::Threading::Tasks::VoidTaskResult,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+DelayPromise")]
impl std::ops::DerefMut for crate::GlobalNamespace::Task_DelayPromise {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+DelayPromise")]
impl crate::GlobalNamespace::Task_DelayPromise {
    pub fn Complete(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Complete", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (token))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (token))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+DelayPromise")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::Task_DelayPromise {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
