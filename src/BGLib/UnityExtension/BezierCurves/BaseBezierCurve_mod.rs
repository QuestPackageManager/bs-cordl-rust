#[cfg(feature = "BGLib+UnityExtension+BezierCurves+BaseBezierCurve")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseBezierCurve {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+BaseBezierCurve")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::UnityExtension::BezierCurves::BaseBezierCurve =>
    "BGLib.UnityExtension.BezierCurves"."BaseBezierCurve"
);
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+BaseBezierCurve")]
impl std::ops::Deref for crate::BGLib::UnityExtension::BezierCurves::BaseBezierCurve {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+BaseBezierCurve")]
impl std::ops::DerefMut for crate::BGLib::UnityExtension::BezierCurves::BaseBezierCurve {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+BaseBezierCurve")]
impl crate::BGLib::UnityExtension::BezierCurves::BaseBezierCurve {
    pub fn Evaluate(
        &mut self,
        current: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("Evaluate", (current))?;
        Ok(__cordl_ret.into())
    }
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
        data: crate::BGLib::UnityExtension::BezierCurves::CurveData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBezierCurveData", (data))?;
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
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+BaseBezierCurve")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::BezierCurves::BaseBezierCurve {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
