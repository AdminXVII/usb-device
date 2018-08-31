use ::Result;
use bus::StringIndex;
use device::{ControlOutResult, ControlInResult};
use descriptor::DescriptorWriter;
use control;

/// A trait implemented by USB class implementations.
pub trait UsbClass {
    /// Called after a USB reset after the bus reset sequence is complete.
    fn reset(&self) -> Result<()> {
        Ok(())
    }

    /// Called when a GET_DESCRIPTOR request is received for a configuration descriptor. When
    /// called, the implementation should write its interface, endpoint and any extra class
    /// descriptors into `writer`. The configuration descriptor itself will be written by
    /// [UsbDevice](::device::UsbDevice) and shouldn't be written by classes.
    ///
    /// # Errors
    ///
    /// Generally errors returned by `DescriptorWriter`. Implementors should propagate any errors
    /// using `?`.
    fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter) -> Result<()> {
        let _ = writer;
        Ok (())
    }

    /// Called when a control request is received with direction HostToDevice.
    ///
    /// All requests are passed to classes in turn, which can choose to accept, ignore or report an
    /// error. Classes can even choose to override standard requests, but doing that is rarely
    /// necessary.
    ///
    /// To ignore the request (default), return [`ControlOutResult::Ignore`]. To accept the request,
    /// return [`ControlOutResult::Ok`]. To report an error and return a STALL handshake to the
    /// host, return [`ControlOutResult::Err`].
    ///
    /// When implementing your own class, you should ignore any requests that are not meant for your
    /// class so that potential other classes in the composite device can process them.
    ///
    /// # Arguments
    ///
    /// * `req` - The request from the SETUP packet.
    /// * `data` - Data received in the DATA stage of the control transfer. Empty if there was no
    ///   DATA stage.
    fn control_out(&self, req: &control::Request, data: &[u8]) -> ControlOutResult {
        let _ = (req, data);
        ControlOutResult::Ignore
    }

    /// Called when a control request is received with direction DeviceToHost.
    ///
    /// All requests are passed to classes in turn, which can choose to accept, ignore or report an
    /// error. Classes can even choose to override standard requests, but doing that is rarely
    /// necessary.
    ///
    /// To ignore the request (default), return [`ControlInResult::Ignore`]. To accept the request,
    /// write your response to the buffer passed in `data` and return [`ControlInResult::Ok`] with
    /// the number of bytes written. Note that the number of bytes should not exceed `req.length`
    /// bytes. To report an error and return a STALL handshake to the host, return
    /// [`ControlInResult::Err`].
    ///
    /// When implementing your own class, you should ignore any requests that are not meant for your
    /// class so that potential other classes in the composite device can process them.
    ///
    /// # Arguments
    ///
    /// * `req` - The request from the SETUP packet.
    /// * `data` - Data to send in the DATA stage of the control transfer.
    fn control_in(&self, req: &control::Request, data: &mut [u8]) -> ControlInResult {
        let _ = (req, data);
        ControlInResult::Ignore
    }

    /// Called when endpoint with address `addr` has received a SETUP packet. Implementing this
    /// shouldn't be necessary in most cases, but is provided for completeness' sake.
    ///
    /// Note: This method may be called for an endpoint address you didn't allocate, and in that
    /// case you should ignore the event.
    fn endpoint_setup(&self, addr: u8) {
        let _ = addr;
    }

    /// Called when endpoint with address `addr` has received data (OUT packet).
    ///
    /// Note: This method may be called for an endpoint address you didn't allocate, and in that
    /// case you should ignore the event.
    fn endpoint_out(&self, addr: u8) {
        let _ = addr;
    }

    /// Called when endpoint with address `addr` has completed transmitting data (IN packet).
    ///
    /// Note: This method may be called for an endpoint address you didn't allocate, and in that
    /// case you should ignore the event.
    fn endpoint_in_complete(&self, addr: u8) {
        let _ = addr;
    }

    /// Gets a class-specific string descriptor.
    ///
    /// Note: All string descriptor requests are passed to all classes in turn, so implementations
    /// should return [`None`] if an unknown index is requested.
    ///
    /// # Arguments
    ///
    /// * `index` - A string index allocated earlier with [`UsbAllocator`](::bus::UsbAllocator).
    /// * `lang_id` - The language ID for the string to retrieve.
    fn get_string<'a>(&'a self, index: StringIndex, lang_id: u16) -> Option<&'a str> {
        let _ = (index, lang_id);
        None
    }
}