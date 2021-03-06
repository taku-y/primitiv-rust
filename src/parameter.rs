use devices::AnyDevice;
use primitiv_sys as _primitiv;
use std::ffi::CString;
use std::io;
use std::path::Path;
use std::ptr::{self, NonNull};
use ApiResult;
use Device;
use Initializer;
use Shape;
use Tensor;
use Wrap;

/// Struct to manage a trainable tensor parameter.
#[derive(Debug)]
pub struct Parameter {
    inner: NonNull<_primitiv::primitivParameter_t>,
    owned: bool,
}

impl_wrap!(Parameter, primitivParameter_t);
impl_drop!(Parameter, primitivDeleteParameter);

impl Parameter {
    /// Creates an invalid parameter object.
    pub fn new() -> Self {
        unsafe {
            let mut parameter_ptr: *mut _primitiv::primitivParameter_t = ptr::null_mut();
            check_api_status!(_primitiv::primitivCreateParameter(&mut parameter_ptr));
            Parameter::from_raw(parameter_ptr, true)
        }
    }

    /// Creates a new Parameter object.
    pub fn from_values<S: Into<Shape>>(shape: S, value: &[f32]) -> Self {
        Self::from_values_on::<S, AnyDevice>(shape, value, None)
    }

    /// Creates a new Parameter object.
    pub fn from_values_on<S: Into<Shape>, D: Device>(
        shape: S,
        value: &[f32],
        device: Option<&mut D>,
    ) -> Self {
        unsafe {
            let mut parameter_ptr: *mut _primitiv::primitivParameter_t = ptr::null_mut();
            check_api_status!(_primitiv::primitivCreateParameterWithValues(
                shape.into().as_ptr(),
                value.as_ptr(),
                value.len(),
                device.map(|d| d.as_mut_ptr()).unwrap_or(ptr::null_mut()),
                &mut parameter_ptr,
            ));
            Parameter::from_raw(parameter_ptr, true)
        }
    }

    /// Creates a new Parameter object.
    pub fn from_initializer<S: Into<Shape>, I: Initializer>(shape: S, initializer: &I) -> Self {
        Self::from_initializer_on::<S, AnyDevice, I>(shape, initializer, None)
    }

    /// Creates a new Parameter object.
    pub fn from_initializer_on<S: Into<Shape>, D: Device, I: Initializer>(
        shape: S,
        initializer: &I,
        device: Option<&mut D>,
    ) -> Self {
        unsafe {
            let mut parameter_ptr: *mut _primitiv::primitivParameter_t = ptr::null_mut();
            check_api_status!(_primitiv::primitivCreateParameterWithInitializer(
                shape.into().as_ptr(),
                initializer.as_ptr(),
                device.map(|d| d.as_mut_ptr()).unwrap_or(ptr::null_mut()),
                &mut parameter_ptr,
            ));
            Parameter::from_raw(parameter_ptr, true)
        }
    }

    /// Initializes the Parameter object.
    pub fn init_by_values<S: Into<Shape>>(&mut self, shape: S, value: &[f32]) {
        self.init_by_values_on::<S, AnyDevice>(shape, value, None);
    }

    /// Initializes the Parameter object.
    pub fn init_by_values_on<S: Into<Shape>, D: Device>(
        &mut self,
        shape: S,
        value: &[f32],
        device: Option<&mut D>,
    ) {
        unsafe {
            check_api_status!(_primitiv::primitivInitializeParameterWithValues(
                self.as_mut_ptr(),
                shape.into().as_ptr(),
                value.as_ptr(),
                value.len(),
                device.map(|d| d.as_mut_ptr()).unwrap_or(ptr::null_mut()),
            ));
        }
    }

    /// Initializes the Parameter object.
    pub fn init_by_initializer<S: Into<Shape>, I: Initializer>(
        &mut self,
        shape: S,
        initializer: &I,
    ) {
        self.init_by_initializer_on::<S, AnyDevice, I>(shape, initializer, None);
    }

    /// Initializes the Parameter object.
    pub fn init_by_initializer_on<S: Into<Shape>, D: Device, I: Initializer>(
        &mut self,
        shape: S,
        initializer: &I,
        device: Option<&mut D>,
    ) {
        unsafe {
            check_api_status!(_primitiv::primitivInitializeParameterWithInitializer(
                self.as_mut_ptr(),
                shape.into().as_ptr(),
                initializer.as_ptr(),
                device.map(|d| d.as_mut_ptr()).unwrap_or(ptr::null_mut()),
            ));
        }
    }

    /// Loads parameters from specified file.
    pub fn load<P: AsRef<Path>>(&mut self, path: P, with_stats: bool) -> io::Result<()> {
        self.load_on::<P, AnyDevice>(path, with_stats, None)
    }

    /// Loads parameters from specified file.
    pub fn load_on<P: AsRef<Path>, D: Device>(
        &mut self,
        path: P,
        with_stats: bool,
        device: Option<&mut D>,
    ) -> io::Result<()> {
        unsafe {
            let path_c = CString::new(path.as_ref().to_str().unwrap()).unwrap();
            let path_ptr = path_c.as_ptr();
            Result::from_api_status(
                _primitiv::primitivLoadParameter(
                    self.as_mut_ptr(),
                    path_ptr,
                    with_stats as u32,
                    device.map(|d| d.as_mut_ptr()).unwrap_or(ptr::null_mut()),
                ),
                (),
            ).map_err(|status| io::Error::new(io::ErrorKind::Other, status.message()))
        }
    }

    /// Saves current parameters into specified file.
    pub fn save<P: AsRef<Path>>(&self, path: P, with_stats: bool) -> io::Result<()> {
        unsafe {
            let path_c = CString::new(path.as_ref().to_str().unwrap()).unwrap();
            let path_ptr = path_c.as_ptr();
            Result::from_api_status(
                _primitiv::primitivSaveParameter(self.as_ptr(), path_ptr, with_stats as u32),
                (),
            ).map_err(|status| io::Error::new(io::ErrorKind::Other, status.message()))
        }
    }

    /// Returns whether the parameter is valid or not.
    pub fn valid(&self) -> bool {
        unsafe {
            let mut retval: u32 = 0;
            check_api_status!(_primitiv::primitivIsValidParameter(
                self.as_ptr(),
                &mut retval as *mut _,
            ));
            retval == 1
        }
    }

    /// Set all gradients to 0.
    pub fn reset_gradient(&mut self) {
        unsafe {
            check_api_status!(_primitiv::primitivResetParameterGradients(
                self.as_mut_ptr(),
            ));
        }
    }

    /// Adds a new optional statistics tensor.
    pub fn add_stats<S: Into<Shape>>(&mut self, name: &str, shape: S) {
        unsafe {
            let name_c = CString::new(name).unwrap();
            let name_ptr = name_c.as_ptr();
            check_api_status!(_primitiv::primitivAddStatsToParameter(
                self.as_mut_ptr(),
                name_ptr,
                shape.into().as_ptr(),
            ));
        }
    }

    /// Checks whether the statistics with name `name` exists or not.
    pub fn has_stats(&self, name: &str) -> bool {
        unsafe {
            let mut retval: u32 = 0;
            let name_c = CString::new(name).unwrap();
            let name_ptr = name_c.as_ptr();
            check_api_status!(_primitiv::primitivHasParameterStats(
                self.as_ptr(),
                name_ptr,
                &mut retval as *mut _,
            ));
            retval == 1
        }
    }

    /// Returns the shape of the parameter.
    pub fn shape(&self) -> Shape {
        unsafe {
            let mut shape_ptr: *mut _primitiv::primitivShape_t = ptr::null_mut();
            check_api_status!(_primitiv::primitivGetParameterShape(
                self.as_ptr(),
                &mut shape_ptr,
            ));
            Shape::from_raw(shape_ptr, true)
        }
    }

    /// Returns the Device object to manage the internal memory.
    pub fn device(&self) -> AnyDevice {
        unsafe {
            let mut device_ptr: *mut _primitiv::primitivDevice_t = ptr::null_mut();
            check_api_status!(_primitiv::primitivGetDeviceFromParameter(
                self.as_ptr(),
                &mut device_ptr,
            ));
            AnyDevice::from_raw(device_ptr, false)
        }
    }

    /// Returns the values of the parameter.
    pub fn value(&self) -> Tensor {
        unsafe {
            let mut tensor_ptr: *const _primitiv::primitivTensor_t = ptr::null_mut();
            check_api_status!(_primitiv::primitivGetParameterValue(
                self.as_ptr(),
                &mut tensor_ptr,
            ));
            Tensor::from_raw(tensor_ptr as *mut _, false)
        }
    }

    /// Returns the current gradient of the parameter.
    pub fn gradient(&self) -> Tensor {
        unsafe {
            let mut tensor_ptr: *const _primitiv::primitivTensor_t = ptr::null_mut();
            check_api_status!(_primitiv::primitivGetParameterGradient(
                self.as_ptr(),
                &mut tensor_ptr,
            ));
            Tensor::from_raw(tensor_ptr as *mut _, false)
        }
    }

    /// Returns the current opotional statistics tensor specified by given name.
    pub fn stats(&self, name: &str) -> Tensor {
        unsafe {
            let mut tensor_ptr: *const _primitiv::primitivTensor_t = ptr::null_mut();
            let name_c = CString::new(name).unwrap();
            let name_ptr = name_c.as_ptr();
            check_api_status!(_primitiv::primitivGetParameterStats(
                self.as_ptr(),
                name_ptr,
                &mut tensor_ptr,
            ));
            Tensor::from_raw(tensor_ptr as *mut _, false)
        }
    }
}

impl Default for Parameter {
    fn default() -> Parameter {
        Parameter::new()
    }
}
