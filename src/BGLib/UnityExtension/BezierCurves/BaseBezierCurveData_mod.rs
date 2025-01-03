#[cfg(feature = "BGLib+UnityExtension+BezierCurves+BaseBezierCurveData")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseBezierCurveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _startControlPointDelta: crate::UnityEngine::Vector3,
    pub _endControlPointDelta: crate::UnityEngine::Vector3,
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+BaseBezierCurveData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::UnityExtension::BezierCurves::BaseBezierCurveData =>
    "BGLib.UnityExtension.BezierCurves"."BaseBezierCurveData"
);
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+BaseBezierCurveData")]
impl std::ops::Deref
for crate::BGLib::UnityExtension::BezierCurves::BaseBezierCurveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+BaseBezierCurveData")]
impl std::ops::DerefMut
for crate::BGLib::UnityExtension::BezierCurves::BaseBezierCurveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+BaseBezierCurveData")]
impl crate::BGLib::UnityExtension::BezierCurves::BaseBezierCurveData {
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
    pub fn get_endControlPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_endControlPoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_endPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_endPoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startControlPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_startControlPoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_startPoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_endControlPoint(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_endControlPoint", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_startControlPoint(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_startControlPoint", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+BaseBezierCurveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::BezierCurves::BaseBezierCurveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
