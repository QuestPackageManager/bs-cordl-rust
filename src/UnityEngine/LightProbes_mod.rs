#[cfg(feature = "UnityEngine+LightProbes")]
#[repr(C)]
#[derive(Debug)]
pub struct LightProbes {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
}
#[cfg(feature = "UnityEngine+LightProbes")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LightProbes => "UnityEngine"
    ."LightProbes"
);
#[cfg(feature = "UnityEngine+LightProbes")]
impl std::ops::Deref for crate::UnityEngine::LightProbes {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+LightProbes")]
impl std::ops::DerefMut for crate::UnityEngine::LightProbes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+LightProbes")]
impl crate::UnityEngine::LightProbes {
    pub fn AreLightProbesAllowed(
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AreLightProbesAllowed", (renderer))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateInterpolatedLightAndOcclusionProbes_Gc_Gc_Gc0(
        positions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        lightProbes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::SphericalHarmonicsL2,
            >,
        >,
        occlusionProbes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CalculateInterpolatedLightAndOcclusionProbes",
                (positions, lightProbes, occlusionProbes),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateInterpolatedLightAndOcclusionProbes_Gc_Gc_Gc1(
        positions: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>,
        lightProbes: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::SphericalHarmonicsL2,
        >,
        occlusionProbes: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CalculateInterpolatedLightAndOcclusionProbes",
                (positions, lightProbes, occlusionProbes),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateInterpolatedLightAndOcclusionProbes_Internal(
        positions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        positionsCount: i32,
        lightProbes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::SphericalHarmonicsL2,
            >,
        >,
        occlusionProbes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CalculateInterpolatedLightAndOcclusionProbes_Internal",
                (positions, positionsCount, lightProbes, occlusionProbes),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCount() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInterpolatedLightProbe(
        &mut self,
        position: crate::UnityEngine::Vector3,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        coefficients: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetInterpolatedLightProbe", (position, renderer, coefficients))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInterpolatedProbe(
        position: crate::UnityEngine::Vector3,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        probe: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::SphericalHarmonicsL2,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInterpolatedProbe", (position, renderer, probe))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInterpolatedProbe_Injected(
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        probe: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::SphericalHarmonicsL2,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInterpolatedProbe_Injected", (position, renderer, probe))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_CallLightProbesUpdatedFunction() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_CallLightProbesUpdatedFunction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_CallNeedsRetetrahedralizationFunction() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_CallNeedsRetetrahedralizationFunction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_CallTetrahedralizationCompletedFunction() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_CallTetrahedralizationCompletedFunction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Tetrahedralize() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Tetrahedralize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TetrahedralizeAsync() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TetrahedralizeAsync", ())?;
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
    pub fn add_lightProbesUpdated(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_lightProbesUpdated", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_needsRetetrahedralization(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_needsRetetrahedralization", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_tetrahedralizationCompleted(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_tetrahedralizationCompleted", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bakedProbes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::SphericalHarmonicsL2,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::SphericalHarmonicsL2,
            >,
        > = __cordl_object.invoke("get_bakedProbes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cellCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_cellCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_coefficients(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = __cordl_object.invoke("get_coefficients", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_positions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        > = __cordl_object.invoke("get_positions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_lightProbesUpdated(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_lightProbesUpdated", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_needsRetetrahedralization(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_needsRetetrahedralization", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_tetrahedralizationCompleted(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_tetrahedralizationCompleted", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bakedProbes(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::SphericalHarmonicsL2,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bakedProbes", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_coefficients(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_coefficients", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+LightProbes")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::LightProbes {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
