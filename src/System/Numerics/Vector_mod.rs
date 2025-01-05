#[cfg(feature = "System+Numerics+Vector")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Numerics+Vector")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Numerics::Vector => "System.Numerics"
    ."Vector"
);
#[cfg(feature = "System+Numerics+Vector")]
impl std::ops::Deref for crate::System::Numerics::Vector {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Numerics+Vector")]
impl std::ops::DerefMut for crate::System::Numerics::Vector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Numerics+Vector")]
impl crate::System::Numerics::Vector {
    pub fn AsVectorUInt64<T>(
        value: crate::System::Numerics::Vector_1<T>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::Vector_1<u64>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::Numerics::Vector_1<u64> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AsVectorUInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals<T>(
        left: crate::System::Numerics::Vector_1<T>,
        right: crate::System::Numerics::Vector_1<T>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::Vector_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::Numerics::Vector_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Equals", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsHardwareAccelerated() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsHardwareAccelerated", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Numerics+Vector")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Numerics::Vector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
