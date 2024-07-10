pub trait DynInstance: wgt::WasmNotSendSync + Sized + 'static {}

impl crate::Instance for dyn DynInstance {
    type A = super::Api;

    unsafe fn init(desc: &crate::InstanceDescriptor) -> Result<Self, crate::InstanceError> {
        todo!()
    }

    unsafe fn create_surface(
        &self,
        display_handle: raw_window_handle::RawDisplayHandle,
        window_handle: raw_window_handle::RawWindowHandle,
    ) -> Result<<Self::A as crate::Api>::Surface, crate::InstanceError> {
        todo!()
    }

    unsafe fn destroy_surface(&self, surface: <Self::A as crate::Api>::Surface) {
        todo!()
    }

    unsafe fn enumerate_adapters(
        &self,
        surface_hint: Option<&<Self::A as crate::Api>::Surface>,
    ) -> Vec<crate::ExposedAdapter<Self::A>> {
        todo!()
    }
}
