#[cfg(feature = "cordl_class_BGLib+UnityExtension+BezierCurves+ObjectBasedBezierCurve")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectBasedBezierCurve {
    __cordl_parent: crate::BGLib::UnityExtension::BezierCurves::BaseBezierCurve_1<
        quest_hook::libil2cpp::Gc<
            crate::BGLib::UnityExtension::BezierCurves::ObjectBasedBezierCurveData,
        >,
    >,
}
#[cfg(feature = "cordl_class_BGLib+UnityExtension+BezierCurves+ObjectBasedBezierCurve")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::UnityExtension::BezierCurves::ObjectBasedBezierCurve {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.UnityExtension.BezierCurves";
    const CLASS_NAME: &'static str = "ObjectBasedBezierCurve";
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
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+ObjectBasedBezierCurve")]
impl std::ops::Deref
for crate::BGLib::UnityExtension::BezierCurves::ObjectBasedBezierCurve {
    type Target = crate::BGLib::UnityExtension::BezierCurves::BaseBezierCurve_1<
        quest_hook::libil2cpp::Gc<
            crate::BGLib::UnityExtension::BezierCurves::ObjectBasedBezierCurveData,
        >,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+ObjectBasedBezierCurve")]
impl std::ops::DerefMut
for crate::BGLib::UnityExtension::BezierCurves::ObjectBasedBezierCurve {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+ObjectBasedBezierCurve")]
impl crate::BGLib::UnityExtension::BezierCurves::ObjectBasedBezierCurve {
    pub fn GetBezierCurveData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BGLib::UnityExtension::BezierCurves::CurveData,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::BGLib::UnityExtension::BezierCurves::CurveData,
                        0usize,
                    >("GetBezierCurveData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetBezierCurveData", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::BGLib::UnityExtension::BezierCurves::CurveData = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetBezierCurveData(
        &mut self,
        newValue: crate::BGLib::UnityExtension::BezierCurves::CurveData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::BGLib::UnityExtension::BezierCurves::CurveData),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetBezierCurveData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetBezierCurveData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (newValue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_isReady(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_isReady")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_isReady", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BGLib+UnityExtension+BezierCurves+ObjectBasedBezierCurve")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::BezierCurves::ObjectBasedBezierCurve {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
