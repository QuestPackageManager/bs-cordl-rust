#[cfg(feature = "System+Numerics+Vector")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Numerics+Vector")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Numerics::Vector {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Numerics";
    const CLASS_NAME: &'static str = "Vector";
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
#[cfg(feature = "System+Numerics+Vector")]
impl std::ops::Deref for crate::System::Numerics::Vector {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Numerics::Vector as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Numerics::Vector_1<T>),
                crate::System::Numerics::Vector_1<u64>,
                1usize,
            >("AsVectorUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Numerics::Vector as quest_hook::libil2cpp::Type >
                    ::class(), "AsVectorUInt64", 1usize
                )
            });
        let __cordl_ret: crate::System::Numerics::Vector_1<u64> = unsafe {
            method.invoke_unchecked((), (value))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Numerics::Vector as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::Numerics::Vector_1<T>,
                    crate::System::Numerics::Vector_1<T>,
                ),
                crate::System::Numerics::Vector_1<T>,
                2usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Numerics::Vector as quest_hook::libil2cpp::Type >
                    ::class(), "Equals", 2usize
                )
            });
        let __cordl_ret: crate::System::Numerics::Vector_1<T> = unsafe {
            method.invoke_unchecked((), (left, right))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsHardwareAccelerated() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Numerics::Vector as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_IsHardwareAccelerated")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Numerics::Vector as quest_hook::libil2cpp::Type >
                    ::class(), "get_IsHardwareAccelerated", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ())? };
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
