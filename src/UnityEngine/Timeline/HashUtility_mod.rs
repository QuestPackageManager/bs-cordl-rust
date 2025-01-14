#[cfg(feature = "UnityEngine+Timeline+HashUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct HashUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+HashUtility")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Timeline::HashUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "HashUtility";
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
#[cfg(feature = "UnityEngine+Timeline+HashUtility")]
impl std::ops::Deref for crate::UnityEngine::Timeline::HashUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+HashUtility")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::HashUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+HashUtility")]
impl crate::UnityEngine::Timeline::HashUtility {
    pub fn CombineHash_Il2CppArray6(
        hashes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>),
                i32,
                1usize,
            >("CombineHash")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CombineHash", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (hashes)) };
        Ok(__cordl_ret.into())
    }
    pub fn CombineHash_i32_i32_0(
        h1: i32,
        h2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32), i32, 2usize>("CombineHash")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CombineHash", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (h1, h2)) };
        Ok(__cordl_ret.into())
    }
    pub fn CombineHash_i32_i32_i32_1(
        h1: i32,
        h2: i32,
        h3: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32, i32), i32, 3usize>("CombineHash")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CombineHash", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (h1, h2, h3)) };
        Ok(__cordl_ret.into())
    }
    pub fn CombineHash_i32_i32_i32_i32_2(
        h1: i32,
        h2: i32,
        h3: i32,
        h4: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32, i32, i32), i32, 4usize>("CombineHash")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CombineHash", 4usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (h1, h2, h3, h4)) };
        Ok(__cordl_ret.into())
    }
    pub fn CombineHash_i32_i32_i32_i32_i32_3(
        h1: i32,
        h2: i32,
        h3: i32,
        h4: i32,
        h5: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32, i32, i32, i32), i32, 5usize>("CombineHash")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CombineHash", 5usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (h1, h2, h3, h4, h5))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CombineHash_i32_i32_i32_i32_i32_i32_4(
        h1: i32,
        h2: i32,
        h3: i32,
        h4: i32,
        h5: i32,
        h6: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32, i32, i32, i32, i32),
                i32,
                6usize,
            >("CombineHash")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CombineHash", 6usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (h1, h2, h3, h4, h5, h6))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CombineHash_i32_i32_i32_i32_i32_i32_i32_5(
        h1: i32,
        h2: i32,
        h3: i32,
        h4: i32,
        h5: i32,
        h6: i32,
        h7: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32, i32, i32, i32, i32, i32),
                i32,
                7usize,
            >("CombineHash")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CombineHash", 7usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (h1, h2, h3, h4, h5, h6, h7))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+HashUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::HashUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
