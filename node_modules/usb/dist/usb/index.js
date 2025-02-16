"use strict";
var __values = (this && this.__values) || function(o) {
    var s = typeof Symbol === "function" && Symbol.iterator, m = s && o[s], i = 0;
    if (m) return m.call(o);
    if (o && typeof o.length === "number") return {
        next: function () {
            if (o && i >= o.length) o = void 0;
            return { value: o && o[i++], done: !o };
        }
    };
    throw new TypeError(s ? "Object is not iterable." : "Symbol.iterator is not defined.");
};
var events_1 = require("events");
var device_1 = require("./device");
var usb = require("./bindings");
if (usb.INIT_ERROR) {
    /* eslint-disable no-console */
    console.warn('Failed to initialize libusb.');
}
Object.setPrototypeOf(usb, events_1.EventEmitter.prototype);
Object.getOwnPropertyNames(device_1.ExtendedDevice.prototype).forEach(function (name) {
    Object.defineProperty(usb.Device.prototype, name, Object.getOwnPropertyDescriptor(device_1.ExtendedDevice.prototype, name) || Object.create(null));
});
// Polling mechanism for discovering device changes until this is fixed:
// https://github.com/libusb/libusb/issues/86
var pollTimeout = 500;
var hotplugSupported = usb._getLibusbCapability(usb.LIBUSB_CAP_HAS_HOTPLUG) > 0;
var pollingHotplug = false;
var pollDevices = new Set();
var pollHotplug = function (start) {
    var e_1, _a, e_2, _b;
    if (start === void 0) { start = false; }
    if (start) {
        pollingHotplug = true;
    }
    else if (!pollingHotplug) {
        return;
    }
    // Collect current devices
    var devices = new Set(usb.getDeviceList());
    if (!start) {
        try {
            // Find attached devices
            for (var devices_1 = __values(devices), devices_1_1 = devices_1.next(); !devices_1_1.done; devices_1_1 = devices_1.next()) {
                var device = devices_1_1.value;
                if (!pollDevices.has(device))
                    usb.emit('attach', device);
            }
        }
        catch (e_1_1) { e_1 = { error: e_1_1 }; }
        finally {
            try {
                if (devices_1_1 && !devices_1_1.done && (_a = devices_1.return)) _a.call(devices_1);
            }
            finally { if (e_1) throw e_1.error; }
        }
        try {
            // Find detached devices
            for (var pollDevices_1 = __values(pollDevices), pollDevices_1_1 = pollDevices_1.next(); !pollDevices_1_1.done; pollDevices_1_1 = pollDevices_1.next()) {
                var device = pollDevices_1_1.value;
                if (!devices.has(device))
                    usb.emit('detach', device);
            }
        }
        catch (e_2_1) { e_2 = { error: e_2_1 }; }
        finally {
            try {
                if (pollDevices_1_1 && !pollDevices_1_1.done && (_b = pollDevices_1.return)) _b.call(pollDevices_1);
            }
            finally { if (e_2) throw e_2.error; }
        }
    }
    pollDevices = devices;
    setTimeout(function () {
        pollHotplug();
    }, pollTimeout);
};
usb.on('newListener', function (event) {
    if (event !== 'attach' && event !== 'detach') {
        return;
    }
    var listenerCount = usb.listenerCount('attach') + usb.listenerCount('detach');
    if (listenerCount === 0) {
        if (hotplugSupported) {
            usb._enableHotplugEvents();
        }
        else {
            pollHotplug(true);
        }
    }
});
usb.on('removeListener', function (event) {
    if (event !== 'attach' && event !== 'detach') {
        return;
    }
    var listenerCount = usb.listenerCount('attach') + usb.listenerCount('detach');
    if (listenerCount === 0) {
        if (hotplugSupported) {
            usb._disableHotplugEvents();
        }
        else {
            pollingHotplug = false;
        }
    }
});
module.exports = usb;
//# sourceMappingURL=index.js.map