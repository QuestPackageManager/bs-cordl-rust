#[cfg(feature = "BGLib+UnityExtension+BezierCurves+GenericBezierCurve")]
#[repr(C)]
#[derive(Debug)]
pub struct GenericBezierCurve {
    __cordl_parent: crate::BGLib::UnityExtension::BezierCurves::BaseBezierCurve_1<
        *mut crate::BGLib::UnityExtension::BezierCurves::GenericBezierCurveData,
    >,
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+GenericBezierCurve")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::UnityExtension::BezierCurves::GenericBezierCurve =>
    "BGLib.UnityExtension.BezierCurves"."GenericBezierCurve"
);
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+GenericBezierCurve")]
impl std::ops::Deref for crate::BGLib::UnityExtension::BezierCurves::GenericBezierCurve {
    type Target = crate::BGLib::UnityExtension::BezierCurves::BaseBezierCurve_1<
        *mut crate::BGLib::UnityExtension::BezierCurves::GenericBezierCurveData,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+GenericBezierCurve")]
impl std::ops::DerefMut
for crate::BGLib::UnityExtension::BezierCurves::GenericBezierCurve {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+GenericBezierCurve")]
impl crate::BGLib::UnityExtension::BezierCurves::GenericBezierCurve {
    pub fn GetBezierCurveData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BGLib::UnityExtension::BezierCurves::CurveData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BGLib::UnityExtension::BezierCurves::CurveData = __cordl_object
            .invoke("GetBezierCurveData", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBezierCurveData", (newValue))?;
        Ok(__cordl_ret.into())
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
    pub fn get_isReady(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isReady", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+GenericBezierCurve")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::BezierCurves::GenericBezierCurve {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}