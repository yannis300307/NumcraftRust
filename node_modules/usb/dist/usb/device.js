"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ExtendedDevice = void 0;
var usb = require("./bindings");
var interface_1 = require("./interface");
var capability_1 = require("./capability");
var isBuffer = function (obj) { return !!obj && obj instanceof Uint8Array; };
var DEFAULT_TIMEOUT = 1000;
var ExtendedDevice = /** @class */ (function () {
    function ExtendedDevice() {
        this._timeout = DEFAULT_TIMEOUT;
    }
    Object.defineProperty(ExtendedDevice.prototype, "timeout", {
        /**
         * Timeout in milliseconds to use for control transfers.
         */
        get: function () {
            return this._timeout || DEFAULT_TIMEOUT;
        },
        set: function (value) {
            this._timeout = value;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(ExtendedDevice.prototype, "configDescriptor", {
        /**
         * Object with properties for the fields of the active configuration descriptor.
         */
        get: function () {
            try {
                return this.__getConfigDescriptor();
            }
            catch (e) {
                // Check descriptor exists
                if (e.errno === usb.LIBUSB_ERROR_NOT_FOUND) {
                    return undefined;
                }
                throw e;
            }
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(ExtendedDevice.prototype, "allConfigDescriptors", {
        /**
         * Contains all config descriptors of the device (same structure as .configDescriptor above)
         */
        get: function () {
            try {
                return this.__getAllConfigDescriptors();
            }
            catch (e) {
                // Check descriptors exist
                if (e.errno === usb.LIBUSB_ERROR_NOT_FOUND) {
                    return [];
                }
                throw e;
            }
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(ExtendedDevice.prototype, "parent", {
        /**
         * Contains the parent of the device, such as a hub. If there is no parent this property is set to `null`.
         */
        get: function () {
            return this.__getParent();
        },
        enumerable: false,
        configurable: true
    });
    /**
     * Open the device.
     * @param defaultConfig
     */
    ExtendedDevice.prototype.open = function (defaultConfig) {
        if (defaultConfig === void 0) { defaultConfig = true; }
        this.__open();
        if (defaultConfig === false) {
            return;
        }
        this.interfaces = [];
        var len = this.configDescriptor ? this.configDescriptor.interfaces.length : 0;
        for (var i = 0; i < len; i++) {
            this.interfaces[i] = new interface_1.Interface(this, i);
        }
    };
    /**
     * Close the device.
     *
     * The device must be open to use this method.
     */
    ExtendedDevice.prototype.close = function () {
        this.__close();
        this.interfaces = undefined;
    };
    /**
     * Set the device configuration to something other than the default (0). To use this, first call `.open(false)` (which tells it not to auto configure),
     * then before claiming an interface, call this method.
     *
     * The device must be open to use this method.
     * @param desired
     * @param callback
     */
    ExtendedDevice.prototype.setConfiguration = function (desired, callback) {
        var _this = this;
        this.__setConfiguration(desired, function (error) {
            if (!error) {
                _this.interfaces = [];
                var len = _this.configDescriptor ? _this.configDescriptor.interfaces.length : 0;
                for (var i = 0; i < len; i++) {
                    _this.interfaces[i] = new interface_1.Interface(_this, i);
                }
            }
            if (callback) {
                callback.call(_this, error);
            }
        });
    };
    /**
     * Perform a control transfer with `libusb_control_transfer`.
     *
     * Parameter `data_or_length` can be an integer length for an IN transfer, or a `Buffer` for an OUT transfer. The type must match the direction specified in the MSB of bmRequestType.
     *
     * The `data` parameter of the callback is actual transferred for OUT transfers, or will be passed a Buffer for IN transfers.
     *
     * The device must be open to use this method.
     * @param bmRequestType
     * @param bRequest
     * @param wValue
     * @param wIndex
     * @param data_or_length
     * @param callback
     */
    ExtendedDevice.prototype.controlTransfer = function (bmRequestType, bRequest, wValue, wIndex, data_or_length, callback) {
        var _this = this;
        var isIn = !!(bmRequestType & usb.LIBUSB_ENDPOINT_IN);
        var wLength = isIn ? data_or_length : data_or_length.length;
        if (isIn) {
            if (!(data_or_length >= 0)) {
                throw new TypeError('Expected size number for IN transfer (based on bmRequestType)');
            }
        }
        else {
            if (!isBuffer(data_or_length)) {
                throw new TypeError('Expected buffer for OUT transfer (based on bmRequestType)');
            }
        }
        // Buffer for the setup packet
        // http://libusbx.sourceforge.net/api-1.0/structlibusb__control__setup.html
        var buf = Buffer.alloc(wLength + usb.LIBUSB_CONTROL_SETUP_SIZE);
        buf.writeUInt8(bmRequestType, 0);
        buf.writeUInt8(bRequest, 1);
        buf.writeUInt16LE(wValue, 2);
        buf.writeUInt16LE(wIndex, 4);
        buf.writeUInt16LE(wLength, 6);
        if (!isIn) {
            buf.set(data_or_length, usb.LIBUSB_CONTROL_SETUP_SIZE);
        }
        var transfer = new usb.Transfer(this, 0, usb.LIBUSB_TRANSFER_TYPE_CONTROL, this.timeout, function (error, buf, actual) {
            if (callback) {
                if (isIn) {
                    callback.call(_this, error, buf.slice(usb.LIBUSB_CONTROL_SETUP_SIZE, usb.LIBUSB_CONTROL_SETUP_SIZE + actual));
                }
                else {
                    callback.call(_this, error, actual);
                }
            }
        });
        try {
            transfer.submit(buf);
        }
        catch (e) {
            if (callback) {
                process.nextTick(function () { return callback.call(_this, e, undefined); });
            }
        }
        return this;
    };
    /**
     * Return the interface with the specified interface number.
     *
     * The device must be open to use this method.
     * @param addr
     */
    ExtendedDevice.prototype.interface = function (addr) {
        if (!this.interfaces) {
            throw new Error('Device must be open before searching for interfaces');
        }
        addr = addr || 0;
        for (var i = 0; i < this.interfaces.length; i++) {
            if (this.interfaces[i].interfaceNumber === addr) {
                return this.interfaces[i];
            }
        }
        throw new Error("Interface not found for address: " + addr);
    };
    /**
     * Perform a control transfer to retrieve a string descriptor
     *
     * The device must be open to use this method.
     * @param desc_index
     * @param callback
     */
    ExtendedDevice.prototype.getStringDescriptor = function (desc_index, callback) {
        // Index 0 indicates null
        if (desc_index === 0) {
            callback();
            return;
        }
        var langid = 0x0409;
        var length = 255;
        this.controlTransfer(usb.LIBUSB_ENDPOINT_IN, usb.LIBUSB_REQUEST_GET_DESCRIPTOR, ((usb.LIBUSB_DT_STRING << 8) | desc_index), langid, length, function (error, buffer) {
            if (error) {
                return callback(error);
            }
            callback(undefined, isBuffer(buffer) ? buffer.toString('utf16le', 2) : undefined);
        });
    };
    /**
     * Perform a control transfer to retrieve an object with properties for the fields of the Binary Object Store descriptor.
     *
     * The device must be open to use this method.
     * @param callback
     */
    ExtendedDevice.prototype.getBosDescriptor = function (callback) {
        var _this = this;
        if (this._bosDescriptor) {
            // Cached descriptor
            return callback(undefined, this._bosDescriptor);
        }
        if (this.deviceDescriptor.bcdUSB < 0x201) {
            // BOS is only supported from USB 2.0.1
            return callback(undefined, undefined);
        }
        this.controlTransfer(usb.LIBUSB_ENDPOINT_IN, usb.LIBUSB_REQUEST_GET_DESCRIPTOR, (usb.LIBUSB_DT_BOS << 8), 0, usb.LIBUSB_DT_BOS_SIZE, function (error, buffer) {
            if (error) {
                // Check BOS descriptor exists
                if (error.errno === usb.LIBUSB_TRANSFER_STALL)
                    return callback(undefined, undefined);
                return callback(error, undefined);
            }
            if (!isBuffer(buffer)) {
                return callback(undefined, undefined);
            }
            var totalLength = buffer.readUInt16LE(2);
            _this.controlTransfer(usb.LIBUSB_ENDPOINT_IN, usb.LIBUSB_REQUEST_GET_DESCRIPTOR, (usb.LIBUSB_DT_BOS << 8), 0, totalLength, function (error, buffer) {
                if (error) {
                    // Check BOS descriptor exists
                    if (error.errno === usb.LIBUSB_TRANSFER_STALL)
                        return callback(undefined, undefined);
                    return callback(error, undefined);
                }
                if (!isBuffer(buffer)) {
                    return callback(undefined, undefined);
                }
                var descriptor = {
                    bLength: buffer.readUInt8(0),
                    bDescriptorType: buffer.readUInt8(1),
                    wTotalLength: buffer.readUInt16LE(2),
                    bNumDeviceCaps: buffer.readUInt8(4),
                    capabilities: []
                };
                var i = usb.LIBUSB_DT_BOS_SIZE;
                while (i < descriptor.wTotalLength) {
                    var capability = {
                        bLength: buffer.readUInt8(i + 0),
                        bDescriptorType: buffer.readUInt8(i + 1),
                        bDevCapabilityType: buffer.readUInt8(i + 2),
                        dev_capability_data: buffer.slice(i + 3, i + buffer.readUInt8(i + 0))
                    };
                    descriptor.capabilities.push(capability);
                    i += capability.bLength;
                }
                // Cache descriptor
                _this._bosDescriptor = descriptor;
                callback(undefined, _this._bosDescriptor);
            });
        });
    };
    /**
     * Retrieve a list of Capability objects for the Binary Object Store capabilities of the device.
     *
     * The device must be open to use this method.
     * @param callback
     */
    ExtendedDevice.prototype.getCapabilities = function (callback) {
        var _this = this;
        var capabilities = [];
        this.getBosDescriptor(function (error, descriptor) {
            if (error)
                return callback(error, undefined);
            var len = descriptor ? descriptor.capabilities.length : 0;
            for (var i = 0; i < len; i++) {
                capabilities.push(new capability_1.Capability(_this, i));
            }
            callback(undefined, capabilities);
        });
    };
    return ExtendedDevice;
}());
exports.ExtendedDevice = ExtendedDevice;
//# sourceMappingURL=device.js.map