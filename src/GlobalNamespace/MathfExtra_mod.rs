#[cfg(feature = "MathfExtra")]
#[repr(C)]
#[derive(Debug)]
pub struct MathfExtra {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "MathfExtra")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::MathfExtra {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MathfExtra";
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
#[cfg(feature = "MathfExtra")]
impl std::ops::Deref for crate::GlobalNamespace::MathfExtra {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MathfExtra")]
impl std::ops::DerefMut for crate::GlobalNamespace::MathfExtra {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MathfExtra")]
impl crate::GlobalNamespace::MathfExtra {
    pub fn Approximately(
        a: f32,
        b: f32,
        precision: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32, f32, f32), bool, 3usize>("Approximately")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Approximately", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (a, b, precision))
        };
        Ok(__cordl_ret.into())
    }
    pub fn MaxAbs(a: f32, b: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32, f32), f32, 2usize>("MaxAbs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MaxAbs", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn Repeat(t: i32, length: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32), i32, 2usize>("Repeat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Repeat", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (t, length)) };
        Ok(__cordl_ret.into())
    }
    pub fn Round_Vector4_1(
        value: crate::UnityEngine::Vector4,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector4, i32),
                crate::UnityEngine::Vector4,
                2usize,
            >("Round")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Round", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector4 = unsafe {
            method.invoke_unchecked((), (value, digits))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Round_f32_0(value: f32, decimals: i32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32, i32), f32, 2usize>("Round")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Round", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (value, decimals)) };
        Ok(__cordl_ret.into())
    }
    pub fn ShortestAngleDifference(
        from: f32,
        to: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32, f32), f32, 2usize>("ShortestAngleDifference")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ShortestAngleDifference", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (from, to)) };
        Ok(__cordl_ret.into())
    }
    pub fn _cordl_Mod_f32_f32_0(
        value: f32,
        _cordl_mod: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32, f32), f32, 2usize>("Mod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Mod", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (value, _cordl_mod))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _cordl_Mod_i32_i32_1(
        value: i32,
        _cordl_mod: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32), i32, 2usize>("Mod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Mod", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (value, _cordl_mod))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MathfExtra")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MathfExtra {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
