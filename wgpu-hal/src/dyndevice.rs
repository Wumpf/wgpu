use crate::{BufferMapping, Device, DeviceError, FenceValue, MemoryRange};

pub trait DynInstance {}

pub trait DynQueue {}

pub trait DynAdapter {}

pub trait DynSurface {}

pub trait DynCommandEncoder {}

pub trait DynBuffer {
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait DynTexture {}

pub trait DynTextureView {}

pub trait DynSurfaceTexture {}

pub trait DynBindGroup {}

pub trait DynBindGroupLayout {}

pub trait DynPipelineLayout {}

pub trait DynShaderModule {}

pub trait DynRenderPipeline {}

pub trait DynComputePipeline {}

pub trait DynCommandBuffer {}

pub trait DynSampler {}

pub trait DynQuerySet {}

pub trait DynFence {
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait DynAccelerationStructure {}

pub trait DynDevice {
    unsafe fn map_buffer(
        &self,
        buffer: &dyn DynBuffer,
        range: MemoryRange,
    ) -> Result<BufferMapping, DeviceError>;
    unsafe fn unmap_buffer(&self, buffer: &dyn DynBuffer) -> Result<(), DeviceError>;
    unsafe fn flush_mapped_ranges(&self, buffer: &dyn DynBuffer, ranges: &[MemoryRange]);
    unsafe fn invalidate_mapped_ranges(&self, buffer: &dyn DynBuffer, ranges: &[MemoryRange]);
    unsafe fn start_capture(&self) -> bool;
    unsafe fn stop_capture(&self);
    unsafe fn get_fence_value(&self, fence: &dyn DynFence) -> Result<FenceValue, DeviceError>;
    unsafe fn wait(
        &self,
        fence: &dyn DynFence,
        value: FenceValue,
        timeout_ms: u32,
    ) -> Result<bool, DeviceError>;
}

impl<T: Device> DynDevice for T {
    unsafe fn map_buffer(
        &self,
        buffer: &dyn DynBuffer,
        range: MemoryRange,
    ) -> Result<BufferMapping, DeviceError> {
        let buffer = buffer.as_any().downcast_ref().unwrap();
        unsafe { <T as Device>::map_buffer(self, buffer, range) }
    }

    unsafe fn unmap_buffer(&self, buffer: &dyn DynBuffer) -> Result<(), DeviceError> {
        let buffer = buffer.as_any().downcast_ref().unwrap();
        unsafe { <T as Device>::unmap_buffer(self, buffer) }
    }

    unsafe fn flush_mapped_ranges(&self, buffer: &dyn DynBuffer, ranges: &[MemoryRange]) {
        let buffer = buffer.as_any().downcast_ref().unwrap();
        unsafe { <T as Device>::flush_mapped_ranges(self, buffer, ranges.iter().cloned()) }
    }

    unsafe fn invalidate_mapped_ranges(&self, buffer: &dyn DynBuffer, ranges: &[MemoryRange]) {
        let buffer = buffer.as_any().downcast_ref().unwrap();
        unsafe { <T as Device>::invalidate_mapped_ranges(self, buffer, ranges.iter().cloned()) }
    }

    unsafe fn start_capture(&self) -> bool {
        unsafe { <T as Device>::start_capture(self) }
    }

    unsafe fn stop_capture(&self) {
        unsafe {
            <T as Device>::start_capture(self);
        }
    }

    unsafe fn get_fence_value(&self, fence: &dyn DynFence) -> Result<FenceValue, DeviceError> {
        let fence = fence.as_any().downcast_ref().unwrap();
        unsafe { <T as Device>::get_fence_value(self, fence) }
    }

    unsafe fn wait(
        &self,
        fence: &dyn DynFence,
        value: FenceValue,
        timeout_ms: u32,
    ) -> Result<bool, DeviceError> {
        let fence = fence.as_any().downcast_ref().unwrap();
        unsafe { <T as Device>::wait(self, fence, value, timeout_ms) }
    }
}
