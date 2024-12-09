#[cfg(feature = "Zenject+MemoryPoolSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct MemoryPoolSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub InitialSize: i32,
    pub MaxSize: i32,
    pub ExpandMethod: crate::Zenject::PoolExpandMethods,
    pub ShowExpandWarning: bool,
}
#[cfg(feature = "Zenject+MemoryPoolSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::MemoryPoolSettings => "Zenject"
    ."MemoryPoolSettings"
);
#[cfg(feature = "Zenject+MemoryPoolSettings")]
impl std::ops::Deref for crate::Zenject::MemoryPoolSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+MemoryPoolSettings")]
impl std::ops::DerefMut for crate::Zenject::MemoryPoolSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+MemoryPoolSettings")]
impl crate::Zenject::MemoryPoolSettings {
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_PoolExpandMethods__cordl_bool1(
        initialSize: i32,
        maxSize: i32,
        expandMethod: crate::Zenject::PoolExpandMethods,
        showExpandWarning: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (initialSize, maxSize, expandMethod, showExpandWarning),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_PoolExpandMethods__cordl_bool1(
        &mut self,
        initialSize: i32,
        maxSize: i32,
        expandMethod: crate::Zenject::PoolExpandMethods,
        showExpandWarning: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialSize, maxSize, expandMethod, showExpandWarning))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+MemoryPoolSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::MemoryPoolSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
